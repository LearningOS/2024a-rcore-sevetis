//!Implementation of [`TaskManager`]
use core::usize;

use super::TaskControlBlock;
use crate::config::BIG_STRIDE;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }

    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let res = self.ready_queue.pop_front();
        if res.is_none() {
            None
        } else {
            let res = res.unwrap();
            Some(res)
        }
    }

    /// Take a smallest stride process out of the ready queue
    pub fn stride_fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        if let Some((idx, task)) = self.ready_queue.iter().enumerate().min_by_key(|x| {
            x.1.get_stride()
        }) {
            self.ready_queue.remove(idx)
        } else {
            None
        }
    }

    // pub fn print_task(&self) {
    //     let length = self.ready_queue.len();
    //     for i in 0..length {
    //         let task_i = &self.ready_queue[i];
    //         println!("i: {} stride: {} prio: {}", i, task_i.get_stride(), task_i.get_prio());
    //     }
    //     println!("--------------------------");
    // }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

/// Stride fetch
pub fn stride_fetch() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().stride_fetch()
}