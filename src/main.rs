use rustabu::io::load_problem_from_file;
use rustabu::solution::Solution;
use rustabu::utils::evaluate;
use rustabu::search::perturb;

fn main() {
    let path = "src/P4/n4_00.dag";
    let initial_solutions = vec![
        (
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
            vec![1, 0, 2, 3, 1, 2, 0, 1, 2, 3, 2, 0, 1, 2, 0, 2, 1, 2, 0, 3],
        ),
        (
            vec![0, 1, 2, 5, 4, 3, 6, 7, 9, 8, 11, 10, 12, 14, 15, 13, 16, 17, 18, 19],
            vec![2, 0, 1, 2, 1, 3, 0, 2, 1, 0, 2, 3, 0, 2, 3, 1, 2, 3, 0, 2],
        ),
        (
            vec![0, 1, 2, 6, 5, 4, 3, 7, 11, 8, 9, 10, 12, 13, 14, 15, 16, 18, 17, 19],
            vec![3, 1, 2, 0, 2, 1, 3, 0, 2, 0, 3, 1, 2, 0, 1, 2, 3, 0, 0, 1],
        ),
        (
            vec![0, 1, 6, 3, 2, 5, 4, 7, 10, 9, 11, 8, 12, 15, 14, 13, 16, 18, 17, 19],
            vec![2, 3, 2, 0, 2, 1, 3, 2, 0, 3, 2, 3, 2, 2, 1, 2, 3, 0, 2, 0],
        ),
        (
            vec![0, 1, 4, 5, 6, 3, 2, 7, 8, 9, 10, 11, 12, 14, 15, 13, 16, 18, 17, 19],
            vec![2, 2, 1, 0, 3, 2, 2, 3, 3, 0, 1, 0, 2, 3, 0, 3, 3, 1, 0, 2],
        ),
    ];

    let which = std::env::args().nth(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    if which >= initial_solutions.len() {
        eprintln!("請輸入 0~{} 之間的整數作為初始解選擇", initial_solutions.len() - 1);
        return;
    }

    match load_problem_from_file(path) {
        Ok(problem) => {
            let (ss, ms) = &initial_solutions[which];
            let solution = Solution::new(ss.clone(), ms.clone());

            let score = evaluate(&problem, &solution);
            println!("第 {} 組初始解的 makespan 評估值為: {}", which, score);

            // 產生鄰近解並取得 mask
            let (neighbor, mask_ss, mask_ms) = perturb(&solution, problem.processor_count);
            let neighbor_score = evaluate(&problem, &neighbor);
            println!("鄰近解: {:?}", neighbor);
            println!("鄰近解的 makespan 評估值為: {}", neighbor_score);
            println!("鄰近解變動 mask_ss: {:?}", mask_ss);
            println!("鄰近解變動 mask_ms: {:?}", mask_ms);
        }
        Err(e) => {
            eprintln!("Error reading problem file: {}", e);
        }
    }
}