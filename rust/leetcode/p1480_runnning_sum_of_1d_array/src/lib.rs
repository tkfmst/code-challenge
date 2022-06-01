/// 1480. Running Sum of 1d Array
/// Easy
///
/// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
///
/// Return the running sum of nums.
///
///
/// Example 1:
///
/// Input: nums = [1,2,3,4]
/// Output: [1,3,6,10]
/// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
///
/// Example 2:
///
/// Input: nums = [1,1,1,1,1]
/// Output: [1,2,3,4,5]
/// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
///
/// Example 3:
///
/// Input: nums = [3,1,2,10,1]
/// Output: [3,4,6,16,17]
///
///
/// Constraints:
/// * 1 <= nums.length <= 1000
/// * -10^6 <= nums[i] <= 10^6

pub struct Solution {}
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len() + 1);
        res.push(nums[0]);
        for n in nums[1..].iter() {
            res.push(res.last().unwrap() + n);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_running_sum_1() {
        let nums = vec![1, 2, 3, 4];
        let output = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(nums), output);
    }

    #[test]
    fn test_running_sum_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let output = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(nums), output);
    }

    #[test]
    fn test_running_sum_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let output = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(nums), output);
    }
}
