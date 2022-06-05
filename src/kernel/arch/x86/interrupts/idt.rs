use core::arch::asm;
use lazy_static::lazy_static;
use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use modular_bitfield::prelude::*;
use x86_64::instructions::segmentation::CS;
use x86_64::registers::segmentation::Segment;
use crate::kernel::arch::x86::interrupts::{exception, idt};
use crate::println;
use crate::kernel::arch::x86::interrupts::exception::*;
use crate::interrupt_error;
use crate::interrupt_error_with_code;

pub type HandlerFunction = extern "C" fn() -> !;
pub struct InterruptDescriptorTable([Entry; 16]);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

pub enum GateType {
    Interrupt = 0xE,
    Trap = 0xF
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct InterruptDescriptorTableRegister {
    pub limit: u16,
    pub baseAddr: u64,
}

#[inline]
pub fn load_idt(idt_register: &InterruptDescriptorTableRegister) {
    unsafe {
        asm! {
            "lidt [{}]",
            in(reg) idt_register,
            options(readonly, nostack, preserves_flags)
        };
    }
}

pub struct Attributes {
    pub gate_type: GateType,
    pub privilege_level: PrivilegeLevel,
    pub present: bool
}

impl Attributes {
    pub const fn new() -> Attributes {
        Attributes {
            gate_type: GateType::Interrupt,
            privilege_level: PrivilegeLevel::Ring0,
            present: true
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    address_low: u16,
    selector: SegmentSelector,
    ist: u8,
    attributes: u8,
    address_middle: u16,
    address_high: u32,
    reserved: u32,
}

impl Entry {
    pub const fn new() -> Entry {
        Entry {
            address_low: 0,
            selector: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
            ist: 0,
            attributes: 0,
            address_middle: 0,
            address_high: 0,
            reserved: 0
        }
    }

    pub fn set_interrupt_stack_table(&mut self, ist: u8) {
        self.ist = (self.ist & 0xF8) | ist;
    }

    pub fn set_handler(&mut self, selector: SegmentSelector, handler: HandlerFunction) {
        let ptr = handler as u64;

        self.selector = selector;
        self.address_low = ptr as u16;
        self.address_middle =  (ptr >> 16) as u16;
        self.address_high = (ptr >> 32) as u32;
    }


    pub fn set_attributes(&mut self, attr: Attributes) {
        self.attributes = (self.attributes & 0x7F) | (attr.present as u8) << 0x7;
        self.attributes = (self.attributes & 0x9F) | (attr.privilege_level as u8) << 0x5;
        self.attributes = (self.attributes & 0xF0) | (attr.gate_type as u8);
    }
 }

impl InterruptDescriptorTable {
    pub fn load(&'static self) {
        use core::mem::size_of;

        let idt_register = InterruptDescriptorTableRegister {
            limit: (size_of::<Self>() -1) as u16,
            baseAddr: self as *const _ as u64
        };

        load_idt(&idt_register)
    }

    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable([Entry::new(); 16])
    }

    pub fn init(&mut self, index: usize, handler: HandlerFunction) {
        let mut entry = Entry::new();
        entry.set_handler(CS::get_reg(), handler);
        entry.set_attributes(Attributes::new());
        entry.set_interrupt_stack_table(0);

        self.0[index] = entry;
    }

    pub fn disable_interrupts(&mut self, entry: usize) {
        self.0[entry].attributes = (self.0[entry].attributes & 0xF0) | (GateType::Trap as u8);
    }

    pub fn enable_interrupts(&mut self, entry: usize) {
        self.0[entry].attributes = (self.0[entry].attributes & 0xF0) | (GateType::Interrupt as u8);
    }

    pub fn set_handler(&mut self, entry: usize, handler: HandlerFunction) {
        self.0[entry].set_handler(CS::get_reg(), handler);
        self.0[entry].set_attributes(Attributes::new());
        self.0[entry].set_interrupt_stack_table(0);
    }

    pub fn set_presentation(&mut self, entry: u8, value: bool) {
        self.0[entry as usize].attributes = (self.0[entry as usize].attributes & 0x7F) | (value as u8) << 0x7;
    }
}

lazy_static! {
    pub static ref IDT: idt::InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.init(0,  interrupt_error!(divide_by_zero_handler));
        idt.set_handler(14, interrupt_error_with_code!(page_fault_handler));
        idt
    };
}

pub fn init() {
    IDT.load();
}