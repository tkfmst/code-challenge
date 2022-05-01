/// 844. Backspace String Compare
/// Easy
///
/// Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
///
/// Note that after backspacing an empty text, the text will continue empty.
///
///
///
/// Example 1:
///
/// Input: s = "ab#c", t = "ad#c"
/// Output: true
/// Explanation: Both s and t become "ac".
///
/// Example 2:
///
/// Input: s = "ab##", t = "c#d#"
/// Output: true
/// Explanation: Both s and t become "".
///
/// Example 3:
///
/// Input: s = "a#c", t = "b"
/// Output: false
/// Explanation: s becomes "c" while t becomes "b".
///
///
///
/// Constraints:
/// * 1 <= s.length, t.length <= 200
/// * s and t only contain lowercase letters and '#' characters.

pub struct Solution {}
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let ss = s.chars().collect::<Vec<_>>();
        let tt = t.chars().collect::<Vec<_>>();

        let mut i = (s.len() - 1) as i32;
        let mut j = (t.len() - 1) as i32;

        while i >= 0 || j >= 0 {
            let mut skip = 0;
            while i >= 0 {
                if ss[i as usize] == '#' {
                    skip += 1;
                } else if skip > 0 {
                    skip -= 1;
                } else {
                    break;
                }
                i -= 1;
            }

            skip = 0;
            while j >= 0 {
                if tt[j as usize] == '#' {
                    skip += 1;
                } else if skip > 0 {
                    skip -= 1;
                } else {
                    break;
                }
                j -= 1;
            }

            if i < 0 && j < 0 {
                // e.g. s="#", t="#"
                return true;
            } else if i < 0 || j < 0 {
                return false;
            }

            if ss[i as usize] != tt[j as usize] {
                return false;
            }

            i -= 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_backspace_compare_1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        let output = true;
        assert_eq!(Solution::backspace_compare(s, t), output);
    }

    #[test]
    fn test_backspace_compare_2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        let output = true;
        assert_eq!(Solution::backspace_compare(s, t), output);
    }

    #[test]
    fn test_backspace_compare_3() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        let output = false;
        assert_eq!(Solution::backspace_compare(s, t), output);
    }

    #[test]
    fn test_backspace_compare_4() {
        let s = "xywrrmp".to_string();
        let t = "xywrrmu#p".to_string();
        let output = true;
        assert_eq!(Solution::backspace_compare(s, t), output);
    }
}
