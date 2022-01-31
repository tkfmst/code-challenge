/// 167. Two Sum II - Input Array Is Sorted
/// Easy
///
/// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
///
/// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
///
/// The tests are generated such that there is exactly one solution. You may not use the same element twice.
///
///
///
/// Example 1:
///
/// Input: numbers = [2,7,11,15], target = 9
/// Output: [1,2]
/// Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
///
/// Example 2:
///
/// Input: numbers = [2,3,4], target = 6
/// Output: [1,3]
/// Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
///
/// Example 3:
///
/// Input: numbers = [-1,0], target = -1
/// Output: [1,2]
/// Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
///
/// Constraints:
/// * 2 <= numbers.length <= 3 * 104
/// * -1000 <= numbers[i] <= 1000
/// * numbers is sorted in non-decreasing order.
/// * -1000 <= target <= 1000
/// * The tests are generated such that there is exactly one solution.

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i1: usize = 0;
        let mut i2: usize = numbers.len() - 1;

        while numbers[i1] + numbers[i2] != target {
            if numbers[i1] + numbers[i2] < target {
                i1 += 1;
            } else {
                i2 -= 1;
            }
        }

        vec![(i1 + 1) as i32, (i2 + 1) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_two_sum_1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(input, target), output);
    }

    #[test]
    fn it_two_sum_2() {
        let input = vec![2, 3, 4];
        let target = 6;
        let output = vec![1, 3];
        assert_eq!(Solution::two_sum(input, target), output);
    }

    #[test]
    fn it_two_sum_3() {
        let input = vec![-1, 0];
        let target = -1;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(input, target), output);
    }
}
