/// 29. Divide Two Integers
/// Medium
///
/// Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
///
/// The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
///
/// Return the quotient after dividing dividend by divisor.
///
/// Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−231, 231 − 1]. For this problem, if the quotient is strictly greater than 231 - 1, then return 231 - 1, and if the quotient is strictly less than -231, then return -231.
///
///
/// Example 1:
///
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = 3.33333.. which is truncated to 3.
///
/// Example 2:
///
/// Input: dividend = 7, divisor = -3
/// Output: -2
/// Explanation: 7/-3 = -2.33333.. which is truncated to -2.
///
///
/// Constraints:
/// * -231 <= dividend, divisor <= 231 - 1
/// * divisor != 0

pub struct Solution {}
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut res = 0;

        let is_neg = (dividend < 0) ^ (divisor < 0);

        let mut p = if dividend > 0 { -dividend } else { dividend };
        let q = if divisor > 0 { -divisor } else { divisor };

        for shift in (0..q.leading_ones()).rev() {
            if p <= (q << shift) {
                p -= q << shift;
                res += -1 << shift;
            }
        }

        if is_neg {
            res
        } else if res == i32::MIN {
            i32::MAX
        } else {
            -res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_divice_1() {
        let dividend = 10;
        let divisor = 3;
        let output = 3;
        assert_eq!(Solution::divide(dividend, divisor), output);
    }

    #[test]
    fn test_divice_2() {
        let dividend = 7;
        let divisor = -3;
        let output = -2;
        assert_eq!(Solution::divide(dividend, divisor), output);
    }
}
