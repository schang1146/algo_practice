// use std::ffi::OsStr;
use std::fs;
// use std::path::Path;

pub fn load_problem(id: &String) {
    let base_path = "./src/";

    // let mut source_dest: &Path;
    // let target_dest: &Path = Path::new(format!("{base_path}foo.txt").as_str());

    for file in fs::read_dir(format!("{base_path}leetcode").as_str()).unwrap() {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap();
        let test: Vec<&str> = file_name.to_str().unwrap().split("-").collect();
        if test[0].eq(id.trim()) {
            println!("Solution found!");
            // source_dest = Path::new(file_path.as_path())
        }
    }
    // let source_dest = Path::new("./src/leetcode/509-fibonacci_number.rs");
}