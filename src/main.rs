#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]

mod kernel;

use core::arch::asm;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;
use kernel::lib::print;
use crate::idt::{Attributes, Entry, Idt};
use crate::kernel::interrupts::idt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    idt::init();
    divide_by_zero();
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