// 414. Third Maximum Number
// Easy
//
// Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.
//
//
// Example 1:
//
// Input: nums = [3,2,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2.
// The third distinct maximum is 1.
//
// Example 2:
//
// Input: nums = [1,2]
// Output: 2
// Explanation:
// The first distinct maximum is 2.
// The second distinct maximum is 1.
// The third distinct maximum does not exist, so the maximum (2) is returned instead.
//
// Example 3:
//
// Input: nums = [2,2,3,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2 (both 2's are counted together since they have the same value).
// The third distinct maximum is 1.
//
//
// Constraints:
//
// * 1 <= nums.length <= 10^4
// * -2^31 <= nums[i] <= 2^31 - 1
pub struct Solution {}
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut ranking = vec![None, None, None];

        for n in nums {
            let nn = Some(n);
            if nn > ranking[0] {
                ranking.swap(1, 2);
                ranking.swap(0, 1);
                ranking[0] = nn;
            } else if nn < ranking[0] && nn > ranking[1] {
                ranking.swap(1, 2);
                ranking[1] = nn;
            } else if nn < ranking[1] && nn > ranking[2] {
                ranking[2] = nn;
            }
        }

        if ranking[2].is_none() {
            ranking[0].unwrap()
        } else {
            ranking[2].unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_third_max_1() {
        let nums = vec![3, 2, 1];
        let output = 1;
        assert_eq!(Solution::third_max(nums), output);
    }

    #[test]
    fn test_third_max_2() {
        let nums = vec![1, 2];
        let output = 2;
        assert_eq!(Solution::third_max(nums), output);
    }

    #[test]
    fn test_third_max_3() {
        let nums = vec![2, 2, 3, 1];
        let output = 1;
        assert_eq!(Solution::third_max(nums), output);
    }

    #[test]
    fn test_third_max_4() {
        let nums = vec![1, 2, std::i32::MIN];
        let output = std::i32::MIN;
        assert_eq!(Solution::third_max(nums), output);
    }
}
