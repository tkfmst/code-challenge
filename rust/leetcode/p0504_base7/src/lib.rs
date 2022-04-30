/// 504. Base 7
/// Easy
///
/// Given an integer num, return a string of its base 7 representation.
///
///
/// Example 1:
///
/// Input: num = 100
/// Output: "202"
///
/// Example 2:
///
/// Input: num = -7
/// Output: "-10"
///
///
/// Constraints:
/// * -10^7 <= num <= 10^7

pub struct Solution {}
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut result = vec![];
        let mut quot = num.abs();
        let mut rem;

        while quot > 0 {
            rem = quot % 7;
            quot = quot / 7;
            result.push(rem as u8 + 48) // 48u8 = '0'
        }

        if num < 0 {
            result.push(45);
        }

        result.reverse();
        println!("{:?}", result);

        result.iter().map(|&u| u as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_convert_to_base7_1() {
        let num = 100;
        let output = "202".to_string();
        assert_eq!(Solution::convert_to_base7(num), output);
    }

    #[test]
    fn test_convert_to_base7_2() {
        let num = -7;
        let output = "-10".to_string();
        assert_eq!(Solution::convert_to_base7(num), output);
    }
}
