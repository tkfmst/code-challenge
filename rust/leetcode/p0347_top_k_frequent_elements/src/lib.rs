// 347. Top K Frequent Elements
// Medium
//
// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
//
//
// Example 1:
//
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]
//
// Example 2:
//
// Input: nums = [1], k = 1
// Output: [1]
//
// Constraints:
// * 1 <= nums.length <= 10^5
// * k is in the range [1, the number of unique elements in the array].
// * It is guaranteed that the answer is unique.

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::with_capacity(nums.len());

        for n in nums {
            *counts.entry(n).or_insert(0) += 1;
        }

        let mut result: Vec<_> = counts.iter().collect();
        result.sort_by(|a, b| b.1.cmp(a.1));

        result[0..(k as usize)].iter().map(|tup| *tup.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_top_k_frequent_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let output = vec![1, 2];

        let mut result = Solution::top_k_frequent(nums, k);
        result.sort();
        assert_eq!(result, output);
    }

    #[test]
    fn test_top_k_frequent_2() {
        let nums = vec![1];
        let k = 1;
        let output = vec![1];

        let mut result = Solution::top_k_frequent(nums, k);
        result.sort();
        assert_eq!(result, output);
    }
}
