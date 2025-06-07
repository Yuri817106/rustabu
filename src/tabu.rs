use std::collections::VecDeque;

pub struct TabuList {
    pub ss_list: VecDeque<Vec<u8>>,
    pub max_len: usize,
}

impl TabuList {
    pub fn new(task_count: usize) -> Self {
        let max_len = task_count / 3; // 可依需求調整
        TabuList {
            ss_list: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    pub fn push(&mut self, mask_ss: Vec<u8>) {
        if self.ss_list.len() == self.max_len {
            self.ss_list.pop_front();
        }
        self.ss_list.push_back(mask_ss);
    }

    pub fn contains_ss(&self, mask: &Vec<u8>) -> bool {
        self.ss_list.contains(mask)
    }

    // 顯示目前 tabu list 內容
    pub fn print(&self) {
        println!("TabuList (ss_list):");
        for (i, mask) in self.ss_list.iter().enumerate() {
            println!("  ss[{}]: {:?}", i, mask);
        }
    }
}