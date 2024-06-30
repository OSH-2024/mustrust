#[macro_export]
macro_rules! configTICK_RATE_HZ { () => { ( 1000 as u32 ) }; }
#[macro_export]
macro_rules! configMAX_PRIORITIES { () => { ( 8 ) }; }
#[macro_export]
macro_rules! configMINIMAL_STACK_SIZE { () => { ( 200 as u16 ) }; }
#[macro_export]
macro_rules! configTOTAL_HEAP_SIZE { () => { ( 124 * 1024 ) }; }

/* Software timer definitions. */
#[macro_export]
macro_rules! configTIMER_TASK_PRIORITY { () => { ( configMAX_PRIORITIES!() - 1 ) }; }
#[macro_export]
macro_rules! configTIMER_QUEUE_LENGTH { () => { 5 }; }
#[macro_export]
macro_rules! configTIMER_TASK_STACK_DEPTH { () => { ( configMINIMAL_STACK_SIZE!() * 2 ) }; }


#[macro_export]
macro_rules! configSETUP_TICK_INTERRUPT { () => { vConfigureTickInterrupt() }; }
#[macro_export]
macro_rules! configCLEAR_TICK_INTERRUPT { () => { vClearTickInterrupt() }; }
