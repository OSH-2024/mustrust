macro_rules! configUSE_PREEMPTION { () => { 1 }; }
macro_rules! configUSE_IDLE_HOOK { () => { 1 }; }
macro_rules! configUSE_TICK_HOOK { () => { 1 }; }
macro_rules! configTICK_RATE_HZ { () => { ( 1000 as u32 ) }; }
macro_rules! configMAX_PRIORITIES { () => { ( 8 ) }; }
macro_rules! configMINIMAL_STACK_SIZE { () => { ( 200 as u16 ) }; }
macro_rules! configTOTAL_HEAP_SIZE { () => { ( 124 * 1024 ) }; }
macro_rules! configUSE_16_BIT_TICKS { () => { 0 }; }

macro_rules! configUSE_MUTEXES { () => { 1 }; }

/* Software timer definitions. */
macro_rules! configUSE_TIMERS { () => { 1 }; }
macro_rules! configTIMER_TASK_PRIORITY { () => { ( configMAX_PRIORITIES!() - 1 ) }; }
macro_rules! configTIMER_QUEUE_LENGTH { () => { 5 }; }
macro_rules! configTIMER_TASK_STACK_DEPTH { () => { ( configMINIMAL_STACK_SIZE!() * 2 ) }; }

/* Set the following definitions to 1 to include the API function, or zero
to exclude the API function. */
macro_rules! INCLUDE_vTaskDelay { () => { 1 }; }

macro_rules! INCLUDE_xSemaphoreGetMutexHolder { () => { 1 }; }

macro_rules! configSETUP_TICK_INTERRUPT { () => { vConfigureTickInterrupt() }; }
macro_rules! configCLEAR_TICK_INTERRUPT { () => { vClearTickInterrupt() }; }
