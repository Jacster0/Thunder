use core::arch::asm;
use core::ptr::addr_of_mut;
use crate::{enum_str, println};

#[macro_export]
macro_rules! interrupt_error {
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

#[macro_export]
macro_rules! interrupt_error_with_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm! {
                    "pop rsi", //pop error code into rsi
                    "mov rdi, rsp",
                    "sub rsp, 8", //align stack pointer
                    "call {}",  //call the function specified by $name
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

enum_str! {
    enum PageFaultErrorCode {
        ProtectionViolation = 1 << 0,
        CausedByWrite = 1 << 1,
        UserMode = 1 << 2,
        MalformedTable = 1 << 3,
        InstructionFetch = 1 << 4,
        Unknown = 1 << 5,
    }
}

impl From<u64> for PageFaultErrorCode {
    fn from(code: u64) -> Self {
        match code {
            0x1 =>  PageFaultErrorCode::ProtectionViolation,
            0x2 =>  PageFaultErrorCode::CausedByWrite,
            0x3 =>  PageFaultErrorCode::UserMode,
            0x4 =>  PageFaultErrorCode::MalformedTable,
            0x5 =>  PageFaultErrorCode::InstructionFetch,
            _ => PageFaultErrorCode::Unknown
        }
    }
}

pub unsafe extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    let cr2: usize;
    asm! {
        "mov {}, cr2",
        out(reg) cr2
    };

    let cause: PageFaultErrorCode = error_code.into();
    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x} with error code {:?}\n{:#?}",
             cr2,
             Into::<PageFaultErrorCode>::into(error_code).name(),
             &*stack_frame);
    loop {}
}