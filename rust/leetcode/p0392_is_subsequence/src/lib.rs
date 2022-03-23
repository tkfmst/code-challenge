// 392. Is Subsequence
// Easy
//
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
//
// Example 1:
//
// Input: s = "abc", t = "ahbgdc"
// Output: true
//
// Example 2:
//
// Input: s = "axc", t = "ahbgdc"
// Output: false
//
//
// Constraints:
//
// * 0 <= s.length <= 100
// * 0 <= t.length <= 10^4
// * s and t consist only of lowercase English letters.

pub struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut it1 = s.chars();
        let mut it2 = t.chars();

        while let Some(c1) = it1.next() {
            loop {
                if let Some(c2) = it2.next() {
                    if c1 == c2 {
                        break;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_subsequence_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let output = true;
        assert_eq!(Solution::is_subsequence(s, t), output);
    }

    #[test]
    fn test_is_subsequence_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let output = false;
        assert_eq!(Solution::is_subsequence(s, t), output);
    }
}
