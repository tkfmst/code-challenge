// 81. Search in Rotated Sorted Array II
// Medium
//
// There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).
//
// Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].
//
// Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.
//
// You must decrease the overall operation steps as much as possible.
//
//
// Example 1:
//
// Input: nums = [2,5,6,0,0,1,2], target = 0
// Output: true
//
// Example 2:
//
// Input: nums = [2,5,6,0,0,1,2], target = 3
// Output: false
//
//
// Constraints:
//
// * 1 <= nums.length <= 5000
// * -10^4 <= nums[i] <= 10^4
// * nums is guaranteed to be rotated at some pivot.
// * -10^4 <= target <= 10^4

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // // use api
        // nums.contains(&target)

        Self::helper(&nums, target)
    }
    fn helper(nums: &[i32], target: i32) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let mid = (nums.len() / 2) as usize;
        if nums[mid] == target {
            true
        } else {
            Self::helper(&nums[0..mid], target)
                || Self::helper(&nums[(mid + 1)..nums.len()], target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let output = true;
        assert_eq!(Solution::search(nums, target), output);
    }

    #[test]
    fn test_search_2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        let output = false;
        assert_eq!(Solution::search(nums, target), output);
    }

    #[test]
    fn test_search_3() {
        let nums = vec![1, 0, 1, 1, 1];
        let target = 0;
        let output = true;
        assert_eq!(Solution::search(nums, target), output);
    }
}
