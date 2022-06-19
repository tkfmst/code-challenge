/// 594. Longest Harmonious Subsequence
/// Easy
///
/// We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
///
/// Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
///
/// A subsequence of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.
///
///
/// Example 1:
///
/// Input: nums = [1,3,2,2,5,2,3,7]
/// Output: 5
/// Explanation: The longest harmonious subsequence is [3,2,2,2,3].
///
/// Example 2:
///
/// Input: nums = [1,2,3,4]
/// Output: 2
///
/// Example 3:
///
/// Input: nums = [1,1,1,1]
/// Output: 0
///
///
/// Constraints:
/// * 1 <= nums.length <= 2 * 10^4
/// * -10^9 <= nums[i] <= 10^9

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut dict = HashMap::new();
        for i in 0..nums.len() {
            *dict.entry(nums[i]).or_insert(0) += 1;
        }

        let mut max = 0;
        for n in nums {
            if let Some(pre) = dict.get(&(n - 1)) {
                if dict.get(&n).unwrap() + pre > max {
                    max = dict.get(&n).unwrap() + pre;
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_lhs_1() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        // let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        // let nums = vec![-2,1, 0, -3,3, 2, 3, 7];

        let output = 5;
        assert_eq!(Solution::find_lhs(nums), output);
    }

    #[test]
    fn test_find_lhs_2() {
        let nums = vec![1, 2, 3, 4];
        let output = 2;
        assert_eq!(Solution::find_lhs(nums), output);
    }

    #[test]
    fn test_find_lhs_3() {
        let nums = vec![1, 1, 1, 1];
        let output = 0;
        assert_eq!(Solution::find_lhs(nums), output);
    }
}
