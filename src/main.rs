#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(asm_sym)]

mod kernel;

use core::arch::asm;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;
use kernel::lib::print;
use crate::idt::{Attributes, Entry, InterruptDescriptorTable};
use kernel::arch::x86::interrupts::idt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    idt::init();
    unsafe { *(0xdeadbeaf as *mut u64) = 42 };
    //divide_by_zero();
    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn divide_by_zero() {
    unsafe {
        asm! {
            "mov dx, 0",
            "div dx",
        }
    }
}