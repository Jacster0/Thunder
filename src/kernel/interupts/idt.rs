use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use modular_bitfield::prelude::*;
use x86_64::instructions::segmentation::CS;
use x86_64::registers::segmentation::Segment;

pub type HandlerFunction = extern "C" fn() -> !;
pub struct Idt([Entry; 16]);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

#[repr(C, packed)]
pub struct Entry {
    address_low: u16,
    selector: SegmentSelector,
    ist: InterruptStackTable,
    attributes: Attributes,
    address_middle: u16,
    address_high: u32,
    reserved: u32,
}

#[bitfield]
pub struct InterruptStackTable {
    offset: B3,
    unused: B5
}

#[bitfield]
pub struct Attributes {
    gate_type: B1,
    unused_one: B3,
    unused_zero: B1,
    descriptor_privilege_level: B2,
    present: B1,
}

impl Entry {
    fn new(selector: SegmentSelector, ist: InterruptStackTable, attr: Attributes, handler: HandlerFunction) -> Entry {
        let ptr = handler as u64;

        Entry {
            selector,
            address_low: ptr as u16,
            address_middle: (ptr >> 16) as u16,
            address_high: (ptr >> 32) as u32,
            ist,
            reserved: 0,
            attributes: attr
        }
    }
}

impl Idt {
    pub fn init(&mut self, entry: u8, handler: HandlerFunction) {
        let ist = InterruptStackTable::new()
            .with_offset(0)
            .with_unused(0);

        let attr = Attributes::new().with_gate_type(1)
            .with_unused_one(1)
            .with_unused_zero(0)
            .with_descriptor_privilege_level(PrivilegeLevel::Ring0 as u8)
            .with_present(1);

        self.0[entry as usize] = Entry::new(CS::get_reg(),ist, attr, handler );
    }

    pub fn disable_interrupts(&mut self, entry: u8) {
        self.0[entry as usize].attributes.set_gate_type(0);
    }

    pub fn enable_interrupts(&mut self, entry: u8) {
        self.0[entry as usize].attributes.set_gate_type(1);
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunction) {
        let ptr = handler as u64;

        self.0[entry as usize].selector = CS::get_reg();
        self.0[entry as usize].address_low = ptr as u16;
        self.0[entry as usize].address_middle =  (ptr >> 16) as u16;
        self.0[entry as usize].address_high = (ptr >> 32) as u32;
    }

    pub fn set_presentation(&mut self, entry: u8, value: bool) {
        self.0[entry as usize].attributes.set_present(value as u8);
    }
}