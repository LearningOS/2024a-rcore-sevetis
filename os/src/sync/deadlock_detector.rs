use alloc::vec::Vec;
use alloc::vec;

/// DEADLOCK
pub const DEAD: isize = -0xDEAD;

/// detector
pub struct DeadLockDetector {
    avail: Vec<usize>,
    alloc: Vec<Vec<usize>>,
    need: Vec<Vec<usize>>,
    enabled: bool,
}

impl DeadLockDetector {
    /// create
    pub fn new() -> Self {
        Self {
            avail: Vec::new(),
            alloc: Vec::new(),
            need: Vec::new(),
            enabled: true,
        }
    }

    /// init
    pub fn init(&mut self, size: usize) {
        self.avail = Vec::new();
        self.alloc = vec![Vec::new(); size];
        self.need = vec![Vec::new(); size];
    }

    /// doc
    pub fn print(&self) {
        println!("avail: {:?}", self.avail);
        println!("alloc: {:?}", self.alloc);
        println!("need: {:?}", self.need);
    }

    /// detect deadlock
    pub fn detect(&mut self, tid: usize, res_id: usize) -> bool {
        while self.need.len() <= tid {
            self.need.push(vec![0; self.avail.len()]);
        }
        self.need[tid][res_id] += 1;
        if self.deadlock() {
            self.need[tid][res_id] -= 1;
            return true;
        }
        false
    }

    fn deadlock(&self) -> bool {
        if !self.enabled { return false }
        let mut work = self.avail.clone();
        let mut finish = vec![false; self.alloc.len()];
        
        loop {
            let mut flag = false;
            for i in 0..self.alloc.len() {
                if finish[i] { continue; }
                let mut statisfied = true;
                for j in 0..self.need[i].len() {
                    if self.need[i][j] > work[j] {
                        statisfied = false;
                        break;
                    }
                }
                if statisfied {
                    for j in 0..self.need[i].len() {
                        work[j] += self.alloc[i][j];
                    }
                    finish[i] = true;
                    flag = true;
                }
            }
            if !flag {
                break;
            }
        }
        !finish.iter().all(|&x| x)
    }

    /// turn on/off
    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    /// add resource
    pub fn add_res(&mut self, val: usize) {
        self.avail.push(val);
        self.alloc.iter_mut().for_each(|v| v.push(0));
        self.need.iter_mut().for_each(|v| v.push(0));
    }

    /// add task
    pub fn add_task(&mut self) {
        let length = self.avail.len();
        self.alloc.push(vec![0; length]);
        self.need.push(vec![0; length]);
    }

    /// allocate resource
    pub fn alloc_res(&mut self, tid: usize, res_id: usize) {
        self.need[tid][res_id] -= 1;
        while self.alloc.len() <= tid {
            self.alloc.push(vec![0; self.avail.len()]);
        }
        self.alloc[tid][res_id] += 1;
        self.avail[res_id] -= 1;
    }

    /// deallocate resource
    pub fn dealloc_res(&mut self, tid: usize, res_id: usize) {
        self.alloc[tid][res_id] -= 1;
        self.avail[res_id] += 1;
    }
}
