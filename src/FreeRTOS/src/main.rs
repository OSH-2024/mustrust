#![no_std]
#![no_main]
use core::panic::PanicInfo;
// include!("uart.rs");

#[no_mangle]
fn main() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
