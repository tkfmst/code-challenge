/// 318. Maximum Product of Word Lengths
/// Medium
///
/// Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.
///
///
/// Example 1:
///
/// Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
/// Output: 16
/// Explanation: The two words can be "abcw", "xtfn".
///
/// Example 2:
///
/// Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
/// Output: 4
/// Explanation: The two words can be "ab", "cd".
///
/// Example 3:
///
/// Input: words = ["a","aa","aaa","aaaa"]
/// Output: 0
/// Explanation: No such pair of words.
///
///
/// Constraints:
/// * 2 <= words.length <= 1000
/// * 1 <= words[i].length <= 1000
/// * words[i] consists only of lowercase English letters.

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut res = 0;
        let mut hm: HashMap<u32, usize> = HashMap::with_capacity(words.len());

        for w in words {
            let mut mask: u32 = 0;
            for c in w.bytes() {
                mask |= 1 << (c - b'a');
            }
            let size = hm.entry(mask).or_default();
            *size = w.len().max(*size);
        }

        for (&k1, &v1) in &hm {
            for (&k2, &v2) in &hm {
                if k1 & k2 == 0 {
                    res = res.max(v1 * v2);
                }
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_product_1() {
        let words = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];
        let output = 16;
        assert_eq!(Solution::max_product(words), output);
    }

    #[test]
    fn test_max_product_2() {
        let words = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ];
        let output = 4;
        assert_eq!(Solution::max_product(words), output);
    }

    #[test]
    fn test_max_product_3() {
        let words = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];
        let output = 0;
        assert_eq!(Solution::max_product(words), output);
    }
}
