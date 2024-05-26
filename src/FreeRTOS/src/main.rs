#![no_std]
#![no_main]
use core::panic::PanicInfo;
use cty;
use core;
// include!("uart.rs");
include!("bindings.rs");

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _exit() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
