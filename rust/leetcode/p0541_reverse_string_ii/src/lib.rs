/// 541. Reverse String II
/// Easy
///
/// Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.
///
/// If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.
///
///
/// Example 1:
///
/// Input: s = "abcdefg", k = 2
/// Output: "bacdfeg"
///
/// Example 2:
///
/// Input: s = "abcd", k = 2
/// Output: "bacd"
///
///
/// Constraints:
/// * 1 <= s.length <= 10^4
/// * s consists of only lowercase English letters.
/// * 1 <= k <= 10^4

pub struct Solution {}
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let ku = k as usize;
        let mut sb = s.into_bytes();

        for ch in sb.chunks_mut(ku * 2) {
            let mut i = 0;
            let mut j = ku.min(ch.len()) - 1;
            while i < j {
                ch.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        String::from_utf8(sb).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_str_1() {
        let s = "abcdefg".to_string();
        let k = 2;
        let output = "bacdfeg".to_string();
        assert_eq!(Solution::reverse_str(s, k), output);
    }

    #[test]
    fn test_reverse_str_2() {
        let s = "abcd".to_string();
        let k = 2;
        let output = "bacd".to_string();
        assert_eq!(Solution::reverse_str(s, k), output);
    }
}
