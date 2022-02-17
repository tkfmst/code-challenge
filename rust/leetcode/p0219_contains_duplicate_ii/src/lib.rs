/// 219. Contains Duplicate II
/// Easy
///
/// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.
///
/// Example 1:
///
/// Input: nums = [1,2,3,1], k = 3
/// Output: true
///
/// Example 2:
///
/// Input: nums = [1,0,1,1], k = 1
/// Output: true
///
/// Example 3:
///
/// Input: nums = [1,2,3,1,2,3], k = 2
/// Output: false
///
///  
///
/// Constraints:
/// * 1 <= nums.length <= 10^5
/// * -10^9 <= nums[i] <= 10^9
/// * 0 <= k <= 10^5

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut dp: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let k = k as usize;
        for (j, n) in nums.iter().enumerate() {
            if let Some(&i) = dp.get(n) {
                if j - i <= k {
                    return true;
                }
            }
            dp.insert(*n, j);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_contains_nearby_duplicate_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let output = true;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), output)
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let output = true;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), output)
    }

    #[test]
    fn test_contains_nearby_duplicate_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let output = false;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), output)
    }
}
