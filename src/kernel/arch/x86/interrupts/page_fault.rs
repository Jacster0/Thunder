use core::arch::asm;
use x86_64::structures::paging::Page;
use crate::enum_str;
use crate::kernel::arch::x86::interrupts::exception::page_fault;

enum_str! {
    enum AccessMode {
        Supervisor = 0x0,
        User = 0x1,
        Unknown = 0x2,
    }
}

enum_str! {
    enum PageFaultErrorCode {
        NonPresentRead = 0x0,
        ProtectionViolationRead = 0x1,
        NonPresentWrite = 0x2,
        ProtectionViolationWrite = 0x3,
        Unknown = 0x4,
    }
}

pub struct PageFaultBitMasks;

impl PageFaultBitMasks {
    pub const ERROR_CODE: usize = 0x3;
    pub const ACCES_MODE: usize = 0x4;
    pub const INSTRUCTION_FETCH: usize = 0x10;
    pub const RESERVED: usize = 0x9;
}

pub struct PageFault {
    pub addr: usize,
    pub error_code_description: PageFaultErrorCode,
    pub access_mode: AccessMode,
    pub caused_by_instruction_fetch: bool,
    pub reserved: bool,
}

impl PageFault {
    fn get_addr() -> usize {
        //when a page fault occurs, the address (Page Fault Linear Address aka PFLA)
        //that the program attempted to access is stored in the cr2 register
        let page_fault_linear_address: usize;

        unsafe {
            asm! {
                "mov {}, cr2", //move the value stored in cr2 register to our page_fault_linear_address variable so that we can return the value
                out(reg) page_fault_linear_address
            };
        }

        return page_fault_linear_address;
    }
}

pub struct PageFaultBuilder;

impl PageFaultBuilder {
    pub fn build(code: usize) -> PageFault {
        let page_fault_error =  match code & PageFaultBitMasks::ERROR_CODE {
            0x0 =>  PageFaultErrorCode::NonPresentRead,
            0x1 =>  PageFaultErrorCode::ProtectionViolationRead,
            0x2 =>  PageFaultErrorCode::NonPresentWrite,
            0x3 =>  PageFaultErrorCode::ProtectionViolationWrite,
            _ =>    PageFaultErrorCode::Unknown
        };

        let access_mode = match code & PageFaultBitMasks::ACCES_MODE {
            0x0 => AccessMode::Supervisor,
            0x1 => AccessMode::User,
            _ => AccessMode::Unknown
        };

        PageFault {
            error_code_description: page_fault_error,
            access_mode,
            caused_by_instruction_fetch: (code & PageFaultBitMasks::INSTRUCTION_FETCH) != 0,
            reserved: (code & PageFaultBitMasks::RESERVED) != 0,
            addr: PageFault::get_addr()
        }
    }
}