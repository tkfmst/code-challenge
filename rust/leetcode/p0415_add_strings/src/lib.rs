// 415. Add Strings
// Easy
//
// Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
//
// You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
//
//
// Example 1:
//
// Input: num1 = "11", num2 = "123"
// Output: "134"
//
// Example 2:
//
// Input: num1 = "456", num2 = "77"
// Output: "533"
//
// Example 3:
//
// Input: num1 = "0", num2 = "0"
// Output: "0"
//
//
// Constraints:
//
// * 1 <= num1.length, num2.length <= 10^4
// * num1 and num2 consist of only digits.
// * num1 and num2 don't have any leading zeros except for the zero itself.

pub struct Solution {}
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut cs1 = num1.chars().rev().peekable();
        let mut cs2 = num2.chars().rev().peekable();

        let mut result = vec![];
        let mut up = 0;

        while cs1.peek().is_some() || cs2.peek().is_some() {
            let tmp = match (cs1.next(), cs2.next()) {
                (Some(c1), Some(c2)) => c1
                    .to_digit(10)
                    .and_then(|n1| c2.to_digit(10).map(|n2| n1 + n2 + up)),
                (Some(c1), None) => c1.to_digit(10).map(|n| n + up),
                (None, Some(c2)) => c2.to_digit(10).map(|n| n + up),
                (None, None) => None,
            };

            if let Some(n) = tmp {
                up = n / 10;
                result.push((n % 10) as u8 + b'0');
            }
        }

        if up != 0 {
            result.push(up as u8 + b'0');
        }

        result.reverse();

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_add_strings_1() {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        let output = "134".to_string();
        assert_eq!(Solution::add_strings(num1, num2), output);
    }

    #[test]
    fn test_add_strings_2() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        let output = "533".to_string();
        assert_eq!(Solution::add_strings(num1, num2), output);
    }

    #[test]
    fn test_add_strings_3() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        let output = "0".to_string();
        assert_eq!(Solution::add_strings(num1, num2), output);
    }
}
