/// 500. Keyboard Row
/// Easy
///
/// Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
///
/// In the American keyboard:
///
/// * the first row consists of the characters "qwertyuiop",
/// * the second row consists of the characters "asdfghjkl", and
/// * the third row consists of the characters "zxcvbnm".
///
/// ![keyboard](./img/keyboard.png)
///
/// Example 1:
///
/// Input: words = ["Hello","Alaska","Dad","Peace"]
/// Output: ["Alaska","Dad"]
///
/// Example 2:
///
/// Input: words = ["omk"]
/// Output: []
///
/// Example 3:
///
/// Input: words = ["adsdf","sfd"]
/// Output: ["adsdf","sfd"]
///
///
/// Constraints:
/// * 1 <= words.length <= 20
/// * 1 <= words[i].length <= 100
/// * words[i] consists of English letters (both lowercase and uppercase).

pub struct Solution {}
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut results = vec![];

        let lines = vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"];

        for w in &words {
            for l in &lines {
                if w.to_lowercase().chars().all(|c| l.contains(c)) {
                    results.push(w.to_string());
                    break;
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_words_1() {
        let words = vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ];
        let output = vec!["Alaska".to_string(), "Dad".to_string()];
        assert_eq!(Solution::find_words(words), output);
    }

    #[test]
    fn test_find_words_2() {
        let words = vec!["omk".to_string()];
        let output: Vec<String> = vec![];
        assert_eq!(Solution::find_words(words), output);
    }

    #[test]
    fn test_find_words_3() {
        let words = vec!["adsdf".to_string(), "sfd".to_string()];
        let output = vec!["adsdf".to_string(), "sfd".to_string()];
        assert_eq!(Solution::find_words(words), output);
    }
}
