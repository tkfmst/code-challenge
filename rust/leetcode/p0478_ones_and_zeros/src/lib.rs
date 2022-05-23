// 474. Ones and Zeroes
// Medium
//
// You are given an array of binary strings strs and two integers m and n.
//
// Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
//
// A set x is a subset of a set y if all elements of x are also elements of y.
//
//
// Example 1:
//
// Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
// Output: 4
// Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
// Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
// {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
//
// Example 2:
//
// Input: strs = ["10","0","1"], m = 1, n = 1
// Output: 2
// Explanation: The largest subset is {"0", "1"}, so the answer is 2.
//
//
// Constraints:
// * 1 <= strs.length <= 600
// * 1 <= strs[i].length <= 100
// * strs[i] consists only of digits '0' and '1'.
// * 1 <= m, n <= 100

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
        dp.insert((0, 0), 0);
        for s in strs.iter() {
            let zero = s.chars().filter(|&c| c == '0').count() as i32;
            let one = s.len() as i32 - zero;
            let entries: Vec<((i32, i32), i32)> = dp.iter().map(|(&k, &v)| (k, v)).collect();

            for ((zero2, one2), size) in entries.iter() {
                let (zero_sum, one_sum) = (zero + zero2, one + one2);
                if zero_sum <= m && one_sum <= n {
                    let entry = dp.entry((zero_sum, one_sum)).or_insert(0);
                    *entry = (*entry).max(size + 1);
                }
            }
        }
        *dp.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_max_form_1() {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let m = 5;
        let n = 3;
        let output = 4;
        assert_eq!(Solution::find_max_form(strs, m, n), output);
    }

    #[test]
    fn test_find_max_form_2() {
        let strs = vec!["10".to_string(), "0".to_string(), "1".to_string()];
        let m = 1;
        let n = 1;
        let output = 2;
        assert_eq!(Solution::find_max_form(strs, m, n), output);
    }
}
