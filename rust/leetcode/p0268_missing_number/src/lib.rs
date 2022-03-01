/// 268. Missing Number
/// Easy
///
/// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
///
///
///
/// Example 1:
///
/// Input: nums = [3,0,1]
/// Output: 2
/// Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.
///
/// Example 2:
///
/// Input: nums = [0,1]
/// Output: 2
/// Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.
///
/// Example 3:
///
/// Input: nums = [9,6,4,2,3,5,7,0,1]
/// Output: 8
/// Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.
///
///
///
/// Constraints:
///
/// * n == nums.length
/// * 1 <= n <= 10^4
/// * 0 <= nums[i] <= n
/// * All the numbers of nums are unique.

pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        (len + 1) * len / 2 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_missing_number_1() {
        let nums = vec![3, 0, 1];
        let output = 2;
        assert_eq!(Solution::missing_number(nums), output);
    }

    #[test]
    fn test_missing_number_2() {
        let nums = vec![0, 1];
        let output = 2;
        assert_eq!(Solution::missing_number(nums), output);
    }

    #[test]
    fn test_missing_number_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let output = 8;
        assert_eq!(Solution::missing_number(nums), output);
    }
}
