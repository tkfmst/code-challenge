/// 1249. Minimum Remove to Make Valid Parentheses
/// Medium
///
/// Given a string s of '(' , ')' and lowercase English characters.
///
/// Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
///
/// Formally, a parentheses string is valid if and only if:
///
/// * It is the empty string, contains only lowercase characters, or
/// * It can be written as AB (A concatenated with B), where A and B are valid strings, or
/// * It can be written as (A), where A is a valid string.
///
///
/// Example 1:
///
/// Input: s = "lee(t(c)o)de)"
/// Output: "lee(t(c)o)de"
/// Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
///
/// Example 2:
///
/// Input: s = "a)b(c)d"
/// Output: "ab(c)d"
///
/// Example 3:
///
/// Input: s = "))(("
/// Output: ""
/// Explanation: An empty string is also valid.
///
///
/// Constraints:
/// * 1 <= s.length <= 10^5
/// * s[i] is either'(' , ')', or lowercase English letter.

pub struct Solution {}
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut it = s.chars();
        let mut stack = vec![];
        let mut acc = vec![];

        while let Some(c) = it.next() {
            match c {
                '(' => stack.push(acc.len()),
                ')' => {
                    if let Some(open_idx) = stack.pop() {
                        acc.insert(open_idx, '(');
                        acc.push(')');
                    }
                }
                _ => acc.push(c),
            }
        }

        acc.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_remove_to_make_valid_1() {
        let s = "lee(t(c)o)de)".to_string();
        let output = "lee(t(c)o)de".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), output);
    }

    #[test]
    fn test_min_remove_to_make_valid_2() {
        let s = "a)b(c)d".to_string();
        let output = "ab(c)d".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), output);
    }

    #[test]
    fn test_min_remove_to_make_valid_3() {
        let s = "))((".to_string();
        let output = "".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), output);
    }

    #[test]
    fn test_min_remove_to_make_valid_4() {
        let s = "())()(((".to_string();
        let output = "()()".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), output);
    }

    #[test]
    fn test_min_remove_to_make_valid_5() {
        let s = ")i()()((fm(((()".to_string();
        let output = "i()()fm()".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), output);
    }
}
