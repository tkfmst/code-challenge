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
    pub fn my_sqrt(x: i32) -> i32 {
        let a: f64 = x as f64;
        let mut b: f64 = a;
        loop {
            let bi: i32 = b as i32;
            b = (b * b + a) / (2_f64 * b);
            if b as i32 == bi {
                break;
            }
        }
        b as i32
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
