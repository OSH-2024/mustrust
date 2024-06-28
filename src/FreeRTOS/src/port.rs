use crate::bindings::*;
use cty;

pub type StackType_t = usize;
pub type BaseType_t = i64;
pub type UBaseType_t = u64;
pub type TickType_t = u32;
pub type CVoidPointer = *mut cty::c_void;

pub const portBYTE_ALIGNMENT_MASK: UBaseType_t = 4;
pub const portMAX_DELAY: TickType = configUSE_16_BIT_TICKS!() ? 0xffff : 0xffffffff;

#[macro_export]
macro_rules! portYIELD { () => { unsafe { asm!("SVC 0"); } } }

#[macro_export]
macro_rules! portDISABLE_INTERRUPTS { () => {
    unsafe {
        asm!("MSR DAIFSET, #2");
        asm!("DSB SY");
        asm!("ISB SY");
    }
} }

#[macro_export]
macro_rules! portENABLE_INTERRUPTS { () => {
    unsafe {
        asm!("MSR DAIFCLR, #2");
        asm!("DSB SY");
        asm!("ISB SY");
    }
} }


#[macro_export]
macro_rules! portSTACK_GROWTH { () => { -1 } }

#[macro_export]
macro_rules! portTICK_PERIOD_MS { () => { (1000 / configTICK_RATE_HZ!()) as u64 } }

#[macro_export]
macro_rules! portBYTE_ALIGNMENT { () => { 16 } }

type portPOINTER_SIZE_TYPE = u64;

#[macro_export]
macro_rules! portYIELD_WITHIN_API {
    () => { portYIELD!() };
}
