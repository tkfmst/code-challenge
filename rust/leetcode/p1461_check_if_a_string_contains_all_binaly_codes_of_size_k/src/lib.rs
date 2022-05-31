/// 1461. Check If a String Contains All Binary Codes of Size K
/// Medium
///
/// Given a binary string s and an integer k, return true if every binary code of length k is a substring of s. Otherwise, return false.
///
///
/// Example 1:
///
/// Input: s = "00110110", k = 2
/// Output: true
/// Explanation: The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indices 0, 1, 3 and 2 respectively.
///
/// Example 2:
///
/// Input: s = "0110", k = 1
/// Output: true
/// Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring.
///
/// Example 3:
///
/// Input: s = "0110", k = 2
/// Output: false
/// Explanation: The binary code "00" is of length 2 and does not exist in the array.
///
///
/// Constraints:
/// * 1 <= s.length <= 5 * 10^5
/// * s[i] is either '0' or '1'.
/// * 1 <= k <= 20

pub struct Solution {}
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut it = s.as_bytes().iter();
        let c2b = |b: u8| if b == b'0' { 0 } else { 1 };

        let mut hash = (&mut it)
            .take((k - 1) as usize)
            .fold(0, |acc, b| (acc << 1) | c2b(*b));

        let n = 1 << k;

        let mask = n - 1;
        let mut set = vec![false; n];

        let mut need = n;

        it.any(|b| {
            hash = ((hash << 1) & mask) | c2b(*b);
            if !set[hash] {
                set[hash] = true;
                need -= 1;
            }

            need == 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_has_all_codes_1() {
        let s = "00110110".to_string();
        let k = 2;
        let output = true;
        assert_eq!(Solution::has_all_codes(s, k), output);
    }

    #[test]
    fn test_has_all_codes_2() {
        let s = "0110".to_string();
        let k = 1;
        let output = true;
        assert_eq!(Solution::has_all_codes(s, k), output);
    }

    #[test]
    fn test_has_all_codes_3() {
        let s = "0110".to_string();
        let k = 2;
        let output = false;
        assert_eq!(Solution::has_all_codes(s, k), output);
    }
}
