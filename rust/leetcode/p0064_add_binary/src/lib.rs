/// 67.Add Binary
///
/// Given two binary strings a and b, return their sum as a binary string.
///
/// Example 1:
///
/// Input: a = "11", b = "1"
/// Output: "100"
///
/// Example 2:
///
/// Input: a = "1010", b = "1011"
/// Output: "10101"
///
/// Constraints:
///
/// - 1 <= a.length, b.length <= 104
/// - a and b consist only of '0' or '1' characters.
/// - Each string does not contain leading zeros except for the zero itself.

#[derive(Debug)]
pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!("{:b}", bits_to_u128(a) + bits_to_u128(b))
    }
}

fn bits_to_u128(bits: String) -> u128 {
    let mut acc = 0;
    for (i, bit) in bits.chars().rev().enumerate() {
        let n = match bit.to_digit(2) {
            Some(b) => (b as u128) * 2_u128.pow(i as u32),
            None => 0,
        };
        acc += n;
    }

    acc
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            "100"
        );
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            "10101"
        );
        assert_eq!(
            Solution::add_binary(
                String::from("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101"),
                String::from("110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011"),
            ),
            "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000"
        );
    }
}
