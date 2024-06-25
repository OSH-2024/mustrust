include!("FreeRTOSConfig.rs");
use cty;
use core::arch::asm;
use core::ptr::*;
use core::intrinsics::*;

use crate::bindings;
use crate::uart;

pub const CORE0_TIMER_IRQCNTL: *mut u32 = 0x40000040 as *mut u32;
static mut timer_cntfrq: cty::uint32_t = 0;
static mut timer_tick: cty::uint32_t = 0;

#[no_mangle]
pub fn enable_cntv() {
    unsafe {
        let mut cntv_ctl: cty::uint32_t = 1;
        asm!("msr cntv_ctl_el0, {0}", in(reg) cntv_ctl);
    }
}

#[no_mangle]
pub fn write_cntv_tval(val: cty::uint32_t) {
    unsafe {
        asm!("msr cntv_tval_el0, {0}", in(reg) val);
    }
}

#[no_mangle]
pub fn read_cntfrq() -> cty::uint32_t {
    unsafe {
        let mut val: cty::uint32_t = 0;
        asm!("mrs {0}, cntfrq_el0", out(reg) val);
        val
    }
}

#[no_mangle]
pub fn init_timer() {
    unsafe {
        timer_cntfrq = read_cntfrq();
        timer_tick = timer_cntfrq;
        write_cntv_tval(timer_cntfrq);
    }
}

#[no_mangle]
pub fn timer_set_tick_rate_hz(rate: cty::uint32_t) {
    unsafe {
        timer_tick = timer_cntfrq / rate;
        write_cntv_tval(timer_tick);
    }
}

#[no_mangle]
pub fn vConfigureTickInterrupt() {
    unsafe {
        init_timer();
        timer_set_tick_rate_hz(configTICK_RATE_HZ!());
        volatile_store(CORE0_TIMER_IRQCNTL, 8 as u32);
        enable_cntv();
    }
}

#[no_mangle]
pub fn vClearTickInterrupt() {
    unsafe {
        write_cntv_tval(timer_tick);
    }
}

#[no_mangle]
pub fn vApplicationIRQHandler(ulCORE0_INT_SRC: cty::uint32_t) {
    let ulInterruptID = ulCORE0_INT_SRC & 0x0007FFFF;
    unsafe {
        if (ulInterruptID & (1 << 3)) != 0 {
            bindings::FreeRTOS_Tick_Handler();
        }
        if (ulInterruptID & (1 << 8)) != 0 {
            uart::irq_handler();
        }
    }
}
