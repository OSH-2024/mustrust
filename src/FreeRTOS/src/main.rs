#![no_std]
#![no_main]
use core::panic::PanicInfo;
// include!("uart.rs");

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
