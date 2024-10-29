//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, mm::translated_byte_buffer, task::{
        change_program_brk,
        current_status,
        current_user_token,
        exit_current_and_run_next, suspend_current_and_run_next, syscall_times, TaskStatus
    },
    timer::{
        get_time_ms,
        get_time_us
    }
};

use alloc::vec;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let mut buffers = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>()
    );
    if buffers.len() > 1 {
        panic!("unimplemented!");
    }

    let buffer = &mut buffers[0];
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let ptr = &time_val as *const TimeVal as *const u8;
    let mut bytes = vec![0; buffer.len()];
    for i in 0..buffer.len() {
        bytes[i] = unsafe { *ptr.add(i) };
    }
    buffer.copy_from_slice(&bytes[..]);

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let mut buffers = translated_byte_buffer(
        current_user_token(),
        _ti as *const u8,
        core::mem::size_of::<TaskInfo>()
    );

    if buffers.len() > 1 {
        panic!("unimplemented!");
    }

    let mut syscts = [0; MAX_SYSCALL_NUM];
    for (id, cnt) in syscall_times().iter() {
        syscts[*id] = *cnt;
    }
    let info = TaskInfo {
        status: current_status(),
        syscall_times: syscts,
        time: get_time_ms(),
    };

    let buffer = &mut buffers[0];
    let ptr = &info as *const TaskInfo as *const u8;
    let mut bytes = vec![0; buffer.len()];
    for i in 0..buffer.len() {
        bytes[i] = unsafe { *ptr.add(i) };
    }
    buffer.copy_from_slice(&bytes[..]);

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if port & 0x7 == 0 || port & !(0x7) != 0 {
        return -1;
    }
    let (r, w, x) = (port & 1, port & 2, port & 4);



    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
