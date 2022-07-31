/// 628. Maximum Product of Three Numbers
/// Easy
///
/// Given an integer array nums, find three numbers whose product is maximum and return the maximum product.
///
///
/// Example 1:
///
/// Input: nums = [1,2,3]
/// Output: 6
///
/// Example 2:
///
/// Input: nums = [1,2,3,4]
/// Output: 24
///
/// Example 3:
///
/// Input: nums = [-1,-2,-3]
/// Output: -6
///
///
/// Constraints:
///
/// * 3 <= nums.length <= 104
/// * -1000 <= nums[i] <= 1000
pub struct Solution {}
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let nums = {
            let mut ns = nums;
            ns.sort();
            ns
        };
        let len = nums.len();
        std::cmp::max(
            nums[len - 1] * nums[len - 2] * nums[len - 3],
            nums[0] * nums[1] * nums[len - 1],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_maximum_product_1() {
        let nums = vec![1, 2, 3];
        let output = 6;
        assert_eq!(Solution::maximum_product(nums), output);
    }

    #[test]
    fn test_maximum_product_2() {
        let nums = vec![1, 2, 3, 4];
        let output = 24;
        assert_eq!(Solution::maximum_product(nums), output);
    }

    #[test]
    fn test_maximum_product_3() {
        let nums = vec![-1, -2, -3];
        let output = -6;
        assert_eq!(Solution::maximum_product(nums), output);
    }

    #[test]
    fn test_maximum_product_4() {
        let nums = vec![-100, -98, -1, 2, 3, 4];
        let output = 39200;
        assert_eq!(Solution::maximum_product(nums), output);
    }
}
