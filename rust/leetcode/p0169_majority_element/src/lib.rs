/// 169. Majority Element
///
/// Given an array nums of size n, return the majority element.
///
/// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
///
///
/// Example 1:
///
/// Input: nums = [3,2,3]
/// Output: 3
///
/// Example 2:
///
/// Input: nums = [2,2,1,1,1,2,2]
/// Output: 2
///
///
///
/// Constraints:
///
/// * n == nums.length
/// * 1 <= n <= 5 * 10^4
/// * -2^31 <= nums[i] <= 2^31 - 1
///
/// Follow-up: Could you solve the problem in linear time and in O(1) space?

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major = -1;
        let mut count = 0;
        for n in nums {
            if count == 0 {
                major = n;
                count = 1;
            } else if major == n {
                count += 1;
            } else {
                count -= 1;
            }
        }
        major
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_majority_element_1() {
        let input = vec![3, 2, 3];
        let output = 3;
        assert_eq!(Solution::majority_element(input), output);
    }

    #[test]
    fn test_majority_element_2() {
        let input = vec![2, 2, 1, 1, 1, 2, 2];
        let output = 2;
        assert_eq!(Solution::majority_element(input), output);
    }
}
