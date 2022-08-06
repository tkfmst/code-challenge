/// 645. Set Mismatch
/// Easy
///
/// You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
///
/// You are given an integer array nums representing the data status of this set after the error.
///
/// Find the number that occurs twice and the number that is missing and return them in the form of an array.
///
///
/// Example 1:
///
/// Input: nums = [1,2,2,4]
/// Output: [2,3]
///
/// Example 2:
///
/// Input: nums = [1,1]
/// Output: [1,2]
///
///
/// Constraints:
/// * 2 <= nums.length <= 10^4
/// * 1 <= nums[i] <= 10^4

pub struct Solution {}

use std::collections::HashSet;
// see https://leetcode.com/problems/set-mismatch/discuss/105528/Simple-Java-O(n)-solution-HashSet
// ![find_error_nums_using_set](./img/find_error_nums_using_set.png)
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::with_capacity(nums.len());
        let mut dup = -1;
        let mut sum = (nums.len() * (nums.len() + 1) / 2) as i32;

        for n in nums {
            if set.contains(&n) {
                dup = n;
            }
            sum -= n;
            set.insert(n);
        }

        vec![dup, sum + dup]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_error_nums_1() {
        let nums = vec![1, 2, 2, 4];
        let output = vec![2, 3];
        assert_eq!(Solution::find_error_nums(nums), output);
    }

    #[test]
    fn test_find_error_nums_2() {
        let nums = vec![1, 1];
        let output = vec![1, 2];
        assert_eq!(Solution::find_error_nums(nums), output);
    }

    #[test]
    fn test_find_error_nums_3() {
        let nums = vec![2, 2];
        let output = vec![2, 1];
        assert_eq!(Solution::find_error_nums(nums), output);
    }

    #[test]
    fn test_find_error_nums_4() {
        let nums = vec![3, 2, 2];
        let output = vec![2, 1];
        assert_eq!(Solution::find_error_nums(nums), output);
    }

    #[test]
    fn test_find_error_nums_5() {
        let nums = vec![3, 2, 3, 4, 6, 5];
        let output = vec![3, 1];
        assert_eq!(Solution::find_error_nums(nums), output);
    }
}
