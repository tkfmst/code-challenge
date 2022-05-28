// 520. Detect Capital
// Easy
//
// We define the usage of capitals in a word to be right when one of the following cases holds:
//
// All letters in this word are capitals, like "USA".
// All letters in this word are not capitals, like "leetcode".
// Only the first letter in this word is capital, like "Google".
//
// Given a string word, return true if the usage of capitals in it is right.
//
//
// Example 1:
//
// Input: word = "USA"
// Output: true
//
// Example 2:
//
// Input: word = "FlaG"
// Output: false
//
//
// Constraints:
//
// 1 <= word.length <= 100
// word consists of lowercase and uppercase English letters.
pub struct Solution {}
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() <= 1 {
            return true;
        }

        let cs = word.as_bytes();
        match (cs[0].is_ascii_uppercase(), cs[1].is_ascii_uppercase()) {
            (true, true) => cs[2..].iter().all(|c| c.is_ascii_uppercase()), // all letters are capital
            (true, false) | (false, false) => cs[2..].iter().all(|c| c.is_ascii_lowercase()), // all letters are camel-case or small
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_detect_capital_use_1() {
        let word = "USA".to_string();
        let output = true;
        assert_eq!(Solution::detect_capital_use(word), output);
    }

    #[test]
    fn test_detect_capital_use_2() {
        let word = "FlaG".to_string();
        let output = false;
        assert_eq!(Solution::detect_capital_use(word), output);
    }

    #[test]
    fn test_detect_capital_use_3() {
        let word = "ggg".to_string();
        let output = true;
        assert_eq!(Solution::detect_capital_use(word), output);
    }
}
