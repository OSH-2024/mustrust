use core::sync::atomic::{AtomicUsize, Ordering};

static mut HEAP: [u8; configTOTAL_HEAP_SIZE] = [0; configTOTAL_HEAP_SIZE];
static NEXT_FREE_BYTE: AtomicUsize = AtomicUsize::new(0);

const configADJUSTED_HEAP_SIZE: usize = configTOTAL_HEAP_SIZE - portBYTE_ALIGNMENT;

pub unsafe fn pvPortMalloc(wanted_size: usize) -> *mut u8 {
    let mut adjusted_size = wanted_size;

    #[cfg(portBYTE_ALIGNMENT > 1)]
    {
        if adjusted_size & portBYTE_ALIGNMENT_MASK != 0 {
            adjusted_size += portBYTE_ALIGNMENT - (adjusted_size & portBYTE_ALIGNMENT_MASK);
        }
    }

    vTaskSuspendAll();
    let next_free_byte = NEXT_FREE_BYTE.load(Ordering::Relaxed);

    let mut pv_return: *mut u8 = core::ptr::null_mut();
    if next_free_byte + adjusted_size <= configADJUSTED_HEAP_SIZE
        && next_free_byte + adjusted_size > next_free_byte
    {
        pv_return = HEAP.as_mut_ptr().offset(next_free_byte as isize);
        NEXT_FREE_BYTE.store(next_free_byte + adjusted_size, Ordering::Relaxed);
    }

    #[cfg(configUSE_MALLOC_FAILED_HOOK)]
    {
        if pv_return.is_null() {
            vApplicationMallocFailedHook();
        }
    }

    xTaskResumeAll();
    pv_return
}

pub unsafe fn vPortFree(_pv: *mut u8) {
    // Memory cannot be freed using this scheme.
    #[cfg(debug_assertions)]
    debug_assert!(_pv.is_null());
}

pub unsafe fn vPortInitialiseBlocks() {
    NEXT_FREE_BYTE.store(0, Ordering::Relaxed);
}

pub fn xPortGetFreeHeapSize() -> usize {
    configADJUSTED_HEAP_SIZE - NEXT_FREE_BYTE.load(Ordering::Relaxed)
}

#[cfg(configUSE_MALLOC_FAILED_HOOK)]
extern "C" fn vApplicationMallocFailedHook() {}
