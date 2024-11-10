use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::*;

/// DEADLOCK
pub const DEAD: isize = -0xDEAD;
pub struct DeadLockDetector {
    avail: Vec<usize>,
    alloc: Vec<Vec<usize>>,
    need: Vec<Vec<usize>>,
    enabled: bool,
}

impl DeadLockDetector {
    fn new() -> Self {
        Self {
            avail: Vec::new(),
            alloc: Vec::new(),
            need: Vec::new(),
            enabled: true,
        }
    }

    pub fn print(&self) {
        println!("avail: {:?}", self.avail);
        println!("alloc: {:?}", self.alloc);
        println!("need: {:?}", self.need);
    }

    pub fn deadlock(&self) -> bool {
        if !self.enabled { return false }
        let mut work = self.avail.clone();
        let mut finish = Vec::new();
        for _ in 0..self.alloc.len() {
            finish.push(false);
        }
        
        loop {
            let unfinished: Vec<usize> = finish.iter()
                .enumerate()
                .filter(|&x| *x.1 == false)
                .map(|x| x.0)
                .collect();
            if unfinished.is_empty() {
                return false;
            }

            let mut flag = false;
            for i in unfinished.iter() {
                let mut statisfied = true;
                for (j, ne) in self.need[*i].iter().enumerate() {
                    if *ne > work[j] { 
                        statisfied = false;
                        break;
                    }
                }
                if statisfied {
                    flag = true;
                    finish[*i] = true;
                    for (j, val) in work.iter_mut().enumerate() {
                        *val += self.alloc[*i][j];
                    }
                }
            }

            if !flag {
                return true;
            }
        }
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    pub fn alloc(&mut self, pid: usize, resource_id: usize, val: isize) {
        while self.alloc.len() <= pid {
            let mut v = Vec::with_capacity(self.avail.len());
            for _ in 0..self.avail.len() {
                v.push(0);
            }
            self.alloc.push(v);
        }
        while self.need.len() <= pid {
            let mut v = Vec::with_capacity(self.avail.len());
            for _ in 0..self.avail.len() {
                v.push(0);
            }
            self.need.push(v);
        }
        self.alloc[pid][resource_id] = (self.alloc[pid][resource_id] as isize + val) as usize;
        self.avail[resource_id] = (self.avail[resource_id] as isize - val) as usize;
        if val >= self.need[pid][resource_id] as isize {
            self.need[pid][resource_id] = 0;
        } else {
            self.need[pid][resource_id] -= val as usize;
        }
    }

    pub fn update_need(&mut self, pid: usize, resource_id: usize, val: isize) {
        while self.need.len() <= pid {
            let mut v = Vec::with_capacity(self.avail.len());
            for _ in 0..self.avail.len() {
                v.push(0);
            }
            self.need.push(v);
        }
        self.need[pid][resource_id] = (self.need[pid][resource_id] as isize + val) as usize;
    }

    pub fn update_avail(&mut self, res_id: usize, val: isize) {
        while self.avail.len() <= res_id {
            self.avail.push(0);
        }
        for ne in self.need.iter_mut() {
            while ne.len() <= res_id {
                ne.push(0);
            }
        }
        for al in self.alloc.iter_mut() {
            while al.len() <= res_id {
                al.push(0);
            }
        }
        self.avail[res_id] = (self.avail[res_id] as isize + val) as usize;
    }
}

lazy_static! {
    pub static ref DETECTOR: UPSafeCell<DeadLockDetector> = unsafe {
        UPSafeCell::new(DeadLockDetector::new())
    };
}

/// deadlock detect
pub fn deadlock() -> bool {
    DETECTOR.exclusive_access().deadlock()
}

/// enable detector
pub fn enable(enable: bool) {
    DETECTOR.exclusive_access().enable(enable);
}

/// alloc resource
pub fn update_alloc(pid: usize, res_id: usize, val: isize) {
    DETECTOR.exclusive_access().alloc(pid, res_id, val);
}

/// update need
pub fn update_need(pid: usize, res_id: usize, val: isize) {
    DETECTOR.exclusive_access().update_need(pid, res_id, val);
}

/// update avial
pub fn update_avail(res_id: usize, val: isize) {
    DETECTOR.exclusive_access().update_avail(res_id, val);
}

/// print
pub fn print() {
    DETECTOR.exclusive_access().print();
}