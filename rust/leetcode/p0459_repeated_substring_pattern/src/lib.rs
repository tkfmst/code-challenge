/// 459. Repeated Substring Pattern
/// Easy
///
/// Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
///
///  
///
/// Example 1:
///
/// Input: s = "abab"
/// Output: true
/// Explanation: It is the substring "ab" twice.
///
/// Example 2:
///
/// Input: s = "aba"
/// Output: false
///
/// Example 3:
///
/// Input: s = "abcabcabcabc"
/// Output: true
/// Explanation: It is the substring "abc" four times or the substring "abcabc" twice.
///
///  
///
/// Constraints:
/// * 1 <= s.length <= 10^4
/// * s consists of lowercase English letters.

pub struct Solution {}
impl Solution {
    // See below for this answer
    // https://leetcode.com/problems/repeated-substring-pattern/discuss/441613/Easiest-Python-Solution-with-detailed-explanation-and-example.
    pub fn repeated_substring_pattern(s: String) -> bool {
        s.repeat(2)[1..(s.len() * 2 - 1)].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_repeated_substring_pattern_1() {
        let s = "abab".to_string();
        let output = true;
        assert_eq!(Solution::repeated_substring_pattern(s), output);
    }

    #[test]
    fn test_repeated_substring_pattern_2() {
        let s = "aba".to_string();
        let output = false;
        assert_eq!(Solution::repeated_substring_pattern(s), output);
    }

    #[test]
    fn test_repeated_substring_pattern_3() {
        let s = "abcabcabcabc".to_string();
        let output = true;
        assert_eq!(Solution::repeated_substring_pattern(s), output);
    }

    #[test]
    fn test_repeated_substring_pattern_a() {
        let s = "abaabaaba".to_string();
        let output = true;
        assert_eq!(Solution::repeated_substring_pattern(s), output);
    }

    #[test]
    fn test_repeated_substring_pattern_4() {
        let s = "ababab".to_string();
        let output = true;
        assert_eq!(Solution::repeated_substring_pattern(s), output);
    }
}
