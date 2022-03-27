/// 704. Binary Search
/// Easy
///
/// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
///  
///
/// Example 1:
///
/// Input: nums = [-1,0,3,5,9,12], target = 9
/// Output: 4
/// Explanation: 9 exists in nums and its index is 4
///
/// Example 2:
///
/// Input: nums = [-1,0,3,5,9,12], target = 2
/// Output: -1
/// Explanation: 2 does not exist in nums so return -1
///
///  
///
/// Constraints:
///
/// * 1 <= nums.length <= 10^4
/// * -10^4 < nums[i], target < 10^4
/// * All the integers in nums are unique.
/// * nums is sorted in ascending order.

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(&nums, target, 0)
    }
    fn helper(nums: &[i32], target: i32, idx: usize) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return idx as i32;
            } else {
                return -1;
            }
        }

        let half = match nums.len() % 2 == 0 {
            true => nums.len() / 2,
            false => nums.len() / 2 + 1,
        };

        if nums[half] > target {
            Self::helper(&nums[0..half], target, idx)
        } else {
            Self::helper(&nums[half..nums.len()], target, idx + half)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let output = 4;
        assert_eq!(Solution::search(nums, target), output);
    }

    #[test]
    fn test_search_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let output = -1;
        assert_eq!(Solution::search(nums, target), output);
    }
}
