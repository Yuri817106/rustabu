use crate::problem::Problem;
use crate::solution::Solution;

/// 評估一個解的總成本（makespan），若解非法則加上懲罰分數
pub fn evaluate(problem: &Problem, solution: &Solution) -> f64 {
    // 非法解的懲罰分數
    const PENALTY: f64 = 3000.0;

    // 1. 檢查解的長度是否正確
    let mut illegal = false;
    if solution.ss.len() != problem.task_count || solution.ms.len() != problem.task_count {
        illegal = true;
    }

    // 2. 檢查 ss 是否為 0..task_count 的排列
    let mut ss_sorted = solution.ss.clone();
    ss_sorted.sort_unstable();
    if ss_sorted != (0..problem.task_count).collect::<Vec<_>>() {
        illegal = true;
    }

    // 2.5. 檢查 ss 是否符合依賴順序
    let mut pos = vec![0; problem.task_count];
    for (i, &task_id) in solution.ss.iter().enumerate() {
        pos[task_id] = i;
    }
    for &(from, to, _) in &problem.trans_data_vol {
        if pos[from] >= pos[to] {
            illegal = true;
            break;
        }
    }

    // 3. 檢查 ms 是否都在合法處理器範圍
    if solution.ms.iter().any(|&m| m >= problem.processor_count) {
        illegal = true;
    }

    // 4. 建立每個任務的開始與結束時間
    let mut start_time = vec![0.0; problem.task_count];
    let mut finish_time = vec![0.0; problem.task_count];

    // 5. 建立每個處理器的可用時間
    let mut proc_available = vec![0.0; problem.processor_count];

    // 6. 建立任務指派的處理器
    let mut task_to_proc = vec![0; problem.task_count];
    for (i, &task_id) in solution.ss.iter().enumerate() {
        task_to_proc[task_id] = solution.ms[i];
    }

    // 7. 建立前驅關係
    let mut preds: Vec<Vec<usize>> = vec![vec![]; problem.task_count];
    for &(from, to, _) in &problem.trans_data_vol {
        preds[to].push(from);
    }

    // 8. 按照 ss 的順序排程
    for (i, &task_id) in solution.ss.iter().enumerate() {
        let proc_id = solution.ms[i];
        let mut ready_time: f64 = 0.0;
        for &pred in &preds[task_id] {
            let pred_proc = task_to_proc[pred];
            let comm_time = if pred_proc == proc_id {
                0.0
            } else {
                let data_vol = problem
                    .trans_data_vol
                    .iter()
                    .find(|&&(f, t, _)| f == pred && t == task_id)
                    .map(|&(_, _, v)| v)
                    .unwrap_or(0.0);
                data_vol * problem.comm_rate[pred_proc][proc_id]
            };
            // 印出通訊成本
            // println!(
            //     "task {} <- pred {}: comm_time = {}, finish_time[pred] = {}",
            //     task_id, pred, comm_time, finish_time[pred]
            // );
            ready_time = ready_time.max(finish_time[pred] + comm_time);
        }
        let est = f64::max(proc_available[proc_id], ready_time);
        let cost = problem.comp_cost[task_id][proc_id];
        start_time[task_id] = est;
        finish_time[task_id] = est + cost;
        proc_available[proc_id] = finish_time[task_id];
        // 印出每個任務的排程
        // println!(
        //     "task {} on proc {}: start = {}, finish = {}, cost = {}",
        //     task_id, proc_id, start_time[task_id], finish_time[task_id], cost
        // );
    }

    // 9. makespan = 所有任務的最大完成時間
    let makespan = finish_time
        .iter()
        .cloned()
        .fold(0.0_f64, f64::max);

    if illegal {
        makespan + PENALTY
    } else {
        makespan
    }
}