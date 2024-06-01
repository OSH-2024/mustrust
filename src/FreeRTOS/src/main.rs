#![no_std]
#![no_main]
#![feature(asm)]

mod uart;

use core;
use core::arch::asm;
use core::panic::PanicInfo;
use cty;
use crate::uart::*;

#[no_mangle]
pub extern "C" fn io_halt() {
    unsafe {
        asm!("wfi");
    }
}

pub extern "C" fn TaskA(pvParameters: *mut cty::c_void) {
    unsafe {
        uart_puts("start TaskA\n");
        loop {
            uart_puthex(xTaskGetTickCount());
            uart_putchar(b'\n');
            vTaskDelay((500 / portTICK_RATE_MS) as u64);
        }
    }
}

static mut timer: TimerHandle_t = 0 as *mut cty::c_void;
static mut count: cty::uint32_t = 0;

#[no_mangle]
pub extern "C" fn interval_func(pxTimer: TimerHandle_t) {
    let mut buf: [cty::c_char; 2] = [0; 2];
    let mut len: cty::c_uint = 0;
    len = uart_read_bytes(&mut buf, (buf.len() - 1) as u32);
    if len > 0 {
        uart_puts(&buf);
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        let task_a: TaskHandle_t = 0 as *mut cty::c_void;
        uart_init();
        uart_puts("qemu exit: Ctrl-A x / qemu monitor: Ctrl-A c\n");
        uart_puts("hello world\n");
        xTaskCreate(Some(TaskA), "Task A", 512, 0 as *mut cty::c_void, tskIDLE_PRIORITY as u64, &mut task_a);
        timer = xTimerCreate("print_every_10ms", (10 / portTICK_RATE_MS) as u64, pdTRUE as u64, 0 as *mut cty::c_void, Some(interval_func));
        if timer != (0 as *mut cty::c_void) {
            xTimerStart(timer, 0);
        }
    }
	vTaskStartScheduler();
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
