use rustabu::io::load_problem_from_file;

fn main() {
    let path = "src/P4/n4_00.dag";

    match load_problem_from_file(path) {
        Ok(problem) => {
            println!("{:#?}", problem);
        }
        Err(e) => {
            eprintln!("Error reading problem file: {}", e);
        }
    }
}