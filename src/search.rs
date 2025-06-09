use crate::solution::Solution;
use crate::utils::evaluate;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn perturb(problem: &crate::problem::Problem, solution: &Solution, processor_count: usize) -> (Solution, Vec<u8>, f64) {
    let mut rng = rand::thread_rng();
    loop {
        let mut new_ss = solution.ss.clone();
        let mut new_ms = solution.ms.clone();
        let mut mask_ss = vec![0u8; new_ss.len()];

        let len_ss = new_ss.len();
        let len_ms = new_ms.len();
        let op = rng.gen_range(0..3); // 0: 單一交換, 1: 隨機重排, 2: 批次交換
        match op {
            0 => { // 單一交換
                if len_ss > 1 {
                    let i = rng.gen_range(0..len_ss);
                    let mut j = rng.gen_range(0..len_ss);
                    while j == i {
                        j = rng.gen_range(0..len_ss);
                    }
                    new_ss.swap(i, j);
                    mask_ss[i] = 1;
                    mask_ss[j] = 1;
                }
            },
            1 => { // 隨機重排一小段
                if len_ss > 3 {
                    let start = rng.gen_range(0..(len_ss-2));
                    let end = (start + rng.gen_range(2..=3)).min(len_ss);
                    new_ss[start..end].shuffle(&mut rng);
                    for idx in start..end {
                        mask_ss[idx] = 1;
                    }
                }
            },
            2 => { // 批次交換
                if len_ss > 4 {
                    for _ in 0..2 {
                        let i = rng.gen_range(0..len_ss);
                        let mut j = rng.gen_range(0..len_ss);
                        while j == i {
                            j = rng.gen_range(0..len_ss);
                        }
                        new_ss.swap(i, j);
                        mask_ss[i] = 1;
                        mask_ss[j] = 1;
                    }
                }
            },
            _ => {}
        }

        // ms 指派到不同處理器
        if len_ms > 0 {
            let i = rng.gen_range(0..len_ms);
            let mut new_proc = rng.gen_range(0..processor_count);
            while new_proc == new_ms[i] && processor_count > 1 {
                new_proc = rng.gen_range(0..processor_count);
            }
            new_ms[i] = new_proc;
        }

        let new_solution = Solution::new(new_ss.clone(), new_ms.clone());
        let score = evaluate(problem, &new_solution);
        // 只回傳合法解
        if score < 3000.0 {
            return (new_solution, mask_ss, score);
        }
        // 否則繼續產生
    }
}