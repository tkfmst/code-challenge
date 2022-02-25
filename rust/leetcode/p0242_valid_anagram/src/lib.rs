/// 242. Valid Anagram
/// Easy
///
/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
///
/// Example 1:
///
/// Input: s = "anagram", t = "nagaram"
/// Output: true
///
/// Example 2:
///
/// Input: s = "rat", t = "car"
/// Output: false
///
/// Constraints:
/// * 1 <= s.length, t.length <= 5 * 10^4
/// * s and t consist of lowercase English letters.
///
/// Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s2: Vec<u8> = s.into_bytes();
        let mut t2: Vec<u8> = t.into_bytes();
        s2.sort_unstable();
        t2.sort_unstable();

        s2 == t2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_anagram_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        let output = true;

        assert_eq!(Solution::is_anagram(s, t), output);
    }

    #[test]
    fn test_is_anagram_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        let output = false;

        assert_eq!(Solution::is_anagram(s, t), output);
    }
}
