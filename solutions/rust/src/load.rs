// use std::ffi::OsStr;
use std::fs;
use std::io::Write;
// use std::path::Path;

pub fn load_problem(id: &String) {
    let base_path = "./src/";
    let mod_path = "./src/build/mod.rs";

    // let mut source_dest: &Path;
    // let target_dest: &Path = Path::new(format!("{base_path}foo.txt").as_str());

    for file in fs::read_dir(format!("{base_path}leetcode").as_str()).unwrap() {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let test: Vec<&str> = file_name.split("_").collect();
        if test[0].eq(format!("s{id}").as_str()) {
            let mod_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(mod_path)
                .unwrap();
            writeln!(&mod_file, "mod {};", file_name).unwrap();
            println!("Solution found!");
            // source_dest = Path::new(file_path.as_path())
        }
    }
    // let source_dest = Path::new("./src/leetcode/509-fibonacci_number.rs");
}