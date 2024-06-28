use crate::bindings::*;
use cty;

pub type StackType = usize;
pub type BaseType = i64;
pub type UBaseType = u64;
pub type TickType = u32;
pub type CVoidPointer = *mut cty::c_void;

pub const portBYTE_ALIGNMENT_MASK: UBaseType = 4;
pub const portMAX_DELAY: TickType = configUSE_16_BIT_TICKS!() ? 0xffff : 0xffffffff;

#[macro_export]
macro_rules! portYIELD { () => { unsafe { asm!("SVC 0"); } } }

#[macro_export]
macro_rules! portYIELD_WITHIN_API {
    () => { portYIELD!() };
}

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

pub fn port_malloc(size: usize) -> Result<CVoidPointer, FreeRtosError> {
    unsafe {
        let mut ret_ptr: *mut c_void = core::ptr::null_mut();
        if size == 0 {
            Err(FreeRtosError::OutOfMemory)
        }
        else {
            Ok(ret_ptr)
        }
    }
}

pub fn port_initialize_stack(
    top_of_stack: *mut StackType,
    code: StackType,
    param_ptr: *mut c_void,
) -> Result<*mut StackType, FreeRtosError> {
    let num: usize = 0;
    let mut ret_val: *mut usize = core::ptr::null_mut();
    if ret_val.is_null() {
        // Initialize stack failed
        Err(FreeRtosError::PortError)
    } else {
        Ok(ret_val)
    }
}
