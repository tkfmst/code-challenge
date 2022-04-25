/// 496. Next Greater Element I
/// Easy
///
/// The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
///
/// You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
///
/// For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
///
/// Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.
///
///
/// Example 1:
///
/// Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
/// Output: [-1,3,-1]
/// Explanation: The next greater element for each value of nums1 is as follows:
/// - 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
/// - 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
/// - 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
///
/// Example 2:
///
/// Input: nums1 = [2,4], nums2 = [1,2,3,4]
/// Output: [3,-1]
/// Explanation: The next greater element for each value of nums1 is as follows:
/// - 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
/// - 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.
///
///
/// Constraints:
/// * 1 <= nums1.length <= nums2.length <= 1000
/// * 0 <= nums1[i], nums2[i] <= 104
/// * All integers in nums1 and nums2 are unique.
/// * All the integers of nums1 also appear in nums2.

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // answer 2
        let mut results = vec![];
        let mut stack = vec![];
        let mut map = HashMap::with_capacity(1000);

        for n in nums2 {
            let mut top = stack.last().cloned();
            while top.is_some() && n > top.unwrap() {
                map.insert(stack.pop().unwrap(), n);
                top = stack.last().cloned();
            }
            stack.push(n);
        }

        for n in nums1 {
            match map.get(&n) {
                Some(&n2) => results.push(n2),
                None => results.push(-1),
            }
        }

        results

        // // answer 1
        // let mut results = vec![];
        //
        // let mut hash = HashMap::with_capacity(1000);
        // for (i, n) in nums2.iter().enumerate() {
        //     hash.insert(n, i);
        // }
        //
        // 'outer: for n in nums1 {
        //     println!("n: {:?}, hash[&n]: {:?}", n, hash[&n]);
        //     let i = hash[&n];
        //     for ii in i..nums2.len() {
        //         if nums2[ii] > n {
        //             results.push(nums2[ii]);
        //             continue 'outer;
        //         }
        //     }
        //     results.push(-1);
        // }
        //
        // results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_greater_element_1() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let output = vec![-1, 3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), output);
    }

    #[test]
    fn test_next_greater_element_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let output = vec![3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), output);
    }
}
