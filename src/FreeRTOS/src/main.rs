#![feature(restrict_std)]
#![no_main]
use core::panic::PanicInfo;
// include!("uart.rs");
include!("bindings.rs");

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
