use rustabu::io::load_problem_from_file;
use rustabu::solution::Solution;
use rustabu::run::tabu_run;

fn main() {
    // 檔案讀取之路徑
    let path = "src/P4/n4_00.dag";

    // 初始解
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
            vec![0, 1, 3, 7, 5, 6, 2, 4, 11, 9, 10, 12, 13, 14, 16, 8, 15, 18, 17, 19],
            vec![3, 0, 0, 0, 0, 0, 3, 0, 2, 0, 3, 0, 3, 0, 0, 2, 0, 0, 0, 2],
        ),
        (
            vec![0, 1, 3, 5, 7, 6, 2, 4, 11, 9, 10, 12, 13, 14, 16, 8, 15, 18, 17, 19],
            vec![2, 0, 0, 0, 0, 0, 3, 0, 2, 0, 3, 0, 3, 0, 0, 2, 0, 0, 0, 2],
        ),
    ];

    // 選擇初始解
    let which = std::env::args().nth(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    if which >= initial_solutions.len() {
        eprintln!("請輸入 0~{} 之間的整數作為初始解選擇", initial_solutions.len() - 1);
        return;
    }

    // 執行
    match load_problem_from_file(path) {
        Ok(problem) => {
            let (ss, ms) = &initial_solutions[which];
            let initial_solution = Solution::new(ss.clone(), ms.clone());
            let _best_solution = tabu_run(&problem, &initial_solution);
        }
        Err(e) => {
            eprintln!("Error reading problem file: {}", e);
        }
    }
}