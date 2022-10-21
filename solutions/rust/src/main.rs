use std::time::{SystemTime};
mod leetcode;
use leetcode::p509::solution::Solution;

fn main() {
    let start = SystemTime::now();
    let solution = Solution::fib(2);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Solution: {}", solution);
    println!("Time elapsed: {} sec(s)", duration.as_secs_f32());
}
