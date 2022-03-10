// 342. Power of Four
// Easy
//
// Given an integer n, return true if it is a power of four. Otherwise, return false.
//
// An integer n is a power of four, if there exists an integer x such that n == 4x.
//
// Example 1:
//
// Input: n = 16
// Output: true
//
// Example 2:
//
// Input: n = 5
// Output: false
//
// Example 3:
//
// Input: n = 1
// Output: true
//
// Constraints:
//
// * -2^31 <= n <= 2^31 - 1

pub struct Solution {}
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        match n {
            0 => false,
            1 => true,
            _ => {
                let quo = n / 4;
                let rem = n % 4;
                if quo == 0 {
                    false
                } else if rem != 0 {
                    false
                } else {
                    Self::is_power_of_four(quo)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_power_of_four_1() {
        let n = 16;
        let output = true;
        assert_eq!(Solution::is_power_of_four(n), output);
    }

    #[test]
    fn test_is_power_of_four_2() {
        let n = 5;
        let output = false;
        assert_eq!(Solution::is_power_of_four(n), output);
    }

    #[test]
    fn test_is_power_of_four_3() {
        let n = 1;
        let output = true;
        assert_eq!(Solution::is_power_of_four(n), output);
    }
}
