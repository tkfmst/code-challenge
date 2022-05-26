/// 509. Fibonacci Number
/// Easy
///
/// The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
///
/// F(0) = 0, F(1) = 1
/// F(n) = F(n - 1) + F(n - 2), for n > 1.
///
/// Given n, calculate F(n).
///
///
/// Example 1:
///
/// Input: n = 2
/// Output: 1
/// Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
///
/// Example 2:
///
/// Input: n = 3
/// Output: 2
/// Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
///
/// Example 3:
///
/// Input: n = 4
/// Output: 3
/// Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

pub struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::helper(n - 1, 1, 0),
        }
    }
    fn helper(n: i32, f1: i32, f2: i32) -> i32 {
        if n == 0 {
            f1
        } else {
            Self::helper(n - 1, f1 + f2, f1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_fib_1() {
        let n = 2;
        let output = 1;
        assert_eq!(Solution::fib(n), output);
    }

    #[test]
    fn test_fib_2() {
        let n = 3;
        let output = 2;
        assert_eq!(Solution::fib(n), output);
    }

    #[test]
    fn test_fib_3() {
        let n = 4;
        let output = 3;
        assert_eq!(Solution::fib(n), output);
    }
}
