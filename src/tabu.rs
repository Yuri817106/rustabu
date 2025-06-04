use std::collections::VecDeque;

pub struct TabuList {
    pub ss_list: VecDeque<Vec<u8>>,
    pub ms_list: VecDeque<Vec<u8>>,
    pub max_len: usize,
}

impl TabuList {
    pub fn new(task_count: usize) -> Self {
        let max_len = task_count * 7; // 可依需求調整
        TabuList {
            ss_list: VecDeque::with_capacity(max_len),
            ms_list: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    pub fn push(&mut self, mask_ss: Vec<u8>, mask_ms: Vec<u8>) {
        if self.ss_list.len() == self.max_len {
            self.ss_list.pop_front();
        }
        if self.ms_list.len() == self.max_len {
            self.ms_list.pop_front();
        }
        self.ss_list.push_back(mask_ss);
        self.ms_list.push_back(mask_ms);
    }

    pub fn contains_ss(&self, mask: &Vec<u8>) -> bool {
        self.ss_list.contains(mask)
    }

    pub fn contains_ms(&self, mask: &Vec<u8>) -> bool {
        self.ms_list.contains(mask)
    }
}