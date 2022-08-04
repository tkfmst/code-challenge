/// 643. Maximum Average Subarray I
/// Easy
///
/// You are given an integer array nums consisting of n elements, and an integer k.
///
/// Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.
///
///
/// Example 1:
///
/// Input: nums = [1,12,-5,-6,50,3], k = 4
/// Output: 12.75000
/// Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
///
/// Example 2:
///
/// Input: nums = [5], k = 1
/// Output: 5.00000
///
///
/// Constraints:
/// * n == nums.length
/// * 1 <= k <= n <= 10^5
/// * -10^4 <= nums[i] <= 10^4

pub struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        let mut cur_sum: i32 = nums[..k].iter().sum();
        let mut max_sum = cur_sum;

        for i in 1..=(nums.len() - k) {
            cur_sum = cur_sum - nums[i - 1] + nums[i + k - 1];
            max_sum = cur_sum.max(max_sum);
        }

        (max_sum as f64) / (k as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_max_average_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let output = 12.75;
        assert_eq!(Solution::find_max_average(nums, k), output);
    }

    #[test]
    fn test_find_max_average_2() {
        let nums = vec![5];
        let k = 1;
        let output = 5.0;
        assert_eq!(Solution::find_max_average(nums, k), output);
    }

    #[test]
    fn test_find_max_average_3() {
        let nums = vec![-1];
        let k = 1;
        let output = -1.0;
        assert_eq!(Solution::find_max_average(nums, k), output);
    }
}
