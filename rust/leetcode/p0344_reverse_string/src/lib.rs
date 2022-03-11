/// 344. Reverse String
/// Easy
///
/// Write a function that reverses a string. The input string is given as an array of characters s.
///
/// You must do this by modifying the input array in-place with O(1) extra memory.
///
///
/// Example 1:
///
/// Input: s = ["h","e","l","l","o"]
/// Output: ["o","l","l","e","h"]
///
/// Example 2:
///
/// Input: s = ["H","a","n","n","a","h"]
/// Output: ["h","a","n","n","a","H"]
///
/// Constraints:
///
/// * 1 <= s.length <= 105
/// * s[i] is a printable ascii character.

pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // // answer 1
        // s.reverse()

        // answer 2
        let l = s.len();
        for i in 0..(l / 2) {
            s.swap(i, l - i - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_string_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, output);
    }

    #[test]
    fn test_reverse_string_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, output);
    }
}
