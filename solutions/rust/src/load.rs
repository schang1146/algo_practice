use std::fs;
use std::path::Path;

pub fn load_problem() -> std::io::Result<()> {
    let source_dest = "./src/leetcode/509-fibonacci_number.rs";
    let target_dest = "./src/foo.txt";
    fs::copy(source_dest, target_dest)?;
    Ok(())
}