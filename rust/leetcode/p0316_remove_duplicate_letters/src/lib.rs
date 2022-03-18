/// 316. Remove Duplicate Letters
/// Medium
///
/// Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.
///
/// Example 1:
///
/// Input: s = "bcabc"
/// Output: "abc"
///
/// Example 2:
///
/// Input: s = "cbacdcbc"
/// Output: "acdb"
///
/// Constraints:
/// * 1 <= s.length <= 10^4
/// * s consists of lowercase English letters.
///
///
/// Note: This question is the same as 1081: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut result: Vec<char> = Vec::with_capacity(26);
        let mut counts: [usize; 26] = [0; 26];
        let mut exists: [bool; 26] = [false; 26];

        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            counts[i] -= 1;

            if exists[i] {
                continue;
            }

            while let Some(&last) = result.last() {
                let j = (last as u8 - b'a') as usize;
                if b < last as u8 && counts[j] > 0 {
                    exists[j] = false;
                    result.pop();
                } else {
                    break;
                }
            }

            result.push(b as char);
            exists[i] = true;
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_duplicate_letters_1() {
        let s = "bcabc".to_string();
        let output = "abc".to_string();
        assert_eq!(Solution::remove_duplicate_letters(s), output);
    }

    #[test]
    fn test_remove_duplicate_letters_2() {
        let s = "cbacdcbc".to_string();
        let output = "acdb".to_string();
        assert_eq!(Solution::remove_duplicate_letters(s), output);
    }
}
