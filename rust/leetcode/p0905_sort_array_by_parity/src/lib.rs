/// 905. Sort Array By Parity
/// Easy
///
/// Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
///
/// Return any array that satisfies this condition.
///
/// Example 1:
///
/// Input: nums = [3,1,2,4]
/// Output: [2,4,3,1]
/// Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
///
/// Example 2:
///
/// Input: nums = [0]
/// Output: [0]
///
///
/// Constraints:
///
/// * 1 <= nums.length <= 5000
/// * 0 <= nums[i] <= 5000

pub struct Solution {}
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        let mut c1 = 0;
        let mut c2 = 0;

        'outer: while c2 < result.len() {
            if result[c2] % 2 == 1 {
                c2 += 1;
                continue;
            }

            while result[c1] % 2 == 0 {
                if c1 >= c2 {
                    c2 += 1;
                    continue 'outer;
                }
                c1 += 1;
            }

            result.swap(c1, c2);
            c1 += 1;
            c2 += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sort_array_by_parity_1() {
        let nums = vec![3, 1, 2, 4];
        let output = vec![2, 4, 3, 1];
        assert_eq!(Solution::sort_array_by_parity(nums), output);
    }

    #[test]
    fn test_sort_array_by_parity_2() {
        let nums = vec![0];
        let output = vec![0];
        assert_eq!(Solution::sort_array_by_parity(nums), output);
    }

    #[test]
    fn test_sort_array_by_parity_3() {
        let nums = vec![0, 1, 2];
        let output = vec![0, 2, 1];
        assert_eq!(Solution::sort_array_by_parity(nums), output);
    }
}
