/// 507. Perfect Number
/// Easy
///
/// A perfect number is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.
///
/// Given an integer n, return true if n is a perfect number, otherwise return false.
///
///  
///
/// Example 1:
///
/// Input: num = 28
/// Output: true
/// Explanation: 28 = 1 + 2 + 4 + 7 + 14
/// 1, 2, 4, 7, and 14 are all divisors of 28.
///
/// Example 2:
///
/// Input: num = 7
/// Output: false
///
///  
///
/// Constraints:
/// * 1 <= num <= 10^8

pub struct Solution {}
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut s = 1;
        for x in 2..=(num as f32).sqrt() as i32 {
            if num % x == 0 {
                s += x + if x * x < num { num / x } else { 0 };
                if s > num {
                    return false;
                }
            }
        }
        s == num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_perfect_number_1() {
        let num = 28;
        let output = true;
        assert_eq!(Solution::check_perfect_number(num), output);
    }

    #[test]
    fn test_check_perfect_number_2() {
        let num = 7;
        let output = false;
        assert_eq!(Solution::check_perfect_number(num), output);
    }
}
