/// 190. Reverse Bits
/// Easy
///
/// Reverse bits of a given 32 bits unsigned integer.
///
/// Note:
/// * Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
/// * In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.
///
///
///
/// Example 1:
///
/// Input: n = 00000010100101000001111010011100
/// Output:    964176192 (00111001011110000010100101000000)
/// Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.
///
/// Example 2:
///
/// Input: n = 11111111111111111111111111111101
/// Output:   3221225471 (10111111111111111111111111111111)
/// Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.
///
///
/// Constraints:
///
/// * The input must be a binary string of length 32

pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        (0..u32::BITS)
            .map(|i| (x >> i) & 1)
            .fold(0, |acc, n| (acc << 1) | n)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_bits_1() {
        let input = 0b00000010100101000001111010011100;
        let output = 964176192; // 0b00111001011110000010100101000000
        println!("{:b}", Solution::reverse_bits(input));
        assert_eq!(Solution::reverse_bits(input), output);
    }

    #[test]
    fn test_reverse_bits_2() {
        let input = 0b11111111111111111111111111111101;
        let output = 3221225471; // 0b10111111111111111111111111111111
        assert_eq!(Solution::reverse_bits(input), output);
    }
}
