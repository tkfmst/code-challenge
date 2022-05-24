/// 32. Longest Valid Parentheses
/// Hard
///
/// Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
///
///
/// Example 1:
///
/// Input: s = "(()"
/// Output: 2
/// Explanation: The longest valid parentheses substring is "()".
///
/// Example 2:
///
/// Input: s = ")()())"
/// Output: 4
/// Explanation: The longest valid parentheses substring is "()()".
///
/// Example 3:
///
/// Input: s = ""
/// Output: 0
///
///
/// Constraints:
/// 0 <= s.length <= 3 * 10^4
/// s[i] is '(', or ')'.

pub struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max = 0;
        let mut stack = vec![];

        stack.push(-1);
        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max = std::cmp::max(max, i as i32 - stack.last().unwrap());
                }
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_valid_parentheses_1() {
        let s = "(()".to_string();
        let output = 2;
        assert_eq!(Solution::longest_valid_parentheses(s), output);
    }

    #[test]
    fn test_longest_valid_parentheses_2() {
        let s = ")()())".to_string();
        let output = 4;
        assert_eq!(Solution::longest_valid_parentheses(s), output);
    }

    #[test]
    fn test_longest_valid_parentheses_3() {
        let s = "".to_string();
        let output = 0;
        assert_eq!(Solution::longest_valid_parentheses(s), output);
    }

    #[test]
    fn test_longest_valid_parentheses_4() {
        let s = "()(())".to_string();
        let output = 6;
        assert_eq!(Solution::longest_valid_parentheses(s), output);
    }

    #[test]
    fn test_longest_valid_parentheses_5() {
        let s = "()(()".to_string();
        let output = 2;
        assert_eq!(Solution::longest_valid_parentheses(s), output);
    }
}
