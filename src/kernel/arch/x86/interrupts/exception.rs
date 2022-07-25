use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::{enum_str, println};
use crate::kernel::arch::x86::interrupts::page_fault;
use crate::kernel::arch::x86::interrupts::page_fault::{PageFault, PageFaultBuilder, PageFaultErrorCode};
use crate::kernel::arch::x86::registers::StackFrame;

#[macro_export]
macro_rules! save_scratch_registers {
    ()  => { "
        push rax
        push rcx
        push rdx
        push rdi
        push rsi
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

                    "mov rsi, [rsp + 15 * 8]", //rsi is used as the second argument passed to a function, so we move the error code into rsi.
                    "mov rdi, rsp", //rdi is used as the first argument passed to a function so we move rsp to rdi
                    "call {}",  //call the function specified by $name with pointer to stack (rdi) and error code (rsi)

                    restore_preserved_registers!(), //restore preserved (callee-saved/non volatile) registers
                    restore_scratch_registers!(), //restore scratch (caller-saved/volatile) registers

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

pub extern "C" fn divide_by_zero(stack_frame: &StackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n");
    println!("\nRegister dump:");
    stack_frame.dump();
}

pub extern "C" fn debug(stack_frame: &StackFrame) {

}

pub extern "C" fn non_maskable_interrupt(stack_frame: &StackFrame) {
    println!("Non-maskable Interrupt!\n");
    println!("Register dump: ");
    stack_frame.dump();
}


pub extern "C" fn breakpoint(stack_frame: &StackFrame) {
    let rip = stack_frame.iret.rip;
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n", rip);
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn overflow(stack_frame: &StackFrame) {
    println!("overflow exception!\n");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn bound_range_exceeded(stack_frame: &StackFrame) {
    println!("Bound range exceeded!\n");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn invalid_opcode(stack_frame: &StackFrame) {
    println!("Invalid opcode exception!\n");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn device_not_available(stack_frame: &StackFrame) {
    println!("Device not available exception!\n");
    println!("Register dump: ");
    stack_frame.dump();
}

//double fault always generate an error code with a value of zero
pub extern "C" fn double_fault(stack_frame: &StackFrame, error_code: usize) {
    println!("Double fault occurred! with error code: {}\n", error_code);
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn invalid_tss(stack_frame: &StackFrame) {
    println!("Invalid TSS fault\n");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn segment_not_present(stack_frame: &StackFrame, error_code: usize) {
    println!("Segment not present exception with error code: {}\n", error_code);
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn stack_segment_fault(stack_frame: &StackFrame, error_code: usize) {
    println!("stack segment fault with error code: {}\n", error_code);
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn general_protection_fault(stack_frame: &StackFrame, error_code: usize) {
    println!("general protection fault with error code: {}\n", error_code);
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn page_fault(stack_frame: &StackFrame, error_code: usize) {
    let page_fault = PageFaultBuilder::build(error_code);

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x} with error {:?}\nAccess mode: {}\
             \nReserved mode: {}\n",
             page_fault.addr,
             page_fault.error_code_description.name(),
             page_fault.access_mode.name(),
             page_fault.reserved);

    if page_fault.caused_by_instruction_fetch {
        println!("Caused by instruction fetch");
    }

    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn x87_floating_point_exception(stack_frame: &StackFrame) {
    println!("x87 Floating-Point Exception!");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn alignment_check(stack_frame: &StackFrame) {
    println!("Alignment Check Exception!");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn simd_floating_point_exception(stack_frame: &StackFrame) {
    println!("SIMD Floating-Point Exception!");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn virtualization_exception(stack_frame: &StackFrame) {
    println!("Virtualization Exception!");
    println!("Register dump: ");
    stack_frame.dump();
}

pub extern "C" fn control_protection_exception(stack_frame: &StackFrame) {

}

pub extern "C" fn hypervisor_injection_exception(stack_frame: &StackFrame) {

}

pub extern "C" fn vmm_communication_exception(stack_frame: &StackFrame) {

}

pub extern "C" fn security_exception(stack_frame: &StackFrame) {
    println!("Security Exception!");
    println!("Register dump: ");
    stack_frame.dump();
}

