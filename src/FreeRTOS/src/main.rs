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
        uart_puts("[TaskA] MMU Testing task start\n");
        uart_puts("[TaskA] Task 1. Bubble sort test\n");
        mmutest::random_initialize(0, 1145141919);
        bindings::initialize_stat();
        bindings::initialize_tcb();
        bindings::initialize_list();
        uart_puts("[TaskA] Initialized array.\n");
        mmutest::bubble_sort();
        bindings::write_back();
        uart_puts("[TaskA] Task 1 ended.\n");
        bindings::uninitialize_tcb();
        uart_puts("[TaskA] TLB hit rate: ");
        print_rate(bindings::TLB_hit as u64, (bindings::TLB_hit + bindings::TLB_miss) as u64);
        uart_puts("[TaskA] Memory hit rate: ");
        print_rate(bindings::memory_hit as u64, (bindings::memory_hit + bindings::memory_miss) as u64);
        uart_puts("[TaskA] Estimated execution time: ");
        uart_putdec(bindings::time_cost as u64);
        uart_puts("ns\n");
        uart_puts("[TaskA] ====================\n");
        uart_puts("[TaskA] Task 2. Quick sort test\n");
        mmutest::random_initialize(0, 1145141919);
        bindings::initialize_stat();
        bindings::initialize_tcb();
        bindings::initialize_list();
        uart_puts("[TaskA] Initialized array.\n");
        mmutest::quick_sort(0, mmutest::virtual_space as i32 - 1);
        bindings::write_back();
        uart_puts("[TaskA] Task 2 ended.\n");
        bindings::uninitialize_tcb();
        uart_puts("[TaskA] TLB hit rate: ");
        print_rate(bindings::TLB_hit as u64, (bindings::TLB_hit + bindings::TLB_miss) as u64);
        uart_puts("[TaskA] Memory hit rate: ");
        print_rate(bindings::memory_hit as u64, (bindings::memory_hit + bindings::memory_miss) as u64);
        uart_puts("[TaskA] Estimated execution time: ");
        uart_putdec(bindings::time_cost as u64);
        uart_puts("ns\n");
        uart_puts("[TaskA] ====================\n");
        uart_puts("[TaskA] Test completed.\n");
        loop {}
    }
}

pub extern "C" fn TaskB(pvParameters: *mut cty::c_void) {
    unsafe {
        uart_puts("[TaskB] Timer task start\n");
        loop {
            uart_puts("[TaskB] Current timer: ");
            uart_putdec(bindings::xTaskGetTickCount());
		    uart_puts("\n");
            bindings::vTaskDelay(514 / portTICK_RATE_MS!());
        }
    }
}

pub extern "C" fn TaskC(pvParameters: *mut cty::c_void) {
    let s1 = "Never gonna give you up";
    let s2 = "Never gonna let you down";
    let s3 = "Never gonna run around and desert you";
    unsafe {
        uart_puts("[TaskC] Ni bei pian le!\n");
        loop {
            uart_puts("[TaskC] ");
            uart_puts(s1);
            uart_puts("\n");
            bindings::vTaskDelay(1958 / portTICK_RATE_MS!());
            uart_puts("[TaskC] ");
            uart_puts(s2);
            uart_puts("\n");
            bindings::vTaskDelay(1958 / portTICK_RATE_MS!());
            uart_puts("[TaskC] ");
            uart_puts(s3);
            uart_puts("\n");
            bindings::vTaskDelay(1958 / portTICK_RATE_MS!());
        }
    }
}

static mut pi_f: [u32; 35000] = [2000; 35000];

pub extern "C" fn TaskD(pvParameters: *mut cty::c_void) {
    unsafe {
        uart_puts("[TaskD] Pi first 10000 digits calculating task start\n");
        let a = 10000;
        let mut e = 0;
        let mut c = 35000;
        let mut cnt = 0;
        while c > 0 {
            let mut d = 0;
            let mut b = c;
            while b > 0 {
                d = d * b + pi_f[b as usize - 1] * a;
                let g = 2 * b - 1;
                pi_f[b as usize - 1] = d % g;
                d /= g;
                b -= 1;
            }
            uart_puts("[TaskD] Pi[");
            uart_putdec(cnt);
            uart_puts(":");
            uart_putdec(cnt + 3);
            uart_puts("] = ");
            uart_putdec_sized((e + d / a) as u64, 4);
            uart_puts("\n");
            bindings::vTaskDelay(300 / portTICK_RATE_MS!());
            e = d % a;
            c -= 14;
            cnt += 4;
        }
        loop {}
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
    let task_name_a = b"TaskA\0".as_ptr() as *const cty::c_char;
    let task_name_b = b"TaskB\0".as_ptr() as *const cty::c_char;
    let task_name_c = b"TaskC\0".as_ptr() as *const cty::c_char;
    let task_name_d = b"TaskD\0".as_ptr() as *const cty::c_char;
    let timer_name_a = b"print_every_10ms\0".as_ptr() as *const cty::c_char;
    unsafe {
        let mut task_a: bindings::TaskHandle_t = 0 as *mut cty::c_void;
        let mut task_b: bindings::TaskHandle_t = 0 as *mut cty::c_void;
        let mut task_c: bindings::TaskHandle_t = 0 as *mut cty::c_void;
        let mut task_d: bindings::TaskHandle_t = 0 as *mut cty::c_void;
        uart_init();
        uart_puts("qemu exit: Ctrl-A x / qemu monitor: Ctrl-A c\n");
        uart_puts("Program by MUSTRUST, USTC OSH 2024\n");
        bindings::xTaskCreate(Some(TaskA), task_name_a, 512, 0 as *mut cty::c_void, bindings::tskIDLE_PRIORITY as u64, &mut task_a);
        bindings::xTaskCreate(Some(TaskB), task_name_b, 512, 0 as *mut cty::c_void, bindings::tskIDLE_PRIORITY as u64, &mut task_b);
        bindings::xTaskCreate(Some(TaskC), task_name_c, 512, 0 as *mut cty::c_void, bindings::tskIDLE_PRIORITY as u64, &mut task_c);
        bindings::xTaskCreate(Some(TaskD), task_name_c, 512, 0 as *mut cty::c_void, bindings::tskIDLE_PRIORITY as u64, &mut task_d);
        timer = bindings::xTimerCreate(timer_name_a, (10 / portTICK_RATE_MS!()) as u64, bindings::pdTRUE as u64, 0 as *mut cty::c_void, Some(interval_func));
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
    uart_puts("panic!\n");
    loop {}
}

#[no_mangle]
fn vApplicationIdleHook() {}

#[no_mangle]
fn vApplicationTickHook() {}
