use crate::solution::Solution;
use crate::utils::evaluate;
use crate::search::perturb;
use crate::tabu::TabuList;
use crate::problem::Problem;
use std::time::Instant;
use rand::Rng;

// 隨機產生一個合法的解（隨機拓撲排序 + 隨機處理器分配）
fn random_valid_solution(problem: &Problem) -> Solution {
    let mut rng = rand::thread_rng();

    // 1. 隨機拓撲排序
    let mut in_deg = vec![0; problem.task_count];
    for &(_from, to, _) in &problem.trans_data_vol {
        in_deg[to] += 1;
    }
    
    let mut ready: Vec<usize> = (0..problem.task_count).filter(|&i| in_deg[i] == 0).collect();
    let mut ss = Vec::with_capacity(problem.task_count);
    while !ready.is_empty() {
        let idx = rng.gen_range(0..ready.len());
        let task = ready.remove(idx);
        ss.push(task);
        for &(_from, to, _) in &problem.trans_data_vol {
            if _from == task {
                in_deg[to] -= 1;
                if in_deg[to] == 0 {
                    ready.push(to);
                }
            }
        }
    }
    // 2. 隨機處理器分配
    let ms: Vec<usize> = (0..problem.task_count)
        .map(|_| rng.gen_range(0..problem.processor_count))
        .collect();
    Solution::new(ss, ms)
}

pub fn tabu_run(problem: &Problem, initial_solution: &Solution) -> (Solution, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let mut current_solution = initial_solution.clone();
    let mut best_solution = current_solution.clone();
    let mut best_score = evaluate(problem, &best_solution);

    let mut tabu_list = TabuList::new(current_solution.ss.len());
    let max_iter = 300;
    let candidates_size = 5;
    let mut min_score = best_score;
    let mut max_score = best_score;
    let mut no_improve_count = 0;  // 新增：無改善次數計數器
    let mut cost_history = Vec::with_capacity(max_iter);

    let start_time = Instant::now();

    let mut temp = 100.0; // 初始溫度
    let cooling = 0.99;   // 降溫速率

    for iter in 0..max_iter {
        // cost_history.push(evaluate(problem, &current_solution)); // 只在這裡 push 一次
        cost_history.push(best_score); // 只在這裡 push 一次
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
            // 模擬退火接受機率
            let accept = if score < best_score {
                true
            } else {
                let prob = f64::exp((best_score - score) / temp);
                rng.gen_bool(prob.min(1.0))
            };
            if accept {
                current_solution = neighbor.clone();

                if score < best_score {
                    best_solution = neighbor.clone();
                    best_score = score;
                    no_improve_count = 0;  // 重置計數器
                    println!("第 {} 代找到更佳解: makespan = {:.2}", iter, best_score);
                } else {
                    no_improve_count += 1;
                    if no_improve_count >= max_iter / 5 {
                        current_solution = random_valid_solution(problem);
                        no_improve_count = 0;
                        println!("第 {} 代重啟隨機合法新解", iter);
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
            temp *= cooling; // 降溫
        }
    }

    let elapsed = start_time.elapsed();

    println!("最優解 makespan: {:.2}", best_score);
    println!("最優解: {:?}", best_solution);
    println!("總計算時間: {:.3} 秒", elapsed.as_secs_f64());

    (best_solution, cost_history)
}