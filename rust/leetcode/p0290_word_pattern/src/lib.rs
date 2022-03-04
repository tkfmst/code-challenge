/// 290. Word Pattern
/// Easy
///
/// Given a pattern and a string s, find if s follows the same pattern.
///
/// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
///
///  
///
/// Example 1:
///
/// Input: pattern = "abba", s = "dog cat cat dog"
/// Output: true
///
/// Example 2:
///
/// Input: pattern = "abba", s = "dog cat cat fish"
/// Output: false
///
/// Example 3:
///
/// Input: pattern = "aaaa", s = "dog cat cat dog"
/// Output: false
///
///  
///
/// Constraints:
///
/// * 1 <= pattern.length <= 300
/// * pattern contains only lower-case English letters.
/// * 1 <= s.length <= 3000
/// * s contains only lowercase English letters and spaces ' '.
/// * s does not contain any leading or trailing spaces.
/// * All the words in s are separated by a single space.
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    const NONE: usize = 300;
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut hash: HashMap<String, usize> = HashMap::new();
        let ss: Vec<&str> = s.split_whitespace().collect();

        if pattern.len() != ss.len() {
            return false;
        }

        for (i, c) in pattern.chars().enumerate() {
            let mut cs = c.to_string();
            cs.push('_'); // distinguish between K and V
            let x = hash.insert(cs, i).unwrap_or(Self::NONE);
            let y = hash.insert(ss[i].to_string(), i).unwrap_or(Self::NONE);
            if x != y && i != y {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_word_pattern_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let output = true;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }

    #[test]
    fn test_word_pattern_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let output = false;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }

    #[test]
    fn test_word_pattern_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        let output = false;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }

    #[test]
    fn test_word_pattern_4() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        let output = false;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }

    #[test]
    fn test_word_pattern_5() {
        let pattern = "a".to_string();
        let s = "a".to_string();
        let output = true;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }

    #[test]
    fn test_word_pattern_6() {
        let pattern = "abc".to_string();
        let s = "b c a".to_string();
        let output = true;
        assert_eq!(Solution::word_pattern(pattern, s), output);
    }
}
