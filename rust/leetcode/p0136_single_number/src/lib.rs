/// 136. Single Number
///
/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
///
///  
///
/// Example 1:
///
/// Input: nums = [2,2,1]
/// Output: 1
///
/// Example 2:
///
/// Input: nums = [4,1,2,1,2]
/// Output: 4
///
/// Example 3:
///
/// Input: nums = [1]
/// Output: 1
///
///  
///
/// Constraints:
/// * 1 <= nums.length <= 3 * 10^4
/// * -3 * 10^4 <= nums[i] <= 3 * 10^4
/// * Each element in the array appears twice except for one element which appears only once.
///

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |xor, n| xor ^ n)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_single_number_1() {
        let input = vec![2, 2, 1];
        let output = 1;
        assert_eq!(Solution::single_number(input), output);
    }

    #[test]
    fn test_single_number_2() {
        let input = vec![4, 1, 2, 1, 2];
        let output = 4;
        assert_eq!(Solution::single_number(input), output);
    }

    #[test]
    fn test_single_number_3() {
        let input = vec![1];
        let output = 1;
        assert_eq!(Solution::single_number(input), output);
    }
}
