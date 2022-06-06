use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::{enum_str, println};
use crate::kernel::arch::x86::interrupts::page_fault::PageFaultErrorCode;

#[macro_export]
macro_rules! save_scratch_registers {
    ()  => { "
        push rax
        push rcx
        push rdx
        push rsi
        push rdi
        push r8
        push r9
        push r10
        push r11
    " }
}

#[macro_export]
macro_rules! restore_scratch_registers {
    ()  => { "
        pop r11
        pop r10
        pop r9
        pop r8
        pop rsi
        pop rdi
        pop rdx
        pop rcx
        pop rax
    " }
}

#[macro_export]
macro_rules! interrupt_error {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
           unsafe {
                asm! {
                    //save scratch (caller-saved) registers
                    save_scratch_registers!(),

                    "mov rdi, rsp", //rdi is used as the first argument passed to a function so we move the contents of rsp to rdi
                    "add rdi, 9*8", //adjust the stack frame pointer
                    "call {}", //call the function specified by $name

                     //restore scratch (caller-saved) registers
                    restore_scratch_registers!(),

                    "iretq", //return program control to the program/procedure that was interrupted
                    sym $name,
                    options(noreturn)
                }
           }
        }
        wrapper
    }}
}

#[macro_export]
macro_rules! interrupt_error_with_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm! {
                    save_scratch_registers!(),

                    "mov rsi, [rsp + 9*8]", //rsi is used as the second argument passed to a function, so we move the error code into rsi.
                    "mov rdi, rsp", //rdi is used as the first argument passed to a function, so we move the contents of rsp to rdi
                    "add rdi, 10*8", //adjust the stack frame pointer
                    "sub rsp, 8", //align stack pointer
                    "call {}",  //call the function specified by $name
                    "add rsp, 8", //restore stack pointer alignment

                    restore_scratch_registers!(),

                    "add rsp,8", //pop error code
                    "iretq", //return program control to the program/procedure that was interrupted
                    sym $name,
                    options(noreturn)
                }
            }
        }
        wrapper
    }}
}

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
}

pub unsafe extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", &*stack_frame );
}

pub unsafe extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    let stack_frame = &*stack_frame;
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
}

pub unsafe extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) {
    let cr2: usize;
    asm! {
        "mov {}, cr2",
        out(reg) cr2
    };

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x} with error code {:?}\n{:#?}",
             cr2,
             Into::<PageFaultErrorCode>::into(error_code).name(),
             &*stack_frame);
}