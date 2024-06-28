use crate::list::*;

#[cfg(configUSE_PREEMPTION!() = 0)]
macro_rules! taskYIELD_IF_USING_PREEMPTION { () => { } }

#[cfg(configUSE_PREEMPTION!() = 1)]
macro_rules! taskYIELD_IF_USING_PREEMPTION { () => { portYIELD_WITHIN_API!() } }

#[macro_export]
macro_rules! taskNOT_WAITING_NOTIFICATION { () => { 0 as u8 } }
#[macro_export]
macro_rules! taskWAITING_NOTIFICATION { () => { 1 as u8 } }
#[macro_export]
macro_rules! taskNOTIFICATION_RECEIVED { () => { 2 as u8 } }

/*
 * The value used to fill the stack of a task when the task is created.  This
 * is used purely for checking the high water mark for tasks.
 */
#[macro_export]
macro_rules! tskSTACK_FILL_BYTE	{ () => { 0xa5 } }

#[macro_export]
macro_rules! tskDYNAMICALLY_ALLOCATED_STACK_AND_TCB { () => { 0 as u8 } }
#[macro_export]
macro_rules! tskSTATICALLY_ALLOCATED_STACK_ONLY { () => { 1 as u8 } }
#[macro_export]
macro_rules! tskSTATICALLY_ALLOCATED_STACK_AND_TCB { () => { 2 as u8 } }

#[macro_export]
macro_rules! tskRUNNING_CHAR { () => { 'X' } }
#[macro_export]
macro_rules! tskBLOCKED_CHAR { () => { 'B' } }
#[macro_export]
macro_rules! tskREADY_CHAR { () => { 'R' } }
#[macro_export]
macro_rules! tskDELETED_CHAR { () => { 'D' } }
#[macro_export]
macro_rules! tskSUSPENDED_CHAR { () => { 'S' } }

#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
#[macro_export]
macro_rules! taskRECORD_READY_PRIORITY {
    ($uxPriority: expr) => {
        if $uxPriority > get_top_ready_priority!() {
            set_top_ready_priority!($uxPriority);
        }
    };
}

#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
fn taskSelectHighestPriorityTask() {
    let mut uxTopPriority: UBaseType_t = uxTopReadyPriority;

    /* Find the highest priority queue that contains ready tasks. */
    while list::list_is_empty(&READY_TASK_LISTS[uxTopPriority as usize]) {
        assert!(uxTopPriority > 0, "No non-zero priority task found.");
        uxTopPriority -= 1;
    }

    /* listGET_OWNER_OF_NEXT_ENTRY indexes through the list, so the tasks of
    the same priority get an equal share of the processor time. */
    pxCurrentTCB = list::get_owner_of_next_entry(&pxReadyTasksLists[uxTopPriority as usize]);
    uxTopReadyPriority = uxTopPriority;
}

#[cfg(not(feature = "configUSE_PORT_OPTIMISED_TASK_SELECTION"))]
#[macro_export]
macro_rules! taskRESET_READY_PRIORITY {
    ($uxPriority: expr) => {};
}

/* pxDelayedTaskList and pxOverflowDelayedTaskList are switched when the tick
count overflows. */
#[macro_export]
macro_rules! taskSWITCH_DELAYED_LISTS {
    () => {
        unsafe {
            let mut delayed_task_list = pxDelayedTaskList.write().unwrap();
            let mut overflowedTaskList = pxOverflowDelayedTaskList.write().unwrap();
            let temp = (*delayedTaskList).clone();
            *delayedTaskList = (*overflowedTaskList).clone();
            *overflowedTaskList = temp;
        }
    };
}

#[cfg(feature = "configUSE_16_BIT_TICKS")]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x8000 as u16 } }
#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x80000000 as u32 } }

pub struct task_control_block {
    state_list_item: ListItem,
    event_list_item: ListItem,
    priority: UBaseType_t,
    task_name: String
    stack: StackType_t,
    stack_length: UBaseType_t,

    #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
    critical_nesting: UBaseType_t,
    #[cfg(feature = "configUSE_MUTEXES")]
    base_priority: UBaseType_t,
    #[cfg(feature = "configUSE_MUTEXES")]
    mutexed_held: UBaseType_t,
    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    runtime_counter: TickType_t,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notified_value: u32,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    nofity_state: u8,
    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    delay_aborted: bool,
}

impl PartialEq for task_control_block {
    fn eq(&self, other: &Self) -> bool {
        self.stack == other.stack
    }
}

impl task_control_block {
    pub fn new() -> Self {
        Self {
            state_list_item: ListItem::default(),
            event_list_item: ListItem::default(),
            priority: 1,
            task_name: String::from("New task"),
            stack: 0,
            stack_length: configMINIMAL_STACK_SIZE!(),
            #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
            critical_nesting: 0,
            #[cfg(feature = "configUSE_MUTEXES")]
            base_priority: 0,
            #[cfg(feature = "configUSE_MUTEXES")]
            mutexed_held: 0,
            #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
            runtime_counter: 0,
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            notified_value: 0,
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            nofity_state: 0,
            #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
            delay_aborted: false,
        }
    }

    pub fn get_name(&self) -> &str {
        self.task_name.as_str()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        self.task_name = String::from(name);
        self
    }

    pub fn set_stack_length(&mut self, length: UBaseType_t) -> Self {
        self.stack_length = length;
        self
    }

    pub fn get_priority(&self) -> UBaseType_t {
        self.priority.clone()
    }

    pub fn set_priority(mut self, priority: UBaseType_t) -> Self {
        if priority > configMAX_PRIORITIES!() {
            // Warning: Priority exceeded
            self.priority = configMAX_PRIORITIES!() - 1;
        }
        else {
            self.priority = priority;
        }
        self
    }
}
