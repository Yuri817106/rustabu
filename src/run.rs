use crate::solution::Solution;
use crate::utils::evaluate;
use crate::search::perturb;
use crate::tabu::TabuList;
use crate::problem::Problem;
use std::time::Instant;

pub fn tabu_run(problem: &Problem, initial_solution: &Solution) -> Solution {
    let mut current_solution = initial_solution.clone();
    let mut best_solution = current_solution.clone();
    let mut best_score = evaluate(problem, &best_solution);

    let mut tabu_list = TabuList::new(current_solution.ss.len());
    let max_iter = 300;
    let candidates_size = 5;
    let mut min_score = best_score;
    let mut max_score = best_score;
    let mut no_improve_count = 0;  // 新增：無改善次數計數器

    let start_time = Instant::now();

    for iter in 0..max_iter {
        let mut candidates = Vec::with_capacity(candidates_size);
        for _ in 0..candidates_size {
            let (neighbor, mask_ss, score) = perturb(problem, &current_solution, problem.processor_count);
            candidates.push((neighbor, mask_ss, score));
        }

        // 依照分數排序，取最佳可接受解
        candidates.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        
        // 先找出最佳可接受解
        let mut best_candidate = None;
        let mut best_candidate_score = f64::MAX;

        for (neighbor, mask_ss, score) in candidates {
            let is_tabu = tabu_list.contains_ss(&mask_ss);
            let aspiration = score < best_score;
            
            if (!is_tabu || aspiration) && score < best_candidate_score {
                best_candidate = Some((neighbor, mask_ss, score));
                best_candidate_score = score;
            }
        }

        if let Some((neighbor, mask_ss, score)) = best_candidate {
            current_solution = neighbor.clone();

            if score < best_score {
                best_solution = neighbor.clone();
                best_score = score;
                no_improve_count = 0;  // 重置計數器
                println!("第 {} 代找到更佳解: makespan = {:.2}", iter, best_score);
            } else {
                no_improve_count += 1;
                if no_improve_count >= max_iter / 5 {
                    // println!("連續 {} 代無改善，重置現行解為最佳解", no_improve_count);
                    current_solution = best_solution.clone();
                    no_improve_count = 0;  // 重置計數器
                }
            }

            if score < min_score {
                min_score = score;
            }
            if score > max_score {
                max_score = score;
            }

            tabu_list.push(mask_ss);
        }
    }

    let elapsed = start_time.elapsed();

    println!("最優解 makespan: {:.2}", best_score);
    println!("最優解: {:?}", best_solution);
    println!("最差解 makespan: {:.2}", max_score);
    println!("最優與最差解差值: {:.2}", max_score - min_score);
    println!("總計算時間: {:.3} 秒", elapsed.as_secs_f64());

    best_solution
}