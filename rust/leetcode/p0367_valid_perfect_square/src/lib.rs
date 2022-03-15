/// 367. Valid Perfect Square
/// Easy
///
/// Given a positive integer num, write a function which returns True if num is a perfect square else False.
///
/// Follow up: Do not use any built-in library function such as sqrt.
///
///
/// Example 1:
///
/// Input: num = 16
/// Output: true
///
///
/// Example 2:
///
/// Input: num = 14
/// Output: false
///
///
/// Constraints:
/// * 1 <= num <= 2^31 - 1

pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // Newton's method
        fn my_sqrt(n: i32) -> i32 {
            let x0: f64 = n as f64;
            let mut xn: f64 = x0;
            loop {
                let xn_1: i32 = xn as i32;
                xn = (xn * xn + x0) / (2_f64 * xn);
                if xn as i32 == xn_1 {
                    break;
                }
            }
            xn as i32
        }

        let square = my_sqrt(num);
        if square.pow(2) == num {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_perfect_square_1() {
        let num = 16;
        let output = true;
        assert_eq!(Solution::is_perfect_square(num), output);
    }

    #[test]
    fn test_is_perfect_square_2() {
        let num = 14;
        let output = false;
        assert_eq!(Solution::is_perfect_square(num), output);
    }
}
