/// 581. Shortest Unsorted Continuous Subarray
/// Medium
///
/// Given an integer array nums, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order.
///
/// Return the shortest such subarray and output its length.
///
///
/// Example 1:
///
/// Input: nums = [2,6,4,8,10,9,15]
/// Output: 5
/// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
///
/// Example 2:
///
/// Input: nums = [1,2,3,4]
/// Output: 0
///
/// Example 3:
///
/// Input: nums = [1]
/// Output: 0
///
///
/// Constraints:
///
/// * 1 <= nums.length <= 10^4
/// * -10^5 <= nums[i] <= 10^5
///
/// Follow up: Can you solve it in O(n) time complexity?

pub struct Solution {}
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        // subarray = [i(start)...i(end)]
        let mut start = -1i32;
        let mut end = -2i32;

        let last = nums.len() - 1;

        let mut left = nums[0];
        let mut right = nums[last];

        // 1,2,, 4,5,6,3, 7,8,9
        for i in 0..nums.len() {
            left = left.max(nums[i]);
            right = right.min(nums[last - i]);
            if nums[i] < left {
                end = i as i32;
            }
            if nums[last - i] > right {
                start = (last - i) as i32;
            }
        }

        end - start + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_unsorted_subarray_1() {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        let output = 5;
        assert_eq!(Solution::find_unsorted_subarray(nums), output);
    }

    #[test]
    fn test_find_unsorted_subarray_2() {
        let nums = vec![1, 2, 3, 4];
        let output = 0;
        assert_eq!(Solution::find_unsorted_subarray(nums), output);
    }

    #[test]
    fn test_find_unsorted_subarray_3() {
        let nums = vec![1];
        let output = 0;
        assert_eq!(Solution::find_unsorted_subarray(nums), output);
    }

    #[test]
    fn test_find_unsorted_subarray_a() {
        let nums = vec![1, 2, 4, 5, 6, 3, 7, 8, 9];
        let output = 4;
        assert_eq!(Solution::find_unsorted_subarray(nums), output);
    }
}
