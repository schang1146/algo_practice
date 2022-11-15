use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.iter().enumerate() {
            if lookup.contains_key(num) {
                let found = lookup.get(num).unwrap();
                return vec![*found, idx as i32];
            }

            lookup.insert(target - num, idx as i32);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }

    #[test]
    fn test_order() {
        assert!(Solution::two_sum(vec![15, 7, 11, 2], 9).contains(&1));
        assert!(Solution::two_sum(vec![15, 7, 11, 2], 9).contains(&3));
    }
}