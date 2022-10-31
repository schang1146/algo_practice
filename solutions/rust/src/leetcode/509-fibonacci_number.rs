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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_fib() {
        assert_eq!(0, Solution::fib(0));
        assert_eq!(1, Solution::fib(1));
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
        assert_eq!(5, Solution::fib(5));
        assert_eq!(8, Solution::fib(6));
        assert_eq!(13, Solution::fib(7));
        assert_eq!(21, Solution::fib(8));
    }

    #[test]
    fn big_fib() {
        assert_eq!(39088169, Solution::fib(38));
        assert_eq!(63245986, Solution::fib(39));
        assert_eq!(102334155, Solution::fib(40));
        assert_eq!(165580141, Solution::fib(41));
        assert_eq!(267914296, Solution::fib(42));
        assert_eq!(433494437, Solution::fib(43));
        assert_eq!(701408733, Solution::fib(44));
        assert_eq!(1134903170, Solution::fib(45));
        assert_eq!(1836311903   , Solution::fib(46));
    }
}