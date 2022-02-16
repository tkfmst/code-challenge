/// 217. Contains Duplicate
/// Easy
///
/// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
///
///  
///
/// Example 1:
///
/// Input: nums = [1,2,3,1]
/// Output: true
///
/// Example 2:
///
/// Input: nums = [1,2,3,4]
/// Output: false
///
/// Example 3:
///
/// Input: nums = [1,1,1,3,3,4,3,2,4,2]
/// Output: true
///
///  
///
/// Constraints:
/// * 1 <= nums.length <= 10^5
/// * -10^9 <= nums[i] <= 10^9
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        for n in nums {
            if hs.contains(&n) {
                return true;
            }
            hs.insert(n);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_contains_duplicate_1() {
        let input = vec![1, 2, 3, 1];
        let output = true;
        assert_eq!(Solution::contains_duplicate(input), output);
    }

    #[test]
    fn test_contains_duplicate_2() {
        let input = vec![1, 2, 3, 4];
        let output = false;
        assert_eq!(Solution::contains_duplicate(input), output);
    }

    #[test]
    fn test_contains_duplicate_3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let output = true;
        assert_eq!(Solution::contains_duplicate(input), output);
    }
}
