use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::println;

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
}

pub unsafe extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}",
             &*stack_frame );
    loop {}
}

#[naked]
pub extern "C" fn divide_by_zero_wrapper() -> ! {
    unsafe {
        asm! {
            "mov rdi, rsp",
            "sub rsp, 8",
            "call {}",
            sym divide_by_zero_handler,
            options(noreturn)
        }
    }
}