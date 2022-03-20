/// 387. First Unique Character in a String
/// Easy
///
/// Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
///
///
/// Example 1:
///
/// Input: s = "leetcode"
/// Output: 0
///
/// Example 2:
///
/// Input: s = "loveleetcode"
/// Output: 2
///
/// Example 3:
///
/// Input: s = "aabb"
/// Output: -1
///
///  
/// Constraints:
/// * 1 <= s.length <= 10^5
/// * s consists of only lowercase English letters.

pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts: [u16; 26] = [0; 26];
        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if counts[(c as u8 - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_first_uniq_char_1() {
        let s = "leetcode".to_string();
        let output = 0;
        assert_eq!(Solution::first_uniq_char(s), output);
    }

    #[test]
    fn test_first_uniq_char_2() {
        let s = "loveleetcode".to_string();
        let output = 2;
        assert_eq!(Solution::first_uniq_char(s), output);
    }

    #[test]
    fn test_first_uniq_char_3() {
        let s = "aabb".to_string();
        let output = -1;
        assert_eq!(Solution::first_uniq_char(s), output);
    }
}
