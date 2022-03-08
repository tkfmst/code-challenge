/// 326. Power of Three
/// Easy
///
/// Given an integer n, return true if it is a power of three. Otherwise, return false.
///
/// An integer n is a power of three, if there exists an integer x such that n == 3x.
///
///
/// Example 1:
///
/// Input: n = 27
/// Output: true
///
/// Example 2:
///
/// Input: n = 0
/// Output: false
///
/// Example 3:
///
/// Input: n = 9
/// Output: true
///
/// Constraints:
/// * -2^31 <= n <= 2^31 - 1

pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        println!("{:?}", n);
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }

        let quo = n / 3;
        let rem = n % 3;

        if rem != 0 {
            false
        } else if quo == 1 {
            true
        } else {
            Self::is_power_of_three(quo)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_power_of_three_1() {
        let n = 27;
        let output = true;
        assert_eq!(Solution::is_power_of_three(n), output);
    }

    #[test]
    fn test_is_power_of_three_2() {
        let n = 0;
        let output = false;
        assert_eq!(Solution::is_power_of_three(n), output);
    }

    #[test]
    fn test_is_power_of_three_3() {
        let n = 9;
        let output = true;
        assert_eq!(Solution::is_power_of_three(n), output);
    }

    #[test]
    fn test_is_power_of_three_4() {
        let n = 45;
        let output = false;
        assert_eq!(Solution::is_power_of_three(n), output);
    }

    #[test]
    fn test_is_power_of_three_5() {
        let n = 1;
        let output = true;
        assert_eq!(Solution::is_power_of_three(n), output);
    }
}
