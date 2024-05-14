include!("bindings.rs");
use core::ptr::*;

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
    tx_mux: *mut SemaphoreHandle_t,
    rx_queue: *mut QueueHandle_t
}

static mut uartctl: *mut UARTCTL;

fn uart_putchar(c: u8) {
    unsafe {
        xSemaphoreTake(&uartctl.tx_mux, portMAX_DELAY);
        while !(read_volatile(AUX_MU_LSR) & 0x20) {}
        write_volatile(AUX_MU_IO, c as u32);
        xSemaphoreGive(&uartctl.tx_mux);
    }
}
