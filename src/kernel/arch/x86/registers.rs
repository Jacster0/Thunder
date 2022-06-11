use crate::{print, println};

#[derive(Default)]
#[repr(packed)]
pub struct ScratchRegisters {
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rax: usize,
}

impl ScratchRegisters {
    pub fn dump(&self) {
        println!("RAX = 0x{:016x} ", { self.rax });
        println!("RCX = 0x{:016x} ", { self.rcx });
        println!("RDX = 0x{:016x} ", { self.rdx });
        println!("RDI = 0x{:016x} ", { self.rdi });
        println!("RSI = 0x{:016x} ", { self.rsi });
        println!("R8  = 0x{:016x} ", { self.r8 });
        println!("R9  = 0x{:016x} ", { self.r9 });
        println!("R10 = 0x{:016x} ", { self.r10 });
        println!("R11 = 0x{:016x} ", { self.r11 });
    }
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

impl PreservedRegisters {
    pub fn dump(&self) {
        println!("RBX = 0x{:016x} ", { self.rbx });
        println!("RBP = 0x{:016x} ", { self.rbp });
        println!("R12 = 0x{:016x} ", { self.r12 });
        println!("R13 = 0x{:016x} ", { self.r13 });
        println!("R14 = 0x{:016x} ", { self.r14 });
        println!("R15 = 0x{:016x} ", { self.r15 });
    }
}

#[derive(Default)]
#[repr(packed)]
pub struct IretRegisters {
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize
}

impl IretRegisters {
    pub fn dump(&self) {
        println!("RFLAG = 0x{:016x} ", { self.rflags });
        println!("CS    = 0x{:016x} ", { self.cs });
        println!("RIP   = 0x{:016x} ", { self.rip });
        println!("RSP   = 0x{:016x} ", { self.rsp });
        println!("SS    = 0x{:016x} ", { self.ss });
    }
}

#[derive(Default)]
#[repr(packed)]
pub struct StackFrame {
    pub preserved: PreservedRegisters,
    pub scratch: ScratchRegisters,
    pub iret: IretRegisters,
}

impl StackFrame {
    pub fn dump(&self) {
        self.scratch.dump();
        self.preserved.dump();
        self.iret.dump();
    }
}