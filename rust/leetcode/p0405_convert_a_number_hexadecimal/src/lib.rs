/// 405. Convert a Number to Hexadecimal
/// Easy
///
/// Given an integer num, return a string representing its hexadecimal representation. For negative integers, two’s complement method is used.
///
/// All the letters in the answer string should be lowercase characters, and there should not be any leading zeros in the answer except for the zero itself.
///
/// Note: You are not allowed to use any built-in library method to directly solve this problem.
///
///  
///
/// Example 1:
///
/// Input: num = 26
/// Output: "1a"
///
/// Example 2:
///
/// Input: num = -1
/// Output: "ffffffff"
///
///  
///
/// Constraints:
/// * -2^31 <= num <= 2^31 - 1

pub struct Solution {}
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut result = vec![];
        let hex = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        // shift without sign, the bits are same
        // >> format!("{:b}", -1i32)
        //    "11111111111111111111111111111111"
        // >> format!("{:b}", -1i32 as u32)
        //    "11111111111111111111111111111111"
        let mut n = num as u32;

        while n > 0 {
            let b = n & 0xf;
            result.push(b as usize);
            n >>= 4;
        }

        result.reverse();

        result.iter().map(|&i| hex[i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_to_hex_1() {
        let num = 26;
        let output = "1a";
        assert_eq!(Solution::to_hex(num), output);
    }

    #[test]
    fn test_to_hex_2() {
        let num = -1;
        let output = "ffffffff";
        assert_eq!(Solution::to_hex(num), output);
    }
}
