pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut fib_data: [i32; 3] = [0, 1, 1];
        if n < 0 {
            panic!("Error: Can't get negative Fibonacci numbers.")
        }
        if n <= 2 {
            return fib_data[n as usize]
        }
        if n > 46 {
            panic!("Error: Can't handle inputs larger than 46!")
        }
        for _ in 0..(n-2) {
            fib_data = [fib_data[1], fib_data[2], fib_data[1] + fib_data[2]]
        }
        return fib_data[2]
    }
}