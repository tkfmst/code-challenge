/// 88. Merge Sorted Array
///
/// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
///
/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
///
/// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
///
/// Example 1:
///
/// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// Output: [1,2,2,3,5,6]
/// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
/// The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
///
/// Example 2:
///
/// Input: nums1 = [1], m = 1, nums2 = [], n = 0
/// Output: [1]
/// Explanation: The arrays we are merging are [1] and [].
/// The result of the merge is [1].
///
/// Example 3:
///
/// Input: nums1 = [0], m = 0, nums2 = [1], n = 1
/// Output: [1]
/// Explanation: The arrays we are merging are [] and [1].
/// The result of the merge is [1].
/// Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.

pub struct Solution {}

impl Solution {
    #[allow(unused_variables)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // // Solve by rust Vec api
        // nums1.truncate(m as usize);
        // nums1.append(nums2);
        // nums1.sort();

        let m = m as usize;

        let _nums1 = nums1.clone();
        let mut it1 = _nums1[0..m].iter().peekable();
        let mut it2 = nums2.iter().peekable();
        let mut i = 0;

        loop {
            let lesser = match (it1.peek(), it2.peek()) {
                (None, None) => {
                    break;
                }
                (Some(_), None) => &mut it1,
                (None, Some(_)) => &mut it2,
                (Some(&&e1), Some(&&e2)) => {
                    if e1 <= e2 {
                        &mut it1
                    } else {
                        &mut it2
                    }
                }
            };

            nums1[i] = *(lesser.next().unwrap());
            i += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_merge1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6],);
    }
    #[test]
    fn test_merge2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1],);
        // assert_eq!(Solution::merge(vec![0], 0, vec![1], 1), vec![1],);
    }
    #[test]
    fn test_merge3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1],);
    }
}
