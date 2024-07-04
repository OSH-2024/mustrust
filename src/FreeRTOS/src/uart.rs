use core::arch::asm;
use core::ptr::*;
use core::intrinsics::*;
use crate::bindings;

/* Defines */
pub const GPFSEL1: *mut u32 = 0x3F200004 as *mut u32;
pub const GPPUDCLK0: *mut u32 = 0x3F200098 as *mut u32;

pub const AUX_ENABLES: *mut u32 = 0x3F215004 as *mut u32;
pub const AUX_MU_IO: *mut u32 = 0x3F215040 as *mut u32;
pub const AUX_MU_IER: *mut u32 = 0x3F215044 as *mut u32;
pub const AUX_MU_IIR: *mut u32 = 0x3F215048 as *mut u32;
pub const AUX_MU_LCR: *mut u32 = 0x3F215048 as *mut u32;
pub const AUX_MU_LSR: *mut u32 = 0x3F215054 as *mut u32;
pub const AUX_MU_BAUD: *mut u32 = 0x3F215068 as *mut u32;

struct UARTCTL {
    tx_mux: bindings::SemaphoreHandle_t,
    rx_queue: bindings::QueueHandle_t
}

static mut uartctl: *mut UARTCTL = 0 as *mut UARTCTL;

pub fn uart_putchar(c: u8) {
    unsafe {
        bindings::xSemaphoreTake((*uartctl).tx_mux, crate::portMAX_DELAY!());
        while (unaligned_volatile_load(AUX_MU_LSR as *const u32) & 0x20) == 0 {}
        unaligned_volatile_store(AUX_MU_IO, c as u32);
        bindings::xSemaphoreGive((*uartctl).tx_mux);
    }
}

pub fn uart_putchar_isr(c: u8) {
    unsafe {
        bindings::xSemaphoreTakeFromISR((*uartctl).tx_mux, core::ptr::null_mut());
        while (unaligned_volatile_load(AUX_MU_LSR) & 0x20) == 0 {}
        unaligned_volatile_store(AUX_MU_IO, c as u32);
        bindings::xSemaphoreGiveFromISR((*uartctl).tx_mux, core::ptr::null_mut());
    }
}

pub fn uart_puts(str: &str) {
    for c in str.bytes() {
        uart_putchar(c);
    }
}

pub fn uart_puthex(v: u64) {
    let hexdigits = "0123456789ABCDEF".as_bytes();
    for i in (0..=60).rev().step_by(4) {
        uart_putchar(hexdigits[((v >> i) & 0xf) as usize]);
    }
}

pub fn uart_putdec(v: u64) {
    let digits = "0123456789".as_bytes();
    if (v == 0) {
        uart_putchar(b'0');
        return;
    }
    let mut v = v;
    let mut w = 1u64;
    while w <= v {
        w *= 10;
    }
    w /= 10;

    while w > 0 {
        uart_putchar(digits[(v / w) as usize]);
        v %= w;
        w /= 10;
    }
}

pub fn uart_putdec_sized(v: u64, len: i32) {
    let digits = "0123456789".as_bytes();
    if (v == 0) {
        uart_putchar(b'0');
        return;
    }
    let mut v = v;
    let mut w = 1u64;
    for _ in 0..len {
        w *= 10;
    }
    w /= 10;

    while w > 0 {
        uart_putchar(digits[(v / w) as usize]);
        v %= w;
        w /= 10;
    }
}

pub fn uart_read_bytes(buf: &mut [u8], length: u32) -> u32 {
    let num: u32 = unsafe { bindings::uxQueueMessagesWaiting((*uartctl).rx_queue) } as u32;
    let mut i: u32 = 0;

    while i < num || i < length {
        unsafe {
            bindings::xQueueReceive((*uartctl).rx_queue, *(&mut buf[i as usize]) as *mut cty::c_void, crate::portMAX_DELAY!());
        }
        i += 1;
    }

    i
}

type InterruptHandler = Option<unsafe extern "C" fn()>;

#[derive(Copy, Clone)]
struct InterruptVector {
    r#fn: InterruptHandler,
}

static mut g_vector_table: [InterruptVector; 64] = [InterruptVector { r#fn: None }; 64];
pub const IRQ_ENABLE_1: *mut u32 = 0x3F00B210 as *mut u32;

pub fn uart_isr_register(r#fn: unsafe extern "C" fn()) {
    unsafe {
        g_vector_table[29].r#fn = Some(r#fn);

        unaligned_volatile_store(AUX_ENABLES, 1);
        unaligned_volatile_store(AUX_MU_IIR, 6);
        unaligned_volatile_store(AUX_MU_IER, 1);

        volatile_store(IRQ_ENABLE_1, (1 << 29) as u32);
    }
}

pub unsafe extern "C" fn uart_isr() {
    if unaligned_volatile_load(AUX_MU_LSR) & 1 != 0 {
        let c = unaligned_volatile_load(AUX_MU_IO) as u8;
        bindings::xQueueSendToBackFromISR((*uartctl).rx_queue, *(&c) as *mut cty::c_void, core::ptr::null_mut());
    }
}

pub fn uart_init() {
    unsafe {
        let mut r = unaligned_volatile_load(GPFSEL1);
        r &= !(7<<12|7<<15);
        r |= 2<<12|2<<15;
        unaligned_volatile_store(GPFSEL1, r);

        for _ in 0..150 {
            asm!("nop");
        }

        unaligned_volatile_store(GPPUDCLK0, (1<<14)|(1<<15));

        for _ in 0..150 {
            asm!("nop");
        }

        unaligned_volatile_store(GPPUDCLK0, 0);

        unaligned_volatile_store(AUX_MU_BAUD, 270);
        unaligned_volatile_store(AUX_MU_LCR, 3);

        uartctl = bindings::pvPortMalloc(core::mem::size_of::<UARTCTL>() as usize) as *mut UARTCTL;
        (*uartctl).tx_mux = bindings::xSemaphoreCreateMutex();
        (*uartctl).rx_queue = bindings::xQueueCreate(16, core::mem::size_of::<u8>() as u64);
        uart_isr_register(uart_isr);
    }
}

const IRQ_BASIC_PENDING: *mut u32 = 0x3F00B200 as *mut u32;
const IRQ_PENDING_1: *mut u32 = 0x3F00B204 as *mut u32;
const IRQ_PENDING_2: *mut u32 = 0x3F00B208 as *mut u32;

pub fn handle_range(pending: u32, base: u32) {
    let mut pending = pending;
    while pending != 0 {
        let bit = 31 - pending.leading_zeros();
        let irq = base + bit;

        if let Some(handler) = unsafe { g_vector_table[irq as usize].r#fn } {
            unsafe { handler(); }
        }

        pending &= !(1 << bit);
    }
}

pub unsafe extern "C" fn irq_handler() {
    let basic = unaligned_volatile_load(IRQ_BASIC_PENDING) & 0x00000300;

    if basic & 0x100 != 0 {
        handle_range(unaligned_volatile_load(IRQ_PENDING_1), 0);
    }
    if basic & 0x200 != 0 {
        handle_range(unaligned_volatile_load(IRQ_PENDING_2), 32);
    }
}