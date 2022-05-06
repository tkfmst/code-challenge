/// 1209. Remove All Adjacent Duplicates in String II
/// Medium
///
/// You are given a string s and an integer k, a k duplicate removal consists of choosing k adjacent and equal letters from s and removing them, causing the left and the right side of the deleted substring to concatenate together.
///
/// We repeatedly make k duplicate removals on s until we no longer can.
///
/// Return the final string after all such duplicate removals have been made. It is guaranteed that the answer is unique.
///
///
/// Example 1:
///
/// Input: s = "abcd", k = 2
/// Output: "abcd"
/// Explanation: There's nothing to delete.
///
/// Example 2:
///
/// Input: s = "deeedbbcccbdaa", k = 3
/// Output: "aa"
/// Explanation:
/// First delete "eee" and "ccc", get "ddbbbdaa"
/// Then delete "bbb", get "dddaa"
/// Finally delete "ddd", get "aa"
///
/// Example 3:
///
/// Input: s = "pbbcggttciiippooaais", k = 2
/// Output: "ps"
///
///
/// Constraints:
/// * 1 <= s.length <= 10^5
/// * 2 <= k <= 10^4
/// * s only contains lower case English letters.

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut i = 0;
        let mut stack = s.chars().collect::<Vec<_>>();
        let mut cnt: HashMap<usize, usize> = HashMap::with_capacity(s.len());

        for j in 0..s.len() {
            stack[i] = stack[j];

            if i > 0 && stack[i - 1] == stack[j] {
                let prev = cnt[&(i - 1)];
                let e = cnt.entry(i).or_insert(0);
                *e = prev + 1;
            } else {
                let e = cnt.entry(i).or_insert(0);
                *e = 1;
            }

            if cnt[&i] == (k as usize) {
                i = match i.checked_sub(k as usize) {
                    Some(ii) => ii + 1,
                    None => 0,
                }
            } else {
                i += 1;
            }
        }

        stack.drain(0..i).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_duplicates_1() {
        let s = "abcd".to_string();
        let k = 2;
        let output = "abcd".to_string();
        assert_eq!(Solution::remove_duplicates(s, k), output);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let s = "deeedbbcccbdaa".to_string();
        let k = 3;
        let output = "aa".to_string();
        assert_eq!(Solution::remove_duplicates(s, k), output);
    }

    #[test]
    fn test_remove_duplicates_3() {
        let s = "pbbcggttciiippooaais".to_string();
        let k = 2;
        let output = "ps".to_string();
        assert_eq!(Solution::remove_duplicates(s, k), output);
    }

    #[test]
    fn test_remove_duplicates_4() {
        let s = "yfttttfbbbbnnnnffbgffffgbbbbgssssgthyyyy".to_string();
        let k = 4;
        let output = "ybth".to_string();
        assert_eq!(Solution::remove_duplicates(s, k), output);
    }
}
