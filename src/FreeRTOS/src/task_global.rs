use crate::list::ListLink;
use crate::port::{BaseType, TickType, UBaseType};
use crate::task_control::TaskHandle;
use crate::*;
use no_std_async::rwlock::RwLock;

/* Some global variables. */
pub static mut TICK_COUNT: TickType = 0;
pub static mut TOP_READY_PRIORITY: UBaseType = 0;
pub static mut PENDED_TICKS: UBaseType = 0;
pub static mut SCHEDULER_RUNNING: bool = false;
pub static mut YIELD_PENDING: bool = false;
pub static mut NUM_OF_OVERFLOWS: BaseType = 0;
pub static mut TASK_NUMBER: UBaseType = 0;
pub static mut NEXT_TASK_UNBLOCK_TIME: TickType = 0;
pub static mut CURRENT_NUMBER_OF_TASKS: UBaseType = 0;

lazy_static! {
    // The TCB of current task
    pub static ref CURRENT_TCB: RwLock<Option<TaskHandle>> = RwLock::new(None);

    // The ready tasks list
    pub static ref READY_TASK_LISTS: [ListLink; configMAX_PRIORITIES!()] = Default::default();

    // Delayed tasks and overflow delayed tasks
    pub static ref DELAYED_TASK_LIST: ListLink = Default::default();
    pub static ref OVERFLOW_DELAYED_TASK_LIST: ListLink = Default::default();

    // Ready tasks
    pub static ref PENDING_READY_LIST: ListLink = Default::default();
}

#[cfg(feature = "INCLUDE_vTaskDelete")]
lazy_static! {
    // Tasks that have been deleted but their memory not yet freed.
    pub static ref TASKS_WAITING_TERMINATION: ListLink = Default::default();
}

#[cfg(feature = "INCLUDE_vTaskSuspend")]
lazy_static! {
    // Tasks that are currently suspended.
    pub static ref SUSPENDED_TASK_LIST: ListLink = Default::default();
}


