/// 557. Reverse Words in a String III
/// Easy
///
/// Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
///
///
/// Example 1:
///
/// Input: s = "Let's take LeetCode contest"
/// Output: "s'teL ekat edoCteeL tsetnoc"
///
/// Example 2:
///
/// Input: s = "God Ding"
/// Output: "doG gniD"
///
///
/// Constraints:
/// * 1 <= s.length <= 5 * 10^4
/// * s contains printable ASCII characters.
/// * s does not contain any leading or trailing spaces.
/// * There is at least one word in s.
/// * All the words in s are separated by a single space.

pub struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_words_1() {
        let s = "Let's take LeetCode contest".to_string();
        let output = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(Solution::reverse_words(s), output);
    }

    #[test]
    fn test_reverse_words_2() {
        let s = "God Ding".to_string();
        let output = "doG gniD".to_string();
        assert_eq!(Solution::reverse_words(s), output);
    }
}
