/// 263. Ugly Number
/// Easy
///
/// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
///
/// Given an integer n, return true if n is an ugly number.
///  
///
/// Example 1:
///
/// Input: n = 6
/// Output: true
/// Explanation: 6 = 2 × 3
///
/// Example 2:
///
/// Input: n = 1
/// Output: true
/// Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
///
/// Example 3:
///
/// Input: n = 14
/// Output: false
/// Explanation: 14 is not ugly since it includes the prime factor 7.
///
///
/// Constraints:
/// * -2^31 <= n <= 2^31 - 1

pub struct Solution {}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        match n {
            0 => false,
            1 => true,
            _ if n % 2 == 0 => Self::is_ugly(n / 2),
            _ if n % 3 == 0 => Self::is_ugly(n / 3),
            _ if n % 5 == 0 => Self::is_ugly(n / 5),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_ugly_1() {
        let n = 6;
        let output = true;
        assert_eq!(Solution::is_ugly(n), output);
    }

    #[test]
    fn test_is_ugly_2() {
        let n = 1;
        let output = true;
        assert_eq!(Solution::is_ugly(n), output);
    }

    #[test]
    fn test_is_ugly_3() {
        let n = 14;
        let output = false;
        assert_eq!(Solution::is_ugly(n), output);
    }

    #[test]
    fn test_is_ugly_4() {
        let n = 0;
        let output = false;
        assert_eq!(Solution::is_ugly(n), output);
    }
}
