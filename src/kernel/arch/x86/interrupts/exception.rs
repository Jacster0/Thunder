use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::{enum_str, println};
use crate::kernel::arch::x86::interrupts::page_fault::PageFaultErrorCode;
use crate::kernel::arch::x86::registers::StackFrame;

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
macro_rules! save_preserved_registers {
    ()  => { "
        push rbx
        push rbp
        push r12
        push r13
        push r14
        push r15
    " }
}

#[macro_export]
macro_rules! restore_preserved_registers {
    ()  => { "
        pop r15
        pop r14
        pop r13
        pop r12
        pop rbp
        pop rbx
    " }
}


#[macro_export]
macro_rules! interrupt_error {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
           unsafe {
                asm! {
                    save_scratch_registers!(), //save scratch (caller-saved/volatile) registers
                    save_preserved_registers!(), //save preserved (callee-saved/non volatile) registers

                    "mov rdi, rsp", //rdi is used as the first argument passed to a function so we move rsp to rdi
                    "call {}", //call the function specified by $name with pointer to stack (rdi)

                    restore_preserved_registers!(), //restore preserved (callee-saved/non volatile) registers
                    restore_scratch_registers!(), //restore scratch (caller-saved/volatile) registers

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
                    save_scratch_registers!(), //save scratch (caller-saved/volatile) registers
                    save_preserved_registers!(), //save preserved (callee-saved/non volatile) registers

                    "mov rsi, [rsp + 15*8]", //rsi is used as the second argument passed to a function, so we move the error code into rsi.
                    "add rsp, 8", //align stack pointer
                    "mov rdi, rsp", //rdi is used as the first argument passed to a function so we move rsp to rdi
                    "call {}",  //call the function specified by $name with pointer to stack (rdi) and error code (rsi)
                    "add rsp,8", //pop error code

                    restore_preserved_registers!(), //restore preserved (callee-saved/non volatile) registers
                    restore_scratch_registers!(), //restore scratch (caller-saved/volatile) registers

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

#[derive(Default)]
#[repr(packed)]
pub struct PreservedRegisters {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub rbp: usize,
    pub rbx: usize,
}

pub unsafe extern "C" fn divide_by_zero_handler(stack_frame: &StackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n");
    println!("\nRegister dump:");
    stack_frame.dump();
}

pub unsafe extern "C" fn breakpoint_handler(stack_frame: &StackFrame) {
    let rip = stack_frame.iret.rip;
    println!("\nEXCEPTION: BREAKPOINT at {:#x}", rip);
    stack_frame.dump();
}

pub unsafe extern "C" fn page_fault_handler(stack_frame: &StackFrame, error_code: usize) {
    //when a page fault occurs, the address (Page Fault Linear Address aka PFLA)
    //that the program attempted to access is stored in the cr2 register
    let page_fault_linear_address: usize;
    asm! {
        "mov {}, cr2", //move the value stored in cr2 register to our page_fault_linear_address variable so that we can print it
        out(reg) page_fault_linear_address
    };

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x} with error code {:?}",
             page_fault_linear_address,
             Into::<PageFaultErrorCode>::into(error_code).name());

    stack_frame.dump();
}