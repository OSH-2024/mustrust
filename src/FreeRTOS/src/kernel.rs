// kernel.rs, FreeRTOS scheduler control APIs.
use crate::list;
use crate::port::UBaseType;
use crate::projdefs::*;
use crate::tasks::{TaskHandle, TaskControlBlock};
use crate::task_global::*;
use crate::*; // TODO: Is this line necessary?
              // use crate::task_control::TaskControlBlock;

/* Definitions returned by xTaskGetSchedulerState().
 * The originial definitons are C constants, we changed them to enums.
 */
pub enum SchedulerState {
    NotStarted,
    Suspended,
    Running,
}

/// Macro for forcing a context switch.
#[macro_export]
macro_rules! taskYIELD {
    () => {
        portYIELD!()
    };
}

#[macro_export]
macro_rules! taskYIELD_IF_USING_PREEMPTION {
    () => {
        #[cfg(feature = "configUSE_PREEMPTION")]
        portYIELD_WITHIN_API!();
    };
}

/// Macro to mark the start of a critical code region.  Preemptive context
/// switches cannot occur when in a critical region.
///
/// NOTE: This may alter the stack (depending on the portable implementation)
/// so must be used with care!
#[macro_export]
macro_rules! taskENTER_CRITICAL {
    () => {
        portENTER_CRITICAL!()
    };
}

#[macro_export]
macro_rules! taskENTER_CRITICAL_FROM_ISR {
    () => {
        portSET_INTERRUPT_MASK_FROM_ISR!()
    };
}

/// Macro to mark the end of a critical code region.  Preemptive context
/// switches cannot occur when in a critical region.
///
/// NOTE: This may alter the stack (depending on the portable implementation)
/// so must be used with care!
#[macro_export]
macro_rules! taskEXIT_CRITICAL {
    () => {
        portEXIT_CRITICAL!()
    };
}

#[macro_export]
macro_rules! taskEXIT_CRITICAL_FROM_ISR {
    ($x: expr) => {
        portCLEAR_INTERRUPT_MASK_FROM_ISR!($x)
    };
}

/// Macro to disable all maskable interrupts.
#[macro_export]
macro_rules! taskDISABLE_INTERRUPTS {
    () => {
        portDISABLE_INTERRUPTS!()
    };
}

/// Macro to enable microcontroller interrupts.
#[macro_export]
macro_rules! taskENABLE_INTERRUPTS {
    () => {
        portENABLE_INTERRUPTS!()
    };
}


pub fn task_start_scheduler() {
    create_idle_task();

    #[cfg(feature = "configUSE_TIMERS")]
    create_timer_task();

    initialize_scheduler();
}

pub fn create_idle_task() -> TaskHandle {
    // println!("number: {}", get_current_number_of_tasks!());
    let idle_task_fn = || {
        loop {
            // trace!("Idle Task running");
            check_tasks_waiting_termination();

            #[cfg(not(feature = "configUSE_PREEMPTION"))]
            taskYIELD!();

            {
                #![cfg(all(feature = "configUSE_PREEMPTION", feature = "configIDLE_SHOULD_YIELD"))]
                if list::listCURRENT_LIST_LENGTH(&READY_TASK_LISTS[0]) > 1 {
                    taskYIELD!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }

            {
                #![cfg(feature = "configUSE_IDLE_HOOK")]
                // trace!("Idle Task running");
            }
        }
    };

    let mut tcb = TaskControlBlock::new();
    tcb.set_priority(0);
    tcb.set_name("Idle");
    tcb.initialize(idle_task_fn)
        .unwrap_or_else(|err| panic!("Idle task creation failed with error: {:?}", err))
}

fn check_tasks_waiting_termination() {
    // TODO: Wait for task_delete.
    // 遍历所有任务，检查状态并处理待删除的任务
    // for task in all_tasks.iter() {
    //     if task.is_waiting_for_termination() {
    //         // 执行任务删除前的清理工作
    //         task.cleanup();
    //         // 从任务列表中移除
    //         all_tasks.remove(task);
    //     }
    // }
}

/// The second (optional) part of task_start_scheduler(),
/// creates the timer task. Will panic if task creation fails.
fn create_timer_task() {
    // TODO: This function relies on the software timer, which we may not implement.
    // timer::create_timer_task()
    // On fail, panic!("No enough heap space to allocate timer task.");
    // // 创建一个定时器任务，周期性执行某些操作
    // let timer_task_fn = || loop {
    //     // 定时器任务的执行逻辑
    //     handle_timer_events();
    //     // 让出CPU，等待下一个周期
    //     taskYIELD!();
    // };

    // // 尝试创建定时器任务，设置优先级、名称等
    // let mut tcb = TaskControlBlock::new();
    // tcb.set_priority(1); // 定时器任务通常优先级较低
    // tcb.set_name("Timer");
    // tcb.initialize(timer_task_fn);
}

/// The third part of task_step_scheduler, do some initialziation
/// and call port_start_scheduler() to set up the timer tick.
fn initialize_scheduler() {
    portDISABLE_INTERRUPTS!();

    set_next_task_unblock_time!(port::portMAX_DELAY);
    set_scheduler_running!(true);
    set_tick_count!(0);

    portCONFIGURE_TIMER_FOR_RUN_TIME_STATS!();

    if port::port_start_scheduler() != pdFALSE!() {
        /* Should not reach here as if the scheduler is running the
        function will not return. */
    } else {
        // TODO: Maybe a trace here?
        /* Should only reach here if a task calls xTaskEndScheduler(). */
        //trace!("Scheduler failed to start.");
    }
}


pub fn task_end_scheduler() {
    portDISABLE_INTERRUPTS!();
    set_scheduler_running!(false);
    port::port_end_scheduler();
}


pub fn task_suspend_all() {
    set_scheduler_suspended!(get_scheduler_suspended!() + 1);
}


pub fn task_resume_all() -> bool {
    // trace!("resume_all called!");
    let mut already_yielded = false;

    // TODO: This is a recoverable error, use Result<> instead.
    assert!(
        get_scheduler_suspended!() > pdFALSE!() as UBaseType,
        "The call to task_resume_all() does not match \
         a previous call to vTaskSuspendAll()."
    );

    taskENTER_CRITICAL!();
    {
        set_scheduler_suspended!(get_scheduler_suspended!() - 1);
        /*println!(
            "get_current_number_of_tasks: {}",
            get_current_number_of_tasks!()
        );*/
        if get_scheduler_suspended!() == pdFALSE!() as UBaseType {
            if get_current_number_of_tasks!() > 0 {
                /*trace!(
                    "Current number of tasks is: {}, move tasks to ready list.",
                    get_current_number_of_tasks!()
                );*/
                if move_tasks_to_ready_list() {
                    reset_next_task_unblock_time();
                }

                process_pended_ticks();

                if get_yield_pending!() {
                    {
                        #![cfg(feature = "configUSE_PREEMPTION")]
                        already_yielded = true;
                    }

                    taskYIELD_IF_USING_PREEMPTION!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }

    // trace!("Already yielded is {}", already_yielded);
    already_yielded
}

fn move_tasks_to_ready_list() -> bool {
    let mut has_unblocked_task = false;
    while !list::listLIST_IS_EMPTY(&PENDING_READY_LIST) {
        // trace!("PEDING_LIST not empty");
        has_unblocked_task = true;
        let task_handle = list::get_owner_of_head_entry(&PENDING_READY_LIST);
        let event_list_item = task_handle.get_event_list_item();
        let state_list_item = task_handle.get_state_list_item();

        list::list_remove(state_list_item);
        list::list_remove(event_list_item);

        task_handle.add_task_to_ready_list().unwrap();

        if task_handle.get_priority() >= get_current_task_priority!() {
            set_yield_pending!(true);
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    has_unblocked_task
}

fn reset_next_task_unblock_time() {
    if list::listLIST_IS_EMPTY(&DELAYED_TASK_LIST) {
        set_next_task_unblock_time!(port::portMAX_DELAY);
    } else {
        let task_handle = list::get_owner_of_head_entry(&DELAYED_TASK_LIST);
        set_next_task_unblock_time!(list::listGET_LIST_ITEM_VALUE(
            &task_handle.get_state_list_item()
        ));
    }
}

fn process_pended_ticks() {
    // trace!("Processing pended ticks");
    let mut pended_counts = get_pended_ticks!();

    if pended_counts > 0 {
        loop {
            if task_increment_tick() {
                set_yield_pending!(true);
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }

            pended_counts -= 1;

            if pended_counts <= 0 {
                break;
            }
        }

        set_pended_ticks!(0);
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}


#[cfg(feature = "configUSE_TICKLESS_IDLE")]
pub fn task_step_tick(ticks_to_jump: TickType) {
    let cur_tick_count = get_tick_count!(); // NOTE: Is this a bug in FreeRTOS?
    let next_task_unblock_time = get_next_task_unblock_time!();

    assert!(cur_tick_count + ticks_to_jump <= next_task_unblock_time);

    set_tick_count!(cur_tick_count + ticks_to_jump);

    traceINCREASE_TICK_COUNT!(xTicksToJump);
}

pub fn task_switch_context() {
    if get_scheduler_suspended!() > pdFALSE!() as UBaseType {

        set_yield_pending!(true);
    } else {
        set_yield_pending!(false);
        traceTASK_SWITCHED_OUT!();

        #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
        generate_context_switch_stats();

        taskCHECK_FOR_STACK_OVERFLOW!();

        task_select_highest_priority_task();
        traceTASK_SWITCHED_IN!();

        // TODO: configUSE_NEWLIB_REENTRANT
    }
}


#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
fn task_select_highest_priority_task() {
    let mut top_priority: UBaseType = get_top_ready_priority!();

    while list::listLIST_IS_EMPTY(&READY_TASK_LISTS[top_priority as usize]) {
        assert!(top_priority > 0, "No task found with a non-zero priority");
        top_priority -= 1;
    }

    let next_task = list::get_owner_of_next_entry(&READY_TASK_LISTS[top_priority as usize]);

    // trace!("Next task is {}", next_task.get_name());
    set_current_task_handle!(next_task);

    set_top_ready_priority!(top_priority);
}

#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
fn generate_context_switch_stats() {
    // let total_run_time = portGET_RUN_TIME_COUNTER_VALUE!() as u32;
    let total_run_time = 0 as u32;
    // trace!("Total runtime: {}", total_run_time);
    set_total_run_time!(total_run_time);

    let task_switched_in_time = get_task_switched_in_time!();
    if total_run_time > task_switched_in_time {
        let current_task = get_current_task_handle!();
        let old_run_time = current_task.get_run_time();
        current_task.set_run_time(old_run_time + total_run_time - task_switched_in_time);
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
    set_task_switched_in_time!(total_run_time);
}

pub fn task_increment_tick() -> bool {
    // TODO: tasks.c 2500
    let mut switch_required = false;

    traceTASK_INCREMENT_TICK!(get_tick_count!());

    // trace!("SCHEDULER_SUSP is {}", get_scheduler_suspended!());
    if get_scheduler_suspended!() == pdFALSE!() as UBaseType {
        let const_tick_count = get_tick_count!() + 1;

        set_tick_count!(const_tick_count);

        if const_tick_count == 0 {
            switch_delayed_list!();
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        if const_tick_count >= get_next_task_unblock_time!() {
            // trace!("UNBLOCKING!");
            loop {
                if list::listLIST_IS_EMPTY(&DELAYED_TASK_LIST) {
                    set_next_task_unblock_time!(port::portMAX_DELAY);
                    break;
                } else {
                    let delay_head_entry_owner = list::get_owner_of_head_entry(&DELAYED_TASK_LIST);
                    let task_handle = delay_head_entry_owner;
                    let state_list_item = task_handle.get_state_list_item();
                    let event_list_item = task_handle.get_event_list_item();
                    let item_value = list::listGET_LIST_ITEM_VALUE(&state_list_item);

                    if const_tick_count < item_value {
                        set_next_task_unblock_time!(item_value);
                        break;
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }

                    list::list_remove(state_list_item.clone());
                    if list::get_list_item_container(&event_list_item).is_some() {
                        list::list_remove(event_list_item.clone());
                    }

                    task_handle.add_task_to_ready_list().unwrap();
                    {
                        #![cfg(feature = "configUSE_PREEMPTION")]

                        if task_handle.get_priority() >= get_current_task_priority!() {
                            switch_required = true;
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                }
            }
        }

        {
            #![cfg(all(feature = "configUSE_PREEMPTION", feature = "configUSE_TIME_SLICING"))]
            let cur_task_pri = get_current_task_priority!();

            if list::listCURRENT_LIST_LENGTH(&READY_TASK_LISTS[cur_task_pri as usize]) > 1 {
                switch_required = true;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }

        {
            #![cfg(feature = "configUSE_TICK_HOOK")]
            if get_pended_ticks!() == 0 {
                // vApplicationTickHook();
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    } else {
        set_pended_ticks!(get_pended_ticks!() + 1);

        #[cfg(feature = "configUSE_TICK_HOOK")]
        vApplicationTickHook();

        #[cfg(feature = "configUSE_PREEMPTION")]
        {
            if get_yield_pending!() {
                switch_required = true;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    }
    switch_required
}

#[cfg(any(
    feature = "INCLUDE_xTaskGetSchedulerState",
    feature = "configUSE_TIMERS"
))]
pub fn task_get_scheduler_state() -> SchedulerState {

    if !get_scheduler_running!() {
        SchedulerState::NotStarted
    } else {
        if get_scheduler_suspended!() == pdFALSE!() as UBaseType {
            SchedulerState::Running
        } else {
            SchedulerState::Suspended
        }
    }
}


#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
#[macro_export]
macro_rules! taskRESET_READY_PRIORITY {
    ($uxPriority: expr) => {};
}

#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
#[macro_export]
macro_rules! taskRECORD_READY_PRIORITY {
    ($uxPriority: expr) => {
        if $uxPriority > get_top_ready_priority!() {
            set_top_ready_priority!($uxPriority);
        }
    };
}