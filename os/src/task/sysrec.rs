use alloc::vec::Vec;
use crate::sync::UPSafeCell;
use crate::config::MAX_SYSCALL_NUM;
use super::processor::current_pid;
use lazy_static::*;

struct SyscallTimes {
    syscall_times: Vec<(usize, u32)>,
}

impl SyscallTimes {

    /// Get syscall times
    pub fn all_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        let mut result = [0; MAX_SYSCALL_NUM];
        for (pid, cnt) in self.syscall_times.iter() {
            result[*pid] = *cnt;
        }
        result
    }

    pub fn record_syscall(&mut self, syscall_id: usize) {
        if let Some((_, cnt)) = self.syscall_times.iter_mut().find(|(id, _)| *id == syscall_id) {
            *cnt += 1;
        } else {
            self.syscall_times.push((syscall_id, 1));
        }
    }
} 


pub struct SyscallRec {
    /// (pid, (syscall_id, syscall_time))
    process_syscalls: Vec<(usize, SyscallTimes)>,
}

impl SyscallRec {

    pub fn new() -> Self {
        Self {
            process_syscalls: Vec::new()
        }
    }

    /// Get the syscall times of a process with its pid
    fn get_process_syscalltimes(&self, pid: usize) -> Option<&SyscallTimes> {
        if let Some((_, syscall_time)) = self.process_syscalls
            .iter()
            .find(|(_pid, _)| {*_pid == pid}) {
            Some(syscall_time)
        } else {
            None
        }
    }

    fn get_process_syscalltimes_mut(&mut self, pid: usize) -> Option<&mut SyscallTimes> {
        if let Some((_, syscall_time)) = self.process_syscalls
            .iter_mut()
            .find(|(_pid, _)| {*_pid == pid}) {
            Some(syscall_time)
        } else {
            None
        }
    }

    /// record syscall times
    pub fn record_syscall(&mut self, syscall_id: usize) {
        let pid = current_pid();
        if let Some(syscall_times) = self.get_process_syscalltimes_mut(pid) {
            syscall_times.record_syscall(syscall_id);
        } else {
            let mut syscall_times: Vec<(usize, u32)> = Vec::new();
            syscall_times.push((syscall_id, 1));
            self.process_syscalls.push((
                pid,
                SyscallTimes { syscall_times }
            ));
        }
    }

    /// Get current task syscall times
    pub fn current_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        let syscall_times = self.get_process_syscalltimes(current_pid()).unwrap();
        syscall_times.all_syscall_times()
    }

}


lazy_static! {
    pub static ref SYSCALL_RECORDER: UPSafeCell<SyscallRec> = 
        unsafe { UPSafeCell::new(SyscallRec::new()) };
}

/// Add syscall times by one
pub fn record_syscall(syscall_id: usize) {
    SYSCALL_RECORDER.exclusive_access().record_syscall(syscall_id);
}

/// Get current syscall times
pub fn current_syscall_times() -> [u32; MAX_SYSCALL_NUM] {
    SYSCALL_RECORDER.exclusive_access().current_syscall_times()
}