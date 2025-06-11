use rustabu::io::load_problem_from_file;
use rustabu::run::{tabu_run, random_valid_solution};
use rustabu::plot::plot_convergence;

fn main() {
    // 檔案讀取之路徑
    let path = "src/P4/n4_00.dag";

    // 執行
    match load_problem_from_file(path) {
        Ok(problem) => {
            // 隨機產生初始解
            let initial_solution = random_valid_solution(&problem);
            let (_best_solution, costs) = tabu_run(&problem, &initial_solution);
            
            // 繪製收斂曲線
            if let Err(e) = plot_convergence(&costs[..300], "convergence.png") {
                eprintln!("繪圖錯誤: {}", e);
            } else {
                println!("收斂曲線已保存至 convergence.png");
            }
        }
        Err(e) => {
            eprintln!("Error reading problem file: {}", e);
        }
    }
}