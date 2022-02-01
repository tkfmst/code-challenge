/// 1. Two Sum
///
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
///  
///
/// Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&index) => return vec![index, i as i32],
                None => {
                    let complement = target - num;

                    map.insert(complement, i as i32);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum_1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];
        assert_eq!(Solution::two_sum(input, target), output);
    }

    #[test]
    fn test_two_sum_2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(input, target), output);
    }

    #[test]
    fn test_two_sum_3() {
        let input = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];
        assert_eq!(Solution::two_sum(input, target), output);
    }
}
