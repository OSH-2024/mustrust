#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
#![feature(const_trait_impl)]
#![feature(effects)]

mod bindings;
mod FreeRTOS_tick_config;
mod kernel;
mod list;
mod mmutest;
mod port;
mod projdefs;
mod queue;
mod queue_api;
mod queue_h;
mod rwlock;
mod semaphore;
mod task_global;
mod task_queue;
mod tasks;
mod trace;
mod uart;

extern crate alloc;

use core;
use core::arch::asm;
use core::panic::PanicInfo;
use cty;
use crate::uart::*;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();


#[no_mangle]
pub extern "C" fn io_halt() {
    unsafe {
        asm!("wfi");
    }
}

fn print_rate(a: u64, b: u64) {
    uart_putdec(a);
    uart_puts(" / ");
    uart_putdec(b);
    uart_puts(" (");
    let epct = a * 100000000 / b;
    uart_putdec(epct / 1000000);
    uart_puts(".");
    uart_putdec_sized(epct % 1000000, 6);
    uart_puts("%)\n");
}

pub extern "C" fn TaskA(pvParameters: *mut cty::c_void) {
    unsafe {
        uart_puts("MMU Testing task start\n");
        uart_puts("Task 1. Bubble sort test\n");
        mmutest::random_initialize(0, 114514);
        bindings::initialize_tcb();
        bindings::initialize_list();
        uart_puts("Initialized array.\n");
        mmutest::bubble_sort();
        bindings::write_back();
        uart_puts("Task 1 ended.\n");
        uart_puts("TLB hit rate: ");
        print_rate(bindings::TLB_hit as u64, (bindings::TLB_hit + bindings::TLB_miss) as u64);
        uart_puts("Memory hit rate: ");
        print_rate(bindings::memory_hit as u64, (bindings::memory_hit + bindings::memory_miss) as u64);
        uart_puts("Estimated execution time: ");
        uart_putdec(bindings::time_cost as u64);
        uart_puts("ns\n");
    }
}

static mut timer: bindings::TimerHandle_t = 0 as *mut cty::c_void;
static mut count: cty::uint32_t = 0;

#[no_mangle]
pub extern "C" fn interval_func(pxTimer: bindings::TimerHandle_t) {
    let mut buf: [cty::c_char; 2] = [0; 2];
    let mut len: cty::c_uint = 0;
    let buf_len: u32 = (buf.len() - 1) as u32;
    len = uart_read_bytes(&mut buf, buf_len as u32);
    if len > 0 {
        if let Ok(nbuf) = core::str::from_utf8(&buf) {
            uart_puts(&nbuf);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let task_name = b"TaskA\0".as_ptr() as *const cty::c_char;
    let timer_name = b"print_every_10ms\0".as_ptr() as *const cty::c_char;
    unsafe {
        let mut task_a: bindings::TaskHandle_t = 0 as *mut cty::c_void;
        uart_init();
        uart_puts("qemu exit: Ctrl-A x / qemu monitor: Ctrl-A c\n");
        uart_puts("Program by MUSTRUST, USTC OSH 2024\n");
        bindings::xTaskCreate(Some(TaskA), task_name, 512, 0 as *mut cty::c_void, bindings::tskIDLE_PRIORITY as u64, &mut task_a);
        timer = bindings::xTimerCreate(timer_name, (10 / portTICK_RATE_MS!()) as u64, bindings::pdTRUE as u64, 0 as *mut cty::c_void, Some(interval_func));
        if timer != (0 as *mut cty::c_void) {
            bindings::xTimerStart(timer, 0);
        }
	    bindings::vTaskStartScheduler();
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _exit() -> ! {
    loop {}
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn vApplicationIdleHook() {}

#[no_mangle]
fn vApplicationTickHook() {}
