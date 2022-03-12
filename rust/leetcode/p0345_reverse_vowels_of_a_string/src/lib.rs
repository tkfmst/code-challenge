/// 345. Reverse Vowels of a String
/// Easy
///
/// Given a string s, reverse only all the vowels in the string and return it.
///
/// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both cases.
///
///
/// Example 1:
///
/// Input: s = "hello"
/// Output: "holle"
///
/// Example 2:
///
/// Input: s = "leetcode"
/// Output: "leotcede"
///
/// Constraints:
/// * 1 <= s.length <= 3 * 105
/// * s consist of printable ASCII characters.

pub struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        let mut v: Vec<char> = s.chars().collect();
        let mut cur1 = 0;
        let mut cur2 = s.len() - 1;

        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        'outer: while cur1 < cur2 {
            if let Some(c1) = v.get(cur1) {
                if vowels.contains(&c1) {
                    while cur1 < cur2 {
                        if let Some(c2) = v.get(cur2) {
                            if vowels.contains(&c2) {
                                v.swap(cur1, cur2);
                                cur2 -= 1;
                                break;
                            }
                        } else {
                            break 'outer;
                        }
                        cur2 -= 1;
                    }
                }
            }
            cur1 += 1;
        }

        v.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_vowels_1() {
        let s = "hello".to_string();
        let output = "holle".to_string();
        assert_eq!(Solution::reverse_vowels(s), output);
    }

    #[test]
    fn test_reverse_vowels_2() {
        let s = "leetcode".to_string();
        let output = "leotcede".to_string();
        assert_eq!(Solution::reverse_vowels(s), output);
    }

    #[test]
    fn test_reverse_vowels_3() {
        let s = "aA".to_string();
        let output = "Aa".to_string();
        assert_eq!(Solution::reverse_vowels(s), output);
    }

    #[test]
    fn test_reverse_vowels_4() {
        let s = "race car".to_string();
        let output = "race car".to_string();
        assert_eq!(Solution::reverse_vowels(s), output);
    }
}
