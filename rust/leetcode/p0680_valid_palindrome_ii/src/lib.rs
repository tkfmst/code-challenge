/// 680. Valid Palindrome II
/// Easy
///
/// Given a string s, return true if the s can be palindrome after deleting at most one character from it.
///
///
/// Example 1:
///
/// Input: s = "aba"
/// Output: true
///
/// Example 2:
///
/// Input: s = "abca"
/// Output: true
/// Explanation: You could delete the character 'c'.
///
/// Example 3:
///
/// Input: s = "abc"
/// Output: false
///
/// Constraints:
///
/// * 1 <= s.length <= 10^5
/// * s consists of lowercase English letters.

pub struct Solution {}
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        Self::is_palindrome(s.as_bytes(), false)
    }

    fn is_palindrome(bs: &[u8], removed: bool) -> bool {
        if bs.len() <= 1 {
            return true;
        }

        let head = 0;
        let last = bs.len() - 1;

        if bs[head] == bs[last] {
            Self::is_palindrome(&bs[(head + 1)..=(last - 1)], removed)
        } else {
            if removed {
                return false;
            }

            return Self::is_palindrome(&bs[(head + 1)..=last], true)
                || Self::is_palindrome(&bs[head..=(last - 1)], true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_palindrome_1() {
        let s = "aba".to_string();
        let output = true;
        assert_eq!(Solution::valid_palindrome(s), output);
    }

    #[test]
    fn test_valid_palindrome_2() {
        let s = "abca".to_string();
        let output = true;
        assert_eq!(Solution::valid_palindrome(s), output);
    }

    #[test]
    fn test_valid_palindrome_3() {
        let s = "abc".to_string();
        let output = false;
        assert_eq!(Solution::valid_palindrome(s), output);
    }

    #[test]
    fn test_valid_palindrome_4() {
        let s = "tebbem".to_string();
        let output = false;
        assert_eq!(Solution::valid_palindrome(s), output);
    }

    #[test]
    fn test_valid_palindrome_5() {
        let s = "yd".to_string();
        let output = true;
        assert_eq!(Solution::valid_palindrome(s), output);
    }

    #[test]
    fn test_valid_palindrome_6() {
        let s = "aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string();
        let output = true;
        assert_eq!(Solution::valid_palindrome(s), output);
    }
}
