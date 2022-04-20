/// 485. Max Consecutive Ones
/// Easy
///
/// Given a binary array nums, return the maximum number of consecutive 1's in the array.
///
/// Example 1:
///
/// Input: nums = [1,1,0,1,1,1]
/// Output: 3
/// Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
///
/// Example 2:
///
/// Input: nums = [1,0,1,1,0,1]
/// Output: 2
///
///
/// Constraints:
///
/// * 1 <= nums.length <= 10^5
/// * nums[i] is either 0 or 1.

pub struct Solution {}
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&i| i == 0)
            .collect::<Vec<_>>()
            .iter()
            .map(|v| v.len())
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_max_consecutive_ones_1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        let output = 3;
        assert_eq!(Solution::find_max_consecutive_ones(nums), output);
    }

    #[test]
    fn test_find_max_consecutive_ones_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let output = 2;
        assert_eq!(Solution::find_max_consecutive_ones(nums), output);
    }

    #[test]
    fn test_find_max_consecutive_ones_a() {
        let nums = vec![0];
        let output = 0;
        assert_eq!(Solution::find_max_consecutive_ones(nums), output);
    }

    #[test]
    fn test_find_max_consecutive_ones_b() {
        let nums = vec![1];
        let output = 1;
        assert_eq!(Solution::find_max_consecutive_ones(nums), output);
    }
}
