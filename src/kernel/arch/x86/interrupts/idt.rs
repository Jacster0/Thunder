use core::arch::asm;
use lazy_static::lazy_static;
use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use modular_bitfield::prelude::*;
use x86_64::instructions::segmentation::CS;
use x86_64::registers::segmentation::Segment;
use crate::kernel::arch::x86::interrupts::{exception, idt};
use crate::kernel::arch::x86::interrupts::exception::*;

use crate:: {
    println,
    interrupt_error,
    interrupt_error_with_code,
    save_scratch_registers,
    restore_scratch_registers,
    save_preserved_registers,
    restore_preserved_registers,
};

pub type HandlerFunction = extern "C" fn() -> !;
pub struct InterruptDescriptorTable([Entry; 31]);

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
pub struct InterruptPointer {
    pub limit: u16,
    pub base_addr: u64,
}

#[inline]
pub fn load_idt(idt_register: &InterruptPointer) {
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

        let idt_register = InterruptPointer {
            limit: (size_of::<Self>() -1) as u16,
            base_addr: self as *const _ as u64
        };

        load_idt(&idt_register)
    }

    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable([Entry::new(); 31])
    }

    pub fn disable_interrupts(&mut self, entry: usize) {
        self.0[entry].attributes = (self.0[entry].attributes & 0xF0) | (GateType::Trap as u8);
    }

    pub fn enable_interrupts(&mut self, entry: usize) {
        self.0[entry].attributes = (self.0[entry].attributes & 0xF0) | (GateType::Interrupt as u8);
    }

    pub fn register_handler(&mut self, entry: usize, handler: HandlerFunction) {
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

        idt.register_handler(0, interrupt_error!(divide_by_zero));
        idt.register_handler(1, interrupt_error!(debug));
        idt.register_handler(2, interrupt_error!(non_maskable_interrupt));
        idt.register_handler(3, interrupt_error!(breakpoint));
        idt.register_handler(4, interrupt_error!(overflow));
        idt.register_handler(5, interrupt_error!(bound_range_exceeded));
        idt.register_handler(6, interrupt_error!(invalid_opcode));
        idt.register_handler(7, interrupt_error!(device_not_available));
        idt.register_handler(8, interrupt_error_with_code!(double_fault));
        // 9 Coprocessor Segment Overrun, not available anymore
        idt.register_handler(10, interrupt_error_with_code!(invalid_tss));
        idt.register_handler(11, interrupt_error_with_code!(segment_not_present));
        idt.register_handler(12, interrupt_error_with_code!(stack_segment_fault));
        idt.register_handler(13, interrupt_error_with_code!(general_protection_fault));
        idt.register_handler(14, interrupt_error_with_code!(page_fault));
        // 15 reserved
        idt.register_handler(16, interrupt_error!(x87_floating_point_exception));
        idt.register_handler(17, interrupt_error_with_code!(alignment_check));
        //idt.register_handler(18, interrupt_error!());
        idt.register_handler(19, interrupt_error!(simd_floating_point_exception));
        idt.register_handler(20, interrupt_error!(virtualization_exception));
        // [21..29] reserved
        idt.register_handler(30, interrupt_error_with_code!(security_exception));

        idt
    };
}

pub fn init() {
    IDT.load();
}