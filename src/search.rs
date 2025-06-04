use crate::solution::Solution;
use rand::Rng;

// 隨機產生一個鄰近解（擾動 ss 或 ms），並分別回傳 ss/ms 的變動 mask
pub fn perturb(solution: &Solution, processor_count: usize) -> (Solution, Vec<u8>, Vec<u8>) {
    let mut rng = rand::thread_rng();
    let mut new_ss = solution.ss.clone();
    let mut new_ms = solution.ms.clone();
    let mut mask_ss = vec![0u8; new_ss.len()];
    let mut mask_ms = vec![0u8; new_ms.len()];

    if rng.gen_bool(0.5) {
        // 隨機交換 ss 內兩個不同位置
        let len = new_ss.len();
        if len > 1 {
            let i = rng.gen_range(0..len);
            let mut j = rng.gen_range(0..len);
            while j == i {
                j = rng.gen_range(0..len);
            }
            new_ss.swap(i, j);
            mask_ss[i] = 1;
            mask_ss[j] = 1;
        }
    } else {
        // 隨機選一個任務，指派到不同處理器
        let len = new_ms.len();
        if len > 0 {
            let i = rng.gen_range(0..len);
            let mut new_proc = rng.gen_range(0..processor_count);
            while new_proc == new_ms[i] && processor_count > 1 {
                new_proc = rng.gen_range(0..processor_count);
            }
            new_ms[i] = new_proc;
            mask_ms[i] = 1;
        }
    }

    (Solution::new(new_ss, new_ms), mask_ss, mask_ms)
}