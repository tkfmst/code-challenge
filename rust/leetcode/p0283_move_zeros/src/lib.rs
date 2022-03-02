/// 283. Move Zeroes
/// Easy
///
/// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
///
/// Note that you must do this in-place without making a copy of the array.
///
///  
///
/// Example 1:
///
/// Input: nums = [0,1,0,3,12]
/// Output: [1,3,12,0,0]
///
/// Example 2:
///
/// Input: nums = [0]
/// Output: [0]
///
/// Constraints:
///
/// 1 <= nums.length <= 10^4
/// -2^31 <= nums[i] <= 2^31 - 1
pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in (0..nums.len()).rev() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_move_zeroes_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_2() {
        let mut nums = vec![0];
        let expected = vec![0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, expected);
    }
}
