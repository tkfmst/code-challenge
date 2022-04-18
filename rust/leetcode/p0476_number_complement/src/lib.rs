/// 476. Number Complement
/// Easy
///
/// The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.
///
/// For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
///
/// Given an integer num, return its complement.
///
///
/// Example 1:
///
/// Input: num = 5
/// Output: 2
/// Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
///
/// Example 2:
///
/// Input: num = 1
/// Output: 0
/// Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
///
/// Constraints:
///
/// 1 <= num < 2^31

pub struct Solution {}
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        // answer 1
        // let mut i = 0;
        // while 2i64.pow(i) <= num as i64 {
        //     i += 1;
        // }
        // (2i64.pow(i) - 1 - (num as i64)) as i32

        // answer 2
        let digits = 32 - num.leading_zeros();
        ((1 << digits) - 1) ^ num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn tes_find_complement_1() {
        let num = 5;
        let output = 2;
        assert_eq!(Solution::find_complement(num), output);
    }

    #[test]
    fn tes_find_complement_2() {
        let num = 1;
        let output = 0;
        assert_eq!(Solution::find_complement(num), output);
    }

    #[test]
    fn tes_find_complement_3() {
        let num = 2;
        let output = 1;
        assert_eq!(Solution::find_complement(num), output);
    }
}
