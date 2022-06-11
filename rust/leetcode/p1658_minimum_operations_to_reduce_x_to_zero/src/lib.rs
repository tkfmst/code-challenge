/// 1658. Minimum Operations to Reduce X to Zero
/// Medium
///
/// You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.
///
/// Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.
///
///
/// Example 1:
///
/// Input: nums = [1,1,4,2,3], x = 5
/// Output: 2
/// Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
///
/// Example 2:
///
/// Input: nums = [5,6,7,8,9], x = 4
/// Output: -1
///
/// Example 3:
///
/// Input: nums = [3,2,20,1,1,3], x = 10
/// Output: 5
/// Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
///
///
/// Constraints:
/// * 1 <= nums.length <= 105
/// * 1 <= nums[i] <= 104
/// * 1 <= x <= 109

pub struct Solution {}
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        let n = nums.len();
        let mut maxi: i32 = -1;
        let mut left = 0;
        let mut cur = 0;

        for right in 0..n {
            cur += nums[right];
            while cur > total - x && left <= right {
                cur -= nums[left];
                left += 1;
            }
            if cur == total - x {
                maxi = maxi.max((right as i32) - (left as i32) + 1);
            }
        }

        return if maxi != -1 { (n as i32) - maxi } else { -1 };
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = vec![1, 1, 4, 2, 3];
        let x = 5;
        let output = 2;
        assert_eq!(Solution::min_operations(nums, x), output);
    }

    #[test]
    fn test_min_operations_2() {
        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;
        let output = -1;
        assert_eq!(Solution::min_operations(nums, x), output);
    }

    #[test]
    fn test_min_operations_3() {
        let nums = vec![3, 2, 20, 1, 1, 3];
        let x = 10;
        let output = 5;
        assert_eq!(Solution::min_operations(nums, x), output);
    }
}
