use crate::problem::Problem;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_problem_from_file(path: &str) -> Result<Problem, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // 過濾註釋行和空行，只保留有效數據行
    let mut in_comment = false;  // 追蹤是否在多行註釋中
    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            let trimmed = line.trim();
            
            // 檢查多行註釋的開始和結束
            if trimmed.contains("/*") {
                in_comment = true;
                return false;
            }
            if trimmed.contains("*/") {
                in_comment = false;
                return false;
            }
            
            // 如果在註釋中或是其他需要過濾的情況，則跳過該行
            !in_comment && 
            !trimmed.is_empty() && 
            !trimmed.starts_with("//") &&
            !trimmed.contains("===") &&
            !trimmed.starts_with("本區段") &&
            !trimmed.starts_with("以下區段") &&
            !trimmed.contains("之參數") &&
            !trimmed.contains("的內容表示") &&
            !trimmed.contains("所需的時間") &&
            !trimmed.contains("之間的資料交換量")
        })
        .map(|line| line.trim().to_string())
        .collect();

    // 用於除錯
    println!("過濾後的有效行:");
    for (i, line) in lines.iter().enumerate() {
        println!("第 {} 行: {}", i + 1, line);
    }

    let mut lines = lines.iter();

    // 讀取基本參數
    let processor_count = lines.next()
        .ok_or("缺少處理器數量")?
        .parse()
        .map_err(|e| format!("無法解析處理器數量: {}", e))?;
    
    let task_count = lines.next()
        .ok_or("缺少任務數量")?
        .trim()
        .parse()
        .map_err(|e| format!("無法解析任務數量: {}", e))?;
    
    let edge_count = lines.next()
        .ok_or("缺少邊緣數量")?
        .trim()
        .parse()
        .map_err(|e| format!("無法解析邊緣數量: {}", e))?;

    // 讀取 comm_rate
    let mut comm_rate = Vec::new();
    for i in 0..processor_count {
        let row: Result<Vec<f64>, _> = lines.next()
            .ok_or(format!("缺少通信率矩陣第 {} 行", i + 1))?
            .split_whitespace()
            .map(|x| x.parse().map_err(|e| format!("無法解析通信率值 '{}': {}", x, e)))
            .collect();
        comm_rate.push(row?);
    }

    // 讀取 comp_cost
    let mut comp_cost = Vec::new();
    for i in 0..task_count {
        let row: Result<Vec<f64>, _> = lines.next()
            .ok_or(format!("缺少計算成本矩陣第 {} 行", i + 1))?
            .split_whitespace()
            .map(|x| x.parse().map_err(|e| format!("無法解析計算成本值 '{}': {}", x, e)))
            .collect();
        comp_cost.push(row?);
    }

    // 讀取 trans_data_vol
    let mut trans_data_vol = Vec::new();
    for i in 0..edge_count {
        let line = lines.next()
            .ok_or(format!("缺少傳輸數據量第 {} 行", i + 1))?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() != 3 {
            return Err(format!("傳輸數據量行 {} 格式錯誤: 需要 3 個值，得到 {}", i + 1, parts.len()).into());
        }

        let from = parts[0].parse()
            .map_err(|e| format!("無法解析來源節點 '{}': {}", parts[0], e))?;
        let to = parts[1].parse()
            .map_err(|e| format!("無法解析目標節點 '{}': {}", parts[1], e))?;
        let vol = parts[2].parse()
            .map_err(|e| format!("無法解析數據量 '{}': {}", parts[2], e))?;
        
        trans_data_vol.push((from, to, vol));
    }

    Ok(Problem {
        processor_count,
        task_count,
        edge_count,
        comm_rate,
        comp_cost,
        trans_data_vol,
    })
}