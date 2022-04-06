/// 434. Number of Segments in a String
/// Easy
///
/// Given a string s, return the number of segments in the string.
///
/// A segment is defined to be a contiguous sequence of non-space characters.
///
///
/// Example 1:
///
/// Input: s = "Hello, my name is John"
/// Output: 5
/// Explanation: The five segments are ["Hello,", "my", "name", "is", "John"]
///
/// Example 2:
///
/// Input: s = "Hello"
/// Output: 1
///
///
/// Constraints:
///
/// * 0 <= s.length <= 300
/// * s consists of lowercase and uppercase English letters, digits, or one of the following characters "!@#$%^&*()_+-=',.:".
/// * The only space character in s is ' '.

pub struct Solution {}
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        // // answer1 use api
        // s.split_whitespace().count() as i32

        // answer2
        let mut count = 0;
        let mut it = s.chars();
        while let Some(c1) = it.next() {
            if c1 != ' ' {
                count += 1;

                while let Some(c2) = it.next() {
                    if c2 == ' ' {
                        break;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_segments_1() {
        let s = "Hello, my name is John".to_string();
        let output = 5;
        assert_eq!(Solution::count_segments(s), output);
    }

    #[test]
    fn test_count_segments_2() {
        let s = "Hello".to_string();
        let output = 1;
        assert_eq!(Solution::count_segments(s), output);
    }

    #[test]
    fn test_count_segments_3() {
        let s = "".to_string();
        let output = 0;
        assert_eq!(Solution::count_segments(s), output);
    }

    #[test]
    fn test_count_segments_4() {
        let s = "                ".to_string();
        let output = 0;
        assert_eq!(Solution::count_segments(s), output);
    }
}
