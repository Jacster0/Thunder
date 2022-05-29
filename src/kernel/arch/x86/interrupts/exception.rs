use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::println;

#[macro_export]
macro_rules! exception_handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
           unsafe {
                asm! {
                    "mov rdi, rsp",
                    "sub rsp, 8", //align stack pointer
                    "call {}", //call the function specified by $name
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

pub unsafe extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", &*stack_frame );
    loop {}
}