use crate::list::ListLink;
use crate::port::{BaseType, TickType, UBaseType};
use crate::tasks::*;
use crate::*;
use synctools::rwlock::*;
use lazy_static::*;

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

// Context switches are held pending while the scheduler is suspended.  Also,
// interrupts must not manipulate the xStateListItem of a TCB, or any of the
// lists the xStateListItem can be referenced from, if the scheduler is suspended.
pub static mut SCHEDULER_SUSPENDED: UBaseType = 0;

// The value of a timer / counter the last time a task was switched in.
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
pub static mut TASK_SWITCHED_IN_TIME: u32 = 0;

// The total amount of execution time as defined by the run time counter clock.
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
pub static mut TOTAL_RUN_TIME: u32 = 0;

// The amount of deleted tasks waiting for cleaning up
#[cfg(feature = "INCLUDE_vTaskDelete")]
pub static mut DELETED_TASKS_WAITING_CLEAN_UP: UBaseType = 0;

/*
 * Global operations of the values above
 */

#[macro_export]
macro_rules! set_scheduler_suspended {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::SCHEDULER_SUSPENDED = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_scheduler_suspended {
    () => {
        unsafe {
            crate::task_global::SCHEDULER_SUSPENDED
        }
    };
}

#[macro_export]
macro_rules! set_deleted_tasks_waiting_clean_up {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::DELETED_TASKS_WAITING_CLEAN_UP = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_deleted_tasks_waiting_clean_up {
    () => {
        unsafe {
            crate::task_global::DELETED_TASKS_WAITING_CLEAN_UP
        }
    };
}

#[macro_export]
macro_rules! set_top_ready_priority {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::TOP_READY_PRIORITY = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_top_ready_priority {
    () => {
        unsafe {
            crate::task_global::TOP_READY_PRIORITY
        }
    };
}

#[macro_export]
macro_rules! set_pended_ticks {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::PENDED_TICKS = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_pended_ticks {
    () => {
        unsafe {
            crate::task_global::PENDED_TICKS
        }
    };
}

#[macro_export]
macro_rules! set_task_number {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::TASK_NUMBER = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_task_number {
    () => {
        unsafe {
            crate::task_global::TASK_NUMBER
        }
    };
}

#[macro_export]
macro_rules! set_yield_pending {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::YIELD_PENDING = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_yield_pending {
    () => {
        unsafe {
            crate::task_global::YIELD_PENDING
        }
    };
}

#[macro_export]
macro_rules! set_current_number_of_tasks {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::CURRENT_NUMBER_OF_TASKS = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_current_number_of_tasks {
    () => {
        unsafe {
            crate::task_global::CURRENT_NUMBER_OF_TASKS
        }
    };
}

#[macro_export]
macro_rules! set_scheduler_running {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::SCHEDULER_RUNNING = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_scheduler_running {
    () => {
        unsafe {
            crate::task_global::SCHEDULER_RUNNING
        }
    };
}

#[macro_export]
macro_rules! set_next_task_unblock_time {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::NEXT_TASK_UNBLOCK_TIME = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_next_task_unblock_time {
    () => {
        unsafe {
            crate::task_global::NEXT_TASK_UNBLOCK_TIME
        }
    };
}

#[macro_export]
macro_rules! set_tick_count {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::TICK_COUNT = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_tick_count {
    () => {
        unsafe {
            crate::task_global::TICK_COUNT
        }
    };
}

#[macro_export]
macro_rules! set_num_of_overflows {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::NUM_OF_OVERFLOWS = $next_val;
        }
    };
}

#[macro_export]
macro_rules! get_num_of_overflows {
    () => {
        unsafe {
            crate::task_global::NUM_OF_OVERFLOWS
        }
    };
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! set_total_run_time {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::TOTAL_RUN_TIME = $next_val;
        }
    };
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! get_total_run_time {
    () => {
        unsafe {
            crate::task_global::TOTAL_RUN_TIME
        }
    };
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! set_task_switched_in_time {
    ($next_val: expr) => {
        unsafe {
            crate::task_global::TASK_SWITCHED_IN_TIME = $next_val;
        }
    };
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! get_task_switched_in_time {
    () => {
        unsafe {
            crate::task_global::TASK_SWITCHED_IN_TIME
        }
    };
}

#[macro_export]
macro_rules! get_current_task_handle_wrapped {
    () => {
        crate::task_global::CURRENT_TCB.read().as_ref()
    };
}

#[macro_export]
macro_rules! get_current_task_handle {
    () => {
        *crate::task_global::CURRENT_TCB.read().as_ref().clone().expect("No current TCB")
    };
}

#[macro_export]
macro_rules! set_current_task_handle {
    ($cloned_new_task: expr) => {
        // info!("CURRENT_TCB changed!");
        *(crate::task_global::CURRENT_TCB).write() = Some($cloned_new_task)
    };
}

#[macro_export]
macro_rules! get_current_task_priority {
    () => {
        get_current_task_handle!().get_priority()
    };
}

#[cfg(feature = "INCLUDE_xTaskAbortDelay")]
#[macro_export]
macro_rules! get_current_task_delay_aborted {
    () => {
        get_current_task_handle!().get_delay_aborted()
    };
}

#[macro_export]
macro_rules! taskCHECK_FOR_STACK_OVERFLOW {
    () => {
    };
}

#[macro_export]
macro_rules! switch_delayed_list {
    () => {
        unsafe {
            let mut delayed = *DELAYED_TASK_LIST.write();
            let mut overflowed = *OVERFLOW_DELAYED_TASK_LIST.write();
            let tmp = (*delayed).clone();
            *delayed = (*overflowed).clone();
            *overflowed = tmp;
        }
    };
}
