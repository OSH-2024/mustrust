use crate::list;
use crate::list::ListLink;
use crate::port::*;
use crate::kernel::*;
use crate::projdefs::pdFALSE;
use crate::tasks::*;
use crate::global::*;
use crate::*;
use core::default::*;

#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
pub const taskEVENT_LIST_ITEM_VALUE_IN_USE: TickType = 0x80000000;

pub fn task_remove_from_event_list(event_list: &ListLink) -> bool {
    let unblocked_tcb = list::get_owner_of_head_entry(event_list);
    let mut ret: bool = false;

    list::list_remove(unblocked_tcb.get_event_list_item());

    if get_scheduler_suspended!() == pdFALSE!() {
        list::list_remove(unblocked_tcb.get_state_list_item());
        unblocked_tcb.add_task_to_ready_list().Unwrap();
    }
    else {
        list::list_insert_end(&PENDING_READY_LIST, unblocked_tcb.get_event_list_item());
    }

    if unblocked_tcb.get_priority() < get_current_task_priority!() {
        ret = true;
        set_yield_pending!(true);
    }
    else {
        ret = false;
    }
    
    {
        #[cfg(feature = "configUSE_TICKLESS_IDLE")]
        reset_next_task_unblock_time();
    }

    ret
}

pub fn task_missed_yield() {
    set_yield_pending!(false);
}

#[derive(Default)]
pub struct time_out {
    overflow_count: BaseType,
    time_on_entering: TickType,
}

pub fn task_set_timeout_state(timeout: &mut time_out) {
    timeout.overflow_count = get_number_of_overflows!();
    timeout.time_on_entering = get_tick_count!();
}

pub fn task_check_for_timeout(timeout: &mut time_out, ticks_to_wait: &mut TickType) -> bool {
    let mut ret: bool = false;
    taskENTER_CRITICAL!();
    {
        let const_tick_count: TickType = get_tick_count!();
        let unwrapped_cur = get_current_task_handle!();
        let mut cfglock1 = false;
        let mut cfglock2 = false;

        {
            #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
            cfglock1 = true;
        }
        {
            #[cfg(feature = "INCLUDE_vTaskSuspend")]
            cfglock2 = true;
        }

        if cfglock1 && unwrapped_cur.get_delay_aborted() {
            unwrapped_cur.set_delay_aborted(false);
            ret = true;
        }
        if cfglock2 && *ticks_to_wait == portMAX_DELAY!() {
            ret = false;
        }

        if get_number_of_overflows!() != timeout.overflow_count
            && const_tick_count >= timeout.time_on_entering
        {
            ret = true;
        }
        else if const_tick_count - timeout.time_on_entering < *ticks_to_wait {
            *ticks_to_wait -= const_tick_count - timeout.time_on_entering;
            task_set_timeout_state(timeout);
            ret = false;
        }
        else {
            ret = true;
        }
    }
    taskEXIT_CRITICAL!();
    ret
}

pub fn task_place_on_event_list(event_list: &ListLink, ticks_to_wait: TickType) {
    let unwrapped_cur = get_current_task_handle!();
    list::list_insert(event_list, unwrapped_cur.get_event_list_item());
    add_current_task_to_delayed_list(ticks_to_wait, true);
}

#[cfg(feature = "configUSE_MUTEXES")]
pub fn task_increment_mutex_held_count() -> Option<TaskHandle> {
    match get_current_task_handle_wrapped!() {
        Some(task) => {
            let new_val = task.get_mutex_held_count() + 1;
            task.set_mutex_held_count(new_val);
            Some(task.clone())
        }
        None => None,
    }
}

