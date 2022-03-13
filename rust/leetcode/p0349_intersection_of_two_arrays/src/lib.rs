/// 349. Intersection of Two Arrays
/// Easy
///
/// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.
///
///  
///
/// Example 1:
///
/// Input: nums1 = [1,2,2,1], nums2 = [2,2]
/// Output: [2]
///
/// Example 2:
///
/// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// Output: [9,4]
/// Explanation: [4,9] is also accepted.
///
///  
///
/// Constraints:
///
/// * 1 <= nums1.length, nums2.length <= 1000
/// * 0 <= nums1[i], nums2[i] <= 1000
use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::with_capacity(if nums1.len() < nums2.len() {
            nums1.len()
        } else {
            nums2.len()
        });

        for n in nums1 {
            if nums2.contains(&n) && !set.contains(&n) {
                set.insert(n);
            }
        }
        set.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_intersection_1() {
        let num1 = vec![1, 2, 2, 1];
        let num2 = vec![2, 2];

        let mut actual = Solution::intersection(num1, num2);
        let mut expected = vec![2];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intersection_2() {
        let num1 = vec![4, 9, 5];
        let num2 = vec![9, 4, 9, 8, 4];

        let mut actual = Solution::intersection(num1, num2);
        let mut expected = vec![4, 9];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }
}
