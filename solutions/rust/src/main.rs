mod leetcode;
mod load;

// use std::io;
use std::time::{SystemTime};
// use leetcode::s509_fibonacci_number::Solution;
use load::load_problem;

fn main() {
    // let mut user_input = String::new();
    // println!("Enter LeetCode problem #:");
    // io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("Error: Unable to read user input");
    load_problem();
    let start = SystemTime::now();
    let solution = Solution::fib(2);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Solution: {}", solution);
    println!("Time elapsed: {} sec(s)", duration.as_secs_f32());
}
