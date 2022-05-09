/// 17. Letter Combinations of a Phone Number
/// Medium
///
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
///
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
///
///
/// Example 1:
///
/// Input: digits = "23"
/// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
///
/// Example 2:
///
/// Input: digits = ""
/// Output: []
///
/// Example 3:
///
/// Input: digits = "2"
/// Output: ["a","b","c"]
///
///
/// Constraints:
/// * 0 <= digits.length <= 4
/// * digits[i] is a digit in the range ['2', '9'].

pub struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        let mut res: Vec<Vec<u8>> = vec![vec![]];
        let digit_to_letters = vec![
            vec![b'a', b'b', b'c'], // idx=0
            vec![b'd', b'e', b'f'],
            vec![b'g', b'h', b'i'],
            vec![b'j', b'k', b'l'],
            vec![b'm', b'n', b'o'],
            vec![b'p', b'q', b'r', b's'],
            vec![b't', b'u', b'v'],
            vec![b'w', b'x', b'y', b'z'], // idx=7
        ];

        for d in digits.chars() {
            let letters = &digit_to_letters[((d as u8) - b'2') as usize];
            let mut buf = vec![];
            while let Some(v) = res.pop() {
                for i in 0..letters.len() {
                    let mut v2 = v.clone();
                    v2.push(letters[i]);
                    buf.push(v2);
                }
            }
            res = buf;
        }

        println!("{:?}", res);
        res.iter()
            .map(|v| v.iter().map(|&b| b as char).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_letter_combinations_1() {
        let digits = "23".to_string();
        let mut expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        expected.sort();
        let mut actual = Solution::letter_combinations(digits);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_letter_combinations_2() {
        let digits = "".to_string();
        let mut expected: Vec<String> = vec![];
        expected.sort();
        let mut actual = Solution::letter_combinations(digits);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_letter_combinations_3() {
        let digits = "2".to_string();
        let mut expected = vec!["a", "b", "c"];
        expected.sort();
        let mut actual = Solution::letter_combinations(digits);
        actual.sort();
        assert_eq!(actual, expected);
    }
}
