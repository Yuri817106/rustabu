use crate::solution::Solution;
use crate::utils::evaluate;
use rand::Rng;

pub fn perturb(problem: &crate::problem::Problem, solution: &Solution, processor_count: usize) -> (Solution, Vec<u8>, f64) {
    let mut rng = rand::thread_rng();
    loop {
        let mut new_ss = solution.ss.clone();
        let mut new_ms = solution.ms.clone();
        let mut mask_ss = vec![0u8; new_ss.len()];

        // swap ss 內兩個不同位置
        let len_ss = new_ss.len();
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

        // ms 指派到不同處理器
        let len_ms = new_ms.len();
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