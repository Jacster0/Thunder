#![no_std]
#![no_main]

mod kernel;
use core::panic::PanicInfo;
use kernel::lib::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    panic!("panic test");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}