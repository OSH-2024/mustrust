#![no_std]
#![no_main]

mod bindings;
mod uart;

use core;
use core::panic::PanicInfo;
use cty;
use crate::bindings::*;
use crate::uart::*;

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

fn vApplicationIdleHook() {}
fn vApplicationTickHook() {}
