/// 350. Intersection of Two Arrays II
/// Easy
///
/// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
///
///
/// Example 1:
///
/// Input: nums1 = [1,2,2,1], nums2 = [2,2]
/// Output: [2,2]
///
/// Example 2:
///
/// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// Output: [4,9]
/// Explanation: [9,4] is also accepted.
///
/// Constraints:
/// * 1 <= nums1.length, nums2.length <= 1000
/// * 0 <= nums1[i], nums2[i] <= 1000
///
/// Follow up:
/// * What if the given array is already sorted? How would you optimize your algorithm?
/// * What if nums1's size is small compared to nums2's size? Which algorithm is better?
/// * What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?

pub struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // answer2
        let mut acc = vec![];
        let mut record = [0; 1001];
        for n1 in nums1 {
            record[n1 as usize] += 1;
        }
        for n2 in nums2 {
            if record[n2 as usize] > 0 {
                record[n2 as usize] -= 1;
                acc.push(n2);
            }
        }
        acc

        // // answer 1
        // let mut acc = vec![];
        //
        // let mut nums1_clone = nums1.clone();
        // nums1_clone.sort();
        //
        // let mut nums2_clone = nums2.clone();
        // nums2_clone.sort();
        // let mut it2 = nums2_clone.iter().peekable();
        //
        // for n in nums1_clone {
        //     println!("n1:{:?}", n);
        //     while let Some(&&n2) = it2.peek() {
        //         println!("n2:{:?}", n2);
        //         if n == n2 {
        //             acc.push(n);
        //             it2.next();
        //             break;
        //         } else if n > n2 {
        //             it2.next();
        //         } else {
        //             break;
        //         }
        //     }
        // }
        //
        // acc
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_intersect_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];

        let mut actual = Solution::intersect(nums1, nums2);
        let mut expected = vec![2, 2];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intersect_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let mut actual = Solution::intersect(nums1, nums2);
        let mut expected = vec![4, 9];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intersect_3() {
        let nums1 = vec![2, 2];
        let nums2 = vec![2];

        let mut actual = Solution::intersect(nums1, nums2);
        let mut expected = vec![2];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }
}
