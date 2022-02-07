/// 202. Happy Number
/// Easy
///
/// Write an algorithm to determine if a number n is happy.
///
/// A happy number is a number defined by the following process:
///
/// * Starting with any positive integer, replace the number by the sum of the squares of its digits.
/// * Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
/// * Those numbers for which this process ends in 1 are happy.
///
/// Return true if n is a happy number, and false if not.
///
///
///
/// Example 1:
///
/// Input: n = 19
/// Output: true
/// Explanation:
/// 12 + 92 = 82
/// 82 + 22 = 68
/// 62 + 82 = 100
/// 12 + 02 + 02 = 1
///
/// Example 2:
///
/// Input: n = 2
/// Output: false
///
///
///
/// Constraints:
/// * 1 <= n <= 2^31 - 1
use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<u32> = HashSet::new();
        Self::check(n as u32, &mut seen)
    }

    fn check(n: u32, seen: &mut HashSet<u32>) -> bool {
        if n == 1 {
            true
        } else {
            let res = n
                .to_string()
                .chars()
                .fold(0, |acc, c| acc + c.to_digit(10).unwrap().pow(2));
            if seen.contains(&res) {
                false
            } else {
                seen.insert(res);
                Self::check(res, seen)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_happy_1() {
        let input: i32 = 19;
        let output = true;
        assert_eq!(Solution::is_happy(input), output);
    }

    #[test]
    fn test_is_happy_2() {
        let input: i32 = 2;
        let output = false;
        assert_eq!(Solution::is_happy(input), output);
    }

    #[test]
    fn test_is_happy_3() {
        let input: i32 = 7;
        let output = true;
        assert_eq!(Solution::is_happy(input), output);
    }
}
