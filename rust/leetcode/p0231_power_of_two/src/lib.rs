/// 231. Power of Two
/// Easy
///
/// Given an integer n, return true if it is a power of two. Otherwise, return false.
///
/// An integer n is a power of two, if there exists an integer x such that n == 2x.
///
///  
///
/// Example 1:
///
/// Input: n = 1
/// Output: true
/// Explanation: 20 = 1
///
/// Example 2:
///
/// Input: n = 16
/// Output: true
/// Explanation: 24 = 16
///
/// Example 3:
///
/// Input: n = 3
/// Output: false
///
///  
///
/// Constraints:
///
/// * -2^31 <= n <= 2^31 - 1

pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // n > 0 && n.count_ones() == 1
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_power_of_two_1() {
        let n = 1;
        let output = true;
        assert_eq!(Solution::is_power_of_two(n), output);
    }

    #[test]
    fn test_is_power_of_two_2() {
        let n = 16;
        let output = true;
        assert_eq!(Solution::is_power_of_two(n), output);
    }

    #[test]
    fn test_is_power_of_two_3() {
        let n = 3;
        let output = false;
        assert_eq!(Solution::is_power_of_two(n), output);
    }

    #[test]
    fn test_is_power_of_two_4() {
        let n = 0;
        let output = false;
        assert_eq!(Solution::is_power_of_two(n), output);
    }

    #[test]
    fn test_is_power_of_two_5() {
        let n = i32::MIN;
        let output = false;
        assert_eq!(Solution::is_power_of_two(n), output);
    }

    #[test]
    fn test_is_power_of_two_6() {
        let n = i32::MIN + 1;
        let output = false;
        assert_eq!(Solution::is_power_of_two(n), output);
    }
}
