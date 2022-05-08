/// 456. 132 Pattern
/// Medium
///
/// Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
///
/// Return true if there is a 132 pattern in nums, otherwise, return false.
///
///
/// Example 1:
///
/// Input: nums = [1,2,3,4]
/// Output: false
/// Explanation: There is no 132 pattern in the sequence.
///
/// Example 2:
///
/// Input: nums = [3,1,4,2]
/// Output: true
/// Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
///
/// Example 3:
///
/// Input: nums = [-1,3,2,0]
/// Output: true
/// Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
///
///
/// Constraints:
/// * -n == nums.length
/// * -1 <= n <= 2 * 10^5
/// * -10^9 <= nums[i] <= 10^9
pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut nums_k = std::i32::MIN;

        let mut stack = vec![];

        for _i in 0..nums.len() {
            let i = nums.len() - 1 - _i;
            println!(
                "i={:?}, nums[i]={:?}, nums_k={:?}, stack={:?}",
                i, nums[i], nums_k, stack
            );

            if nums[i] < nums_k {
                return true;
            }

            // stack is sorted by desc(large -> small)
            while let Some(n) = stack.last() {
                if nums[i] > *n {
                    // num_k is the second largest value at i
                    nums_k = *n;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(nums[i]);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[test]
    // fn test_find132pattern_1() {
    //     let nums = vec![1, 2, 3, 4];
    //     let output = false;
    //     assert_eq!(Solution::find132pattern(nums), output);
    // }
    //
    // #[test]
    // fn test_find132pattern_2() {
    //     let nums = vec![3, 1, 4, 2];
    //     let output = true;
    //     assert_eq!(Solution::find132pattern(nums), output);
    // }

    #[test]
    fn test_find132pattern_3() {
        let nums = vec![-1, 3, 2, 0];
        let output = true;
        assert_eq!(Solution::find132pattern(nums), output);
    }

    // #[test]
    // fn test_find132pattern_4() {
    //     let nums = vec![
    //         -10, -10, -9, -10, -10, -10, -10, -9, -10, -9, -10, -9, -10, -10, -10, -9, -9, -9, -10,
    //         -10, -10, -9, -10, -10, -10, -9, -9, -9, -9, -10, -10, -9, -9, -9, -10, -9, -10, -10,
    //         -10, -9, -10, -10, -9, -10, -10, -9, -9, -9, -10, -9, -10, -9, -10, -10, -9, -9, -10,
    //         -10, -9, -9, -9, -10, -9, -10, -10, -10, -9, -9, -10, -9, -10, -10, -9, -10, -10, -10,
    //         -10, -9, -10, -10, -10, -10, -10, -9, -9, -9, -10, -10, -9, -9, -9, -9, -9, -9, -10,
    //         -10, -10, -10, -9, -9, -10, -10, -10, -9, -9, -10, -9, -10, -10, -9, -9, -9, -10, -10,
    //         -10, -9, -10, -10, -10, -10, -9, -9, -9, -10, -10, -9, -9, -9, -10, -10, -9, -9, -10,
    //         -9, -10, -10, -9, -9, -9, -10, -9, -9, -9, -10, -10, -9, -9, -10, -10, -10, -9, -10,
    //         -10, -10, -10, -10, -10, -10, -10, -9, -10, -10, -9, -9, -9, -10, -10, -9, -9, -9, -10,
    //         -9, -9, -10, -9, -9, -10, -10, -9, -9, -10, -10, -10, -10, -10, -10, -9, -9, -9, -10,
    //         -10, -10, -9, -10, -9, -10, -10, -10, -10, -9, -9, -9, -10, -9, -9, -9, -9, -9, -9,
    //         -10, -10, -9, -9, -9, -10, -9, -10, -10, -10, -9, -9, -9, -9, -9, -10, -9, -10, -9, -9,
    //         -9, -9, -9, -9, -9, -10, -10, -10, -10, -9, -9, -10, -10, -9, -10, -10, -9, -9, -9, -9,
    //         -10, -10, -10, -9, -9, -9, -9, -10, -10, -9, -9, -10, -9, -9, -10, -9, -9, -10, -10,
    //         -10, -10, -10, -9, -10, -9, -9, -9, -9, -10, -10, -10, -9, -10, -10, -10, -10, -10, -9,
    //         -9, -10, -10, -10, -10, -9, -9, -9, -10, -9, -10, -10, -10, -9, -10, -9, -9, -10, -9,
    //         -9, -10, -9, -10, -10, -9, -9, -10, -9, -10, -10, -10, -10, -10, -9, -9, -10, -10, -10,
    //         -9, -10, -9, -10, -9, -10, -9, -9, -9, -10, -9, -10, -9, -9, -9, -10, -10, -10, -10,
    //         -10, -9, -10, -9, -10, -9, -9, -10, -10, -9, -9, -9, -10, -9, -10, -10, -10, -9, -10,
    //         -9, -10, -9, -10, -9, -10, -9, -10, -10, -10, -9, -10, -9, -9, -10, -10, -10, -10, -9,
    //         -10, -10, -9, -10, -10, -9, -10, -10, -10, -10, -9, -9, -9, -9, -10, -9, -9, -9, -9,
    //         -10, -9, -10, -10, -10, -10, -9, -10, -10, -10, -9, -9, -10, -9, -10, -10, -9, -9, -10,
    //         -9, -10, -9, -10, -9, -10, -9, -10, -9, -9, -9, -10, -10, -10, -10, -9, -9, -9, -10,
    //         -9, -9, -10, -9, -9, -9, -9, -9, -10, -9, -9, -9, -9, -10, -10, -10, -9, -9, -9, -9,
    //         -9, -10, -10, -9, -9, -10, -10, -10, -10, -10, -10, -9, -10, -10, -9, -9, -10, -10,
    //         -10, -9, -10, -9, -10, -10, -9, -9, -10, -10, -9, -10, -10, -9, -10, -10, -10, -9, -9,
    //         -10, -10, -10, -10, -10, -10, -10, -9, -9, -9, -10, -9, -10, -10, -10, -9, -10, -10,
    //         -9, -9, -10, -9, -9, -9, -10, -10, -9, -10, -9, -9,
    //     ];
    //     let output = false;
    //     assert_eq!(Solution::find132pattern(nums), output);
    // }
}