// queue_api.rs, queue APIs
// This file is created by Ning Yuting.
// To solve the issue of mutability of queue.

use crate::port::*;
use crate::queue::*;
use crate::queue_h::*;
use core::cell::UnsafeCell;

pub struct Queue<T>(UnsafeCell<QueueDefinition<T>>)
where
    T: Default + Clone;

// send, sync is used for sharing queue among threads
unsafe impl<T: Default + Clone> Send for Queue<T> {}
unsafe impl<T: Default + Clone> Sync for Queue<T> {}

impl<T> Queue<T>
where
    T: Default + Clone,
{
    pub fn new(length: UBaseType) -> Self {
        Queue(UnsafeCell::new(QueueDefinition::new(
            length,
            QueueType::Base,
        )))
    }

    pub fn send(&self, pvItemToQueue: T, xTicksToWait: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
        }
    }

    pub fn send_to_front(
        &self,
        pvItemToQueue: T,
        xTicksToWait: TickType,
    ) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_FRONT)
        }
    }

    pub fn send_to_back(&self, pvItemToQueue: T, xTicksToWait: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
        }
    }

    pub fn overwrite(&self, pvItemToQueue: T) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, 0, queueOVERWRITE)
        }
    }

    pub fn send_to_front_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueSEND_TO_FRONT)
        }
    }

    pub fn send_to_back_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueSEND_TO_BACK)
        }
    }

    pub fn overwrite_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueOVERWRITE)
        }
    }

    pub fn receive(&self, xTicksToWait: TickType) -> Result<T, QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_receive(xTicksToWait, false)
        }
    }

    pub fn peek(&self, xTicksToWait: TickType) -> Result<T, QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_receive(xTicksToWait, true)
        }
    }
}