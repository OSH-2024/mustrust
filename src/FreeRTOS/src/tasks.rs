use crate::list::*;
use alloc::sync::{Arc, Weak};
use core::ops::FnOnce;
use core::mem;
use crate::port;
use cty;
use no_std_async::rwlock::RwLock;

pub type TaskHandleType = *mut cty::c_void;

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

#[cfg(feature = "configUSE_16_BIT_TICKS")]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x8000 as u16 } }
#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
#[macro_export]
macro_rules! taskEVENT_LIST_ITEM_VALUE_IN_USE { () => { 0x80000000 as u32 } }

pub struct TaskControlBlock {
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

impl PartialEq for TaskControlBlock {
    fn eq(&self, other: &Self) -> bool {
        self.stack == other.stack
    }
}

impl TaskControlBlock {
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

    pub fn get_base_priority(&self) -> UBaseType_t {
        self.base_priority
    }

    pub fn set_base_priority(mut self, priority: UBaseType_t) -> Self {
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
        self.runtime_counter = next_val
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
    pub fn get_mutex_held_count(&self) -> UBaseType_t {
        self.mutexed_held
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&mut self, new_count: UBaseType_t) {
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
        self.stack = px_stack as *mut StackType;
        let mut top_of_stack = self.stack + self.stack_length - 1;
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
        
        let stack_ptr = self.stack;
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
pub fn record_ready_priority(priority: UBaseType_t) {
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
    pub fn get_mutex_held_count(&self) -> UBaseType_t {
        GetTaskControlBlockRead!(self).get_mutex_held_count()
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&self, new_count: UBaseType_t) {
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

    pub fn get_priority(&self) -> UBaseType_t {
        GetTaskControlBlockRead!(self).get_priority()
    }

    pub fn set_priority(&self, priority: UBaseType_t) {
        GetTaskControlBlockWrite!(self).set_priority(priority)
    }
}
