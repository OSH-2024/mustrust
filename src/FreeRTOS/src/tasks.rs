use crate::*;
use crate::list::*;
use alloc::sync::{Arc, Weak};
use core::ops::FnOnce;
use core::mem;
use crate::port;
use crate::task_global::*;
use cty;
use no_std_async::RwLock;

pub type TaskHandleType = *mut cty::c_void;

#[cfg(feature = "configUSE_PREEMPTION")]
macro_rules! taskYIELD_IF_USING_PREEMPTION { () => { } }

#[cfg(not(feature = "configUSE_PREEMPTION"))]
macro_rules! taskYIELD_IF_USING_PREEMPTION { () => { portYIELD_WITHIN_API!() } }

#[macro_export]
macro_rules! taskENTER_CRITICAL { () => { portENTER_CRITICAL!() } }
#[macro_export]
macro_rules! taskEXIT_CRITICAL { () => { portEXIT_CRITICAL!() } }

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
    let mut uxTopPriority: UBaseType = uxTopReadyPriority;

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

#[cfg(feature = "configUSE_16_BIT_TICKS")]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x8000 as u16 } }
#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x80000000 as u32 } }

pub struct TaskControlBlock {
    state_list_item: ListItem,
    event_list_item: ListItem,
    priority: UBaseType,
    task_name: String,
    stack_pointer: StackType_t,
    stack_length: UBaseType,

    #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
    critical_nesting: UBaseType,
    #[cfg(feature = "configUSE_MUTEXES")]
    base_priority: UBaseType,
    #[cfg(feature = "configUSE_MUTEXES")]
    mutexed_held: UBaseType,
    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    runtime_counter: TickType_t,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notified_value: u32,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    nofity_state: u8,
    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    delay_aborted: bool,
}

impl PartialEq for TaskControlBlock {
    fn eq(&self, other: &Self) -> bool {
        self.stack_pointer == other.stack_pointer
    }
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            state_list_item: ListItem::default(),
            event_list_item: ListItem::default(),
            priority: 1,
            task_name: String::from("New task"),
            stack_pointer: 0,
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

    pub fn set_stack_length(&mut self, length: UBaseType) -> Self {
        self.stack_length = length;
        self
    }

    pub fn get_priority(&self) -> UBaseType {
        self.priority.clone()
    }

    pub fn set_priority(mut self, priority: UBaseType) -> Self {
        if priority > configMAX_PRIORITIES!() {
            // Warning: Priority exceeded
            self.priority = configMAX_PRIORITIES!() - 1;
        }
        else {
            self.priority = priority;
        }
        self
    }

    pub fn get_base_priority(&self) -> UBaseType {
        self.base_priority
    }

    pub fn set_base_priority(mut self, priority: UBaseType) -> Self {
        if priority > configMAX_PRIORITIES!() {
            // Warning: Priority exceeded
            self.base_priority = configMAX_PRIORITIES!() - 1;
        }
        else {
            self.base_priority = priority;
        }
        self
    }

    pub fn get_stack_list_item(&self) -> ListItem {
        self.state_list_item.clone()
    }

    pub fn get_event_list_item(&self) -> ListItem {
        self.event_list_item.clone()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn get_runtime(&self) -> TickType_t {
        self.runtime_counter
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn set_runtime(&mut self, next_val: TickType_t) -> TickType_t {
        let prev_val = self.runtime_counter;
        self.runtime_counter = next_val;
        prev_val
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn get_delay_aborted(&self) -> bool {
        self.delay_aborted
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted(&mut self, next_val: bool) -> bool {
        let prev_val = self.delay_aborted;
        self.delay_aborted = next_val;
        prev_val
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_mutex_held_count(&self) -> UBaseType {
        self.mutexed_held
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&mut self, new_count: UBaseType) {
        self.mutexed_held = new_count;
    }

    pub fn initialize<F>(mut self, func: F) -> Result<TaskHandle, FreeRtosError>
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let size_of_stacktype = core::mem::size_of::<StackType>();
        let stacksize_as_bytes = size_of_stacktype * self.stack_length as usize;
        // Initialize stack
        let px_stack = port::port_malloc(stacksize_as_bytes)?;
        self.stack_pointer = px_stack as *mut StackType;
        let mut top_of_stack = self.stack_pointer + self.stack_length - 1;
        top_of_stack = top_of_stack & portBYTE_ALIGNMENT_MASK as *mut StackType;
        // Initialize task
        let f = Box::new(Box::new(func) as Box<dyn FnOnce()>);
        let param_ptr = &*f as *const _ as *mut _;
        let result = port::port_initialise_stack(top_of_stack as *mut _, 32, param_ptr);
        match result {
            Ok(_) => core::mem::forget(f),
            Err(e) => return Err(e),
        }

        #[cfg(feature = "configUSE_MUTEXES")]
        {
            self.mutexed_held = 0;
            self.base_priority = self.priority;
        }

        #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
        {
            self.critical_nesting = 0;
        }

        #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
        {
            self.runtime_counter = 0;
        }
        
        let stack_ptr = self.stack_pointer;
        let handle = TaskHandle(Arc::new(RwLock::new(self)));
        list::set_list_item_owner(&handle.get_state_list_item(), handle.clone());
        list::set_list_item_owner(&handle.get_event_list_item(), handle.clone());
        list::set_list_item_owner(&handle.get_stack_list_item(), handle.clone());
        let item_value = (configMAX_PRIORITIES!() - handle.get_priority()) as TickType;
        list::listSET_LIST_ITEM_VALUE(&handle.get_stack_list_item(), item_value);
        Ok(handle)
    }
}

#[derive(Clone)]
pub struct TaskHandle(Arc<RwLock<TaskControlBlock>>);

impl PartialEq for TaskHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.0.read().unwrap() == *other.0.read().unwrap()
    }
}

impl From<Weak<RwLock<TaskControlBlock>>> for TaskHandle {
    fn from(weak_link: Weak<RwLock<TaskControlBlock>>) -> Self {
        TaskHandle(
            weak_link
                .upgrade()
                .unwrap_or_else(|| panic!("Owner is not set")),
        )
    }
}

impl From<TaskHandle> for Weak<RwLock<TaskControlBlock>> {
    fn from(task: TaskHandle) -> Self {
        Arc::downgrade(&task.0)
    }
}

#[macro_use]
pub fn record_ready_priority(priority: UBaseType) {
    if priority > get_top_ready_priority!() {
        set_top_ready_priority!(priority);
    }
}

#[macro_export]
macro_rules! GetTaskControlBlockRead {
    ($handle: expr) => {
        match $handle.0.try_read() {
            Ok(x) => x,
            Err(_) => panic!("Task handle locked"),
        }
    }
}

#[macro_export]
macro_rules! GetTaskControlBlockWrite {
    ($handle: expr) => {
        match $handle.0.try_write() {
            Ok(x) => x,
            Err(_) => panic!("Task handle locked"),
        }
    }
}

impl TaskHandle {
    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn get_runtime(&self) -> TickType_t {
        GetTaskControlBlockRead!(self).get_runtime()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn set_runtime(&self, next_val: TickType_t) -> TickType_t {
        GetTaskControlBlockWrite!(self).set_runtime(next_val)
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn get_delay_aborted(&self) -> bool {
        GetTaskControlBlockRead!(self).get_delay_aborted()
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted(&self, next_val: bool) -> bool {
        GetTaskControlBlockWrite!(self).set_delay_aborted(next_val)
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_mutex_held_count(&self) -> UBaseType {
        GetTaskControlBlockRead!(self).get_mutex_held_count()
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&self, new_count: UBaseType) {
        GetTaskControlBlockWrite!(self).set_mutex_held_count(new_count)
    }

    pub fn from_arc(arc: Arc<RwLock<TaskControlBlock>>) -> Self {
        TaskHandle(arc)
    }

    pub fn from(tcb: TaskControlBlock) -> Self {
        TaskHandle(Arc::new(RwLock::new(tcb)))
    }

    pub fn as_raw(self) -> TaskHandleType {
        Arc::into_raw(self.0) as TaskHandleType
    }

    pub fn get_event_list_item(&self) -> ListItem {
        GetTaskControlBlockRead!(self).get_event_list_item()
    }

    pub fn get_state_list_item(&self) -> ListItem {
        GetTaskControlBlockRead!(self).get_state_list_item()
    }

    pub fn get_name(&self) -> String {
        GetTaskControlBlockRead!(self).get_name()
    }

    pub fn get_priority(&self) -> UBaseType {
        GetTaskControlBlockRead!(self).get_priority()
    }

    pub fn set_priority(&self, priority: UBaseType) {
        GetTaskControlBlockWrite!(self).set_priority(priority)
    }

    pub fn set_priority_in_detail(&mut self, priority: UBaseType) {
        let mut priority = priority;
        let mut yielding = false;
        let mut current_base_priority: UBaseType = 0;
        let mut priority_used_on_entry: UBaseType = 0;

        if priority > configMAX_PRIORITIES!() as UBaseType {
            // Warning: Priority exceeded
            priority = (configMAX_PRIORITIES!() - 1) as UBaseType;
        }

        taskENTER_CRITICAL!();
        {
            let tcb = GetTaskControlBlockWrite!(self);
            let px_tcb: &TaskControlBlock = &tcb;
            traceTASK_PRIORITY_SET!(px_tcb, priority);

            {
                #![cfg(feature = "configUSE_MUTEXES")]
                current_base_priority = px_tcb.get_base_priority();
            }

            {
                #![cfg(not(feature = "configUSE_MUTEXES"))]
                current_base_priority = px_tcb.get_priority();
            }

            if current_base_priority != priority {
                if self != &mut get_current_task_handle!() {
                    if priority > get_current_task_priority!() {
                        yielding = true;
                    }
                }
            }
            else if self == &mut get_current_task_handle!() {
                yielding = true;
            }

            {
                #![cfg(feature = "configUSE_MUTEXES")]
                if px_tcb.get_base_priority() == px_tcb.get_priority() {
                    px_tcb.set_priority(priority);
                }
                px_tcb.set_base_priority(priority);
            }

            let event_list_item = px_tcb.get_event_list_item();
            let state_list_item = px_tcb.get_state_list_item();

            unsafe {
                if (list::listGET_LIST_ITEM_VALUE(&event_list_item) & taskEVENT_LIST_ITEM_VALUE_IN_USE) == 0 {
                    list::listSET_LIST_ITEM_VALUE(&event_list_item, (configMAX_PRIORITIES!() - priority) as TickType);
                }
            }

            if list::is_contained_within(&READY_TASK_LISTS[priority_used_on_entry as usize], &state_list_item) {
                if list::list_remove(state_list_item) == 0 {
                    portRESET_READY_PRIORITY!(priority_used_on_entry, get_top_ready_priority!());
                }
                self.add_task_to_ready_list();
            }

            if yielding {
                taskYIELD_IF_USING_PREEMPTION!();
            }
        }
        taskEXIT_CRITICAL!();
    }

    pub fn get_base_priority(&self) -> UBaseType {
        GetTaskControlBlockRead!(self).get_base_priority()
    }

    pub fn set_base_priority(&self, priority: UBaseType) {
        GetTaskControlBlockWrite!(self).set_base_priority(priority)
    }

    pub fn add_task_to_ready_list(&self) -> Result<(), FreeRtosError> {
        let tcb = GetTaskControlBlockWrite!(self);
        let priority = self.get_priority();
        traceMOVED_TASK_TO_READY_STATE!(&tcb);
        record_ready_priority(priority);
        // Insert to list
        list::list_insert_end(&READY_TASK_LISTS[priority as usize], &Arc::clone(&tcb.get_state_list_item()));
        tracePOST_MOVED_TASK_TO_READY_STATE!(&tcb);
        Ok(())
    }

    fn add_new_task_to_ready_list(&self) -> Result<(), FreeRtosError> {
        let tcb = GetTaskControlBlockWrite!(self);
        taskENTER_CRITICAL!();
        {
            let current_number_of_tasks = get_current_number_of_tasks!() + 1;
            set_current_number_of_tasks!(current_number_of_tasks);
            if task_global::CURRENT_TCB.read().unwrap().is_none() {
                set_current_task_handle!(self.clone());
            }
            else {
                let task_handle = get_current_task_handle!();
                if !get_scheduler_running!() {
                    if task_handle.get_priority() < tcb.get_priority() {
                        set_current_task_handle!(self.clone());
                    }
                }
            }
            set_task_number!(get_task_number!() + 1);
            traceTASK_CREATE!(self.clone());
            self.add_task_to_ready_list();
        }
        taskEXIT_CRITICAL!();
        if get_scheduler_running!() {
            let current_priority = get_current_task_priority!();
            if current_priority < tcb.priority {
                taskYIELD_IF_USING_PREEMPTION!();
            }
        }
        Ok(())
    }
}

pub fn add_current_task_to_delayed_list(ticks_to_delay: TickType, can_block_indefinitely: bool) {
    let current_task_handle = get_current_task_handle!();
    {
        #![cfg(feature = "INCLUDE_xTaskAbortDelay")]
        current_task_handle.set_delay_aborted(false);
    }

    if list::list_remove(current_task_handle.get_event_list_item()) == 0 {
        // Removed from ready list
        // Reset the highest priority of the ready list
        portRESET_READY_PRIORITY!(current_task_handle.get_priority(), get_top_ready_priority!());
    }

    {
        // INCLUDE_vTaskSuspend is defined
        #![cfg(feature = "INCLUDE_vTaskSuspend")]
        if ticks_to_delay == port::portMAX_DELAY && can_block_indefinitely {
            // Add the task to suspend list instead of delayed list
            let current_state_list_item = current_task_handle.get_state_list_item();
            list::list_insert_end(&SUSPENDED_TASK_LIST, &current_state_list_item);
        }
        else {
            // Calculate when the task will be resumed and insert into delayed list
            let time = get_tick_count!() + ticks_to_delay;
            let current_state_list_item = current_task_handle.get_state_list_item();
            list::listSET_LIST_ITEM_VALUE(&current_state_list_item, time);

            if time < get_tick_count!() {
                // Add the task to overflow delayed list
                list::vListInsert(&OVERFLOW_DELAYED_TASK_LIST, &current_state_list_item);
            }
            else {
                // Add the task to delayed list
                list::vListInsert(&DELAYED_TASK_LIST, &current_state_list_item);

                // Next task unblock time should be updated
                if time < get_next_task_unblock_time!() {
                    set_next_task_unblock_time!(time);
                }
            }
        }
    }

    {
        // INCLUDE_vTaskSuspend is not defined
        #![cfg(not(feature = "INCLUDE_vTaskSuspend"))]
        let time = get_tick_count!() + ticks_to_delay;
        let current_state_list_item = current_task_handle.get_state_list_item();
        list::listSET_LIST_ITEM_VALUE(&current_state_list_item, time);

        if time < get_tick_count!() {
            // Add the task to overflow delayed list
            list::vListInsert(&OVERFLOW_DELAYED_TASK_LIST, &current_state_list_item);
        }
        else {
            // Add the task to delayed list
            list::vListInsert(&DELAYED_TASK_LIST, &current_state_list_item);

            // Next task unblock time should be updated
            if time < get_next_task_unblock_time!() {
                set_next_task_unblock_time!(time);
            }
        }
    }
}

pub fn reset_next_task_unblock_time() {
    if list::list_is_empty(&DELAYED_TASK_LIST) {
        // No tasks were blocked, so the next unblock time is set to portMAX_DELAY
        set_next_task_unblock_time!(port::portMAX_DELAY);
    }
    else {
        // Get the handle of the first entry in the delayed task list
        let mut temp = get_owner_of_head_entry(&DELAYED_TASK_LIST);
        set_next_task_unblock_time!(list::listGET_LIST_ITEM_VALUE(&temp.get_state_list_item()));
    }
}

#[cfg(feature = "INCLUDE_vTaskDelete")]
pub fn task_delete(task_to_delete: Option<TaskHandle>) {
    let tcb = GetTaskControlBlockWrite!(task_to_delete);
    
    taskENTER_CRITICAL!();
    {
        if list::list_remove(tcb.get_state_list_item()) == 0 {
            // Removed from ready list
            // Reset the highest priority of the ready list
            portRESET_READY_PRIORITY!(tcb.get_priority(), get_priority!());
        }

        if list::get_list_item_container(tcb.get_event_list_item()).is_some() {
            // Reset the event list item
            list::list_remove(tcb.get_event_list_item());
        }

        set_task_number!(get_task_number!() + 1);

        if tcb == get_current_task_handle!() {
            // If the task being deleted is the currently running task then
            // insert it end of the waiting termination list
            list::list_insert_end(&TASKS_WAITING_TERMINATION, tcb.get_state_list_item());

            // Add the number of deleted tasks waiting to be cleaned up
            set_deleted_tasks_waiting_clean_up!(get_deleted_tasks_waiting_clean_up!() + 1);

            portPRE_TASK_DELETE_HOOK!(tcb, get_yield_pending!());
        }
        else {
            // Decrease the number of tasks
            set_current_number_of_tasks!(get_current_number_of_tasks!() - 1);

            // Release the task's memory
            let stack_pointer = GetTaskControlBlockRead!(tcb).stack_pointer;
            port::port_free(stack_pointer as *mut _);

            // Reset the next unblock time
            reset_next_task_unblock_time();
        }
    }
    taskEXIT_CRITICAL!();
}

#[cfg(feature = "INCLUDE_vTaskSuspend")]
pub fn is_task_suspended(task: &TaskHandle) -> bool {
    let mut result = false;
    let tcb = GetTaskControlBlockRead!(task);
    
    // Check if the task is in the suspended list
    if list::is_contained_within(&SUSPENDED_TASK_LIST, &tcb.get_state_list_item()) {
        // The task is in the pending ready list
        if !list::is_contained_within(&PENDING_READY_LIST, &tcb.get_event_list_item()) {
            // The task is not waiting for an event
            if list::get_list_item_container(tcb.get_event_list_item()).is_some() {
                result = true;
            }
        }
    }
    result
}

#[cfg(feature = "INCLUDE_vTaskSuspend")]
pub fn resume_task(task: TaskHandle) {
    let mut tcb = GetTaskControlBlockRead!(task);
    
    if task != get_current_task_handle!() {
        taskENTER_CRITICAL!();
        {
            if is_task_suspended(&task) {
                traceTASK_RESUME!(&tcb);

                list::list_remove(tcb.get_state_list_item());
                task.add_task_to_ready_list();

                let current_priority = get_current_task_handle!().get_priority();
                if current_priority <= tcb.get_priority() {
                    // Yield if the new priority is higher
                    taskYIELD_IF_USING_PREEMPTION!();
                }
            }
        }
        taskEXIT_CRITICAL!();
    }
}
