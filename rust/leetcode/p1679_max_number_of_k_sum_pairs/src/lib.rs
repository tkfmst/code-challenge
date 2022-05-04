/// 1679. Max Number of K-Sum Pairs
/// Medium
///
/// You are given an integer array nums and an integer k.
///
/// In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.
///
/// Return the maximum number of operations you can perform on the array.
///
///
/// Example 1:
///
/// Input: nums = [1,2,3,4], k = 5
/// Output: 2
/// Explanation: Starting with nums = [1,2,3,4]:
/// - Remove numbers 1 and 4, then nums = [2,3]
/// - Remove numbers 2 and 3, then nums = []
/// There are no more pairs that sum up to 5, hence a total of 2 operations.
///
/// Example 2:
///
/// Input: nums = [3,1,3,4,3], k = 6
/// Output: 1
/// Explanation: Starting with nums = [3,1,3,4,3]:
/// - Remove the first two 3's, then nums = [1,4,3]
/// There are no more pairs that sum up to 6, hence a total of 1 operation.
///
///
/// Constraints:
/// * 1 <= nums.length <= 10^5
/// * 1 <= nums[i] <= 10^9
/// * 1 <= k <= 10^9
pub struct Solution {}
impl Solution {
    // answer 2 O(logN)
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        nums.sort();
        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            let s = nums[l] + nums[r];
            if s == k {
                l += 1;
                r -= 1;
                result += 1;
            } else if s < k {
                l += 1;
            } else {
                r -= 1;
            }
        }

        result
    }
    // // answer 1 O(n)
    //  use std::collections::HashMap;
    // pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut result = 0;
    //     let mut hash = HashMap::new();
    //
    //     for i in 0..nums.len() {
    //         let n1 = hash.entry(k - nums[i]).or_insert(0);
    //         if *n1 > 0 {
    //             *n1 -= 1;
    //             result += 1;
    //         } else {
    //             let n2 = hash.entry(nums[i]).or_insert(0);
    //             *n2 += 1;
    //         }
    //     }
    //
    //     result
    // }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_operations_1() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        let output = 2;
        assert_eq!(Solution::max_operations(nums, k), output);
    }

    #[test]
    fn test_max_operations_2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        let output = 1;
        assert_eq!(Solution::max_operations(nums, k), output);
    }
}
