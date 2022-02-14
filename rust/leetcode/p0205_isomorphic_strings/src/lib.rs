/// 205. Isomorphic Strings
/// Easy
///
/// Given two strings s and t, determine if they are isomorphic.
///
/// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
///
/// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
///
///  
///
/// Example 1:
///
/// Input: s = "egg", t = "add"
/// Output: true
///
/// Example 2:
///
/// Input: s = "foo", t = "bar"
/// Output: false
///
/// Example 3:
///
/// Input: s = "paper", t = "title"
/// Output: true
///
///  
///
/// Constraints:
///
/// * 1 <= s.length <= 5 * 10^4
/// * t.length == s.length
/// * s and t consist of any valid ascii character.
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, char> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();
        let mut cs1 = s.chars();
        let mut cs2 = t.chars();

        let mut res = false;

        loop {
            match (cs1.next(), cs2.next()) {
                (Some(c1), Some(c2)) => {
                    if let Some(&conv) = map.get(&c1) {
                        if conv != c2 {
                            break;
                        }
                    } else if let Some(_) = set.get(&c2) {
                        break;
                    } else {
                        map.insert(c1, c2);
                        set.insert(c2);
                    }
                }
                (None, None) => {
                    res = true;
                    break;
                }
                _ => {
                    // not reach
                    break;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_isomorphic_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let output = true;
        assert_eq!(Solution::is_isomorphic(s, t), output);
    }

    #[test]
    fn test_is_isomorphic_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let output = false;
        assert_eq!(Solution::is_isomorphic(s, t), output);
    }

    #[test]
    fn test_is_isomorphic_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        let output = true;
        assert_eq!(Solution::is_isomorphic(s, t), output);
    }

    #[test]
    fn test_is_isomorphic_4() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        let output = false;
        assert_eq!(Solution::is_isomorphic(s, t), output);
    }
}
