/// 448. Find All Numbers Disappeared in an Array
/// Easy
///
/// Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
///
///
/// Example 1:
///
/// Input: nums = [4,3,2,7,8,2,3,1]
/// Output: [5,6]
///
/// Example 2:
///
/// Input: nums = [1,1]
/// Output: [2]
///
///
/// Constraints:
/// * n == nums.length
/// * 1 <= n <= 10^5
/// * 1 <= nums[i] <= n

pub struct Solution {}
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ns = nums.clone();
        for i in 0..ns.len() {
            let idx = ns[i].abs() as usize - 1;
            ns[idx] = -ns[idx].abs();
        }
        ns.iter()
            .enumerate()
            .filter(|(_, &n)| n > 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_disappeared_numbers_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let output = vec![5, 6];
        assert_eq!(Solution::find_disappeared_numbers(nums), output);
    }

    #[test]
    fn test_find_disappeared_numbers_2() {
        let nums = vec![1, 1];
        let output = vec![2];
        assert_eq!(Solution::find_disappeared_numbers(nums), output);
    }
}
