/// 228. Summary Ranges
/// Easy
///
/// You are given a sorted unique integer array nums.
///
/// Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
///
/// Each range [a,b] in the list should be output as:
///
/// * "a->b" if a != b
/// * "a" if a == b
///
///
///
/// Example 1:
///
/// Input: nums = [0,1,2,4,5,7]
/// Output: ["0->2","4->5","7"]
/// Explanation: The ranges are:
/// [0,2] --> "0->2"
/// [4,5] --> "4->5"
/// [7,7] --> "7"
///
/// Example 2:
///
/// Input: nums = [0,2,3,4,6,8,9]
/// Output: ["0","2->4","6","8->9"]
/// Explanation: The ranges are:
/// [0,0] --> "0"
/// [2,4] --> "2->4"
/// [6,6] --> "6"
/// [8,9] --> "8->9"
///
///
///
/// Constraints:
/// * 0 <= nums.length <= 20
/// * -2^31 <= nums[i] <= 2^31 - 1
/// * All the values of nums are unique.
/// * nums is sorted in ascending order.

pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut i1 = 0;

        while i1 < nums.len() {
            let mut i2 = i1;
            while i2 + 1 < nums.len() && nums[i2 + 1] - nums[i2] == 1 {
                i2 += 1;
            }

            if i1 == i2 {
                result.push(nums[i1].to_string());
            } else {
                result.push(format!(
                    "{}->{}",
                    nums[i1].to_string(),
                    nums[i2].to_string()
                ));
            }

            i1 = i2 + 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_summary_ranges_1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let output = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];
        assert_eq!(Solution::summary_ranges(nums), output);
    }

    #[test]
    fn test_summary_ranges_2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let output = vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string(),
        ];
        assert_eq!(Solution::summary_ranges(nums), output);
    }
}
