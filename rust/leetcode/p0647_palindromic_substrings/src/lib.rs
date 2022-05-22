/// 647. Palindromic Substrings
/// Medium
///
/// Given a string s, return the number of palindromic substrings in it.
///
/// A string is a palindrome when it reads the same backward as forward.
///
/// A substring is a contiguous sequence of characters within the string.
///
///
/// Example 1:
///
/// Input: s = "abc"
/// Output: 3
/// Explanation: Three palindromic strings: "a", "b", "c".
///
/// Example 2:
///
/// Input: s = "aaa"
/// Output: 6
/// Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
///
///
/// Constraints:
/// * 1 <= s.length <= 1000
/// * s consists of lowercase English letters.
///
pub struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut cnt = 0;
        if s.len() == 0 {
            return 0;
        }

        let cs = s.chars().collect::<Vec<_>>();

        for i in 0..s.len() {
            // odd
            Self::check_palindrome(&mut cnt, &cs, i, i);
            // even
            Self::check_palindrome(&mut cnt, &cs, i, i + 1);
        }

        cnt
    }
    fn check_palindrome(cnt: &mut i32, s: &Vec<char>, i: usize, j: usize) {
        let mut i = i;
        let mut j = j;
        while j < s.len() && s[i] == s[j] {
            *cnt += 1;
            if i.checked_sub(1).is_none() {
                break;
            }
            i -= 1;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_substrings_1() {
        let s = "abc".to_string();
        let output = 3;
        assert_eq!(Solution::count_substrings(s), output);
    }

    #[test]
    fn test_count_substrings_2() {
        let s = "aaa".to_string();
        let output = 6;
        assert_eq!(Solution::count_substrings(s), output);
    }
}
