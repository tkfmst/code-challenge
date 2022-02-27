/// 258. Add Digits
/// Easy
///
/// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.
///
/// Example 1:
///
/// Input: num = 38
/// Output: 2
/// Explanation: The process is
/// 38 --> 3 + 8 --> 11
/// 11 --> 1 + 1 --> 2
/// Since 2 has only one digit, return it.
///
/// Example 2:
///
/// Input: num = 0
/// Output: 0
///
///  
///
/// Constraints:
/// * 0 <= num <= 2^31 - 1

pub struct Solution {}
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        match num {
            0 => 0,
            _ if num % 9 == 0 => 9,
            _ => num % 9,
        }
        // let n = num
        //     .to_string()
        //     .chars()
        //     .fold(0, |acc, x| acc + x.to_digit(10).unwrap()) as i32;
        //
        // if n < 10 {
        //     n
        // } else {
        //     Self::add_digits(n)
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_add_digits_1() {
        let num = 38;
        let output = 2;
        assert_eq!(Solution::add_digits(num), output);
    }

    #[test]
    fn test_add_digits_2() {
        let num = 0;
        let output = 0;
        assert_eq!(Solution::add_digits(num), output);
    }
}
