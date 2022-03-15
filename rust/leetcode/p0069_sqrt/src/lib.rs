/// 69. Sqrt(x)
///
/// Given a non-negative integer x, compute and return the square root of x.
///
/// Since the return type is an integer, the decimal digits are truncated, and only the integer part of the result is returned.
///
/// Note: You are not allowed to use any built-in exponent function or operator, such as pow(x, 0.5) or x ** 0.5.
///
///  
///
/// Example 1:
///
/// Input: x = 4
/// Output: 2
///
/// Example 2:
///
/// Input: x = 8
/// Output: 2
///
/// Explanation: The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.

#[derive(Debug)]
pub struct Solution {}

impl Solution {
    // Newton's method
    pub fn my_sqrt(num: i32) -> i32 {
        // x(0)^2 = num
        // 0 = x(0)^2 - num
        //
        // // Therefore the intersection of (A) and (B) is the solution.
        // (A) f(x(n)) = x(n)^2 - x(0)
        // (B) y = 0
        //
        // // grad
        // (f(x(n+1)) - f(x(n))) / (x(n+1) - x(n)) = f'(x(n))
        // f'(x(n)) = 2x(n)
        //
        // // when f(x(n+1)) = 0
        // x(n+1) = x(n) - f(x(n)) / f'(x(n)))
        //
        // // when f(x(n)) = x(n)^2 + x(0)
        // x(n+1) = x(n) - (x(n)^2 + x(0)) / f'(x(n))
        //
        // // when f'(x(n)) = 2x(n)
        // x(n+1) = x(n) - (x(n)^2 + x(0)) / (2 * (x(n)))
        // x(n+1) = (x(n)^2 + x(0)) / (2 * (x(n)))
        let x_0: f64 = num as f64;
        let mut x_n: f64 = x_0;
        loop {
            let x_n_1: i32 = x_n as i32;
            x_n = (x_n * x_n + x_0) / (2_f64 * x_n);
            if x_n as i32 == x_n_1 {
                break;
            }
        }
        x_n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4_i32), 2_i32);
        assert_eq!(Solution::my_sqrt(8_i32), 2_i32);
        assert_eq!(Solution::my_sqrt(2147395599_i32), 46339_i32);
    }
}
