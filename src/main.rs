use rustabu::io::load_problem_from_file;
use rustabu::solution::Solution;
use rustabu::utils::evaluate;
use rustabu::search::perturb;
use rustabu::tabu::TabuList;
use std::time::Instant;

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
            let mut current_solution = Solution::new(ss.clone(), ms.clone());
            let mut best_solution = current_solution.clone();
            let mut best_score = evaluate(&problem, &best_solution);

            let mut tabu_list = TabuList::new(ss.len());
            let max_iter = 1000;
            let mut min_score = best_score;
            let mut max_score = best_score;

            let start_time = Instant::now();

            for iter in 0..max_iter {
                let (neighbor, mask_ss, mask_ms) = perturb(&current_solution, problem.processor_count);
                let neighbor_score = evaluate(&problem, &neighbor);

                // 封鎖非法解
                if neighbor_score >= 3000.0 {
                    continue;
                }

                let is_tabu = tabu_list.contains_ss(&mask_ss) && tabu_list.contains_ms(&mask_ms);
                let aspiration = neighbor_score < best_score;

                if is_tabu && !aspiration {
                    continue;
                }

                // 更新目前解
                current_solution = neighbor.clone();

                if neighbor_score < best_score {
                    best_solution = neighbor.clone();
                    best_score = neighbor_score;
                    println!("第 {} 代找到更佳解: makespan = {:.2}", iter, best_score);
                }

                if neighbor_score < min_score {
                    min_score = neighbor_score;
                }
                if neighbor_score > max_score {
                    max_score = neighbor_score;
                }

                tabu_list.push(mask_ss, mask_ms);
            }

            let elapsed = start_time.elapsed();

            println!("最優解 makespan: {:.2}", best_score);
            println!("最優解: {:?}", best_solution);
            println!("最差解 makespan: {:.2}", max_score);
            println!("最優與最差解差值: {:.2}", max_score - min_score);
            println!("總計算時間: {:.3} 秒", elapsed.as_secs_f64());
        }
        Err(e) => {
            eprintln!("Error reading problem file: {}", e);
        }
    }
}