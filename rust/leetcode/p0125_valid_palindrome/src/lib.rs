/// 125. Valid Palindrome
///
/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
///
/// Given a string s, return true if it is a palindrome, or false otherwise.
///
///  
///
/// Example 1:
///
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
///
/// Example 2:
///
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
///
/// Example 3:
///
/// Input: s = " "
/// Output: true
/// Explanation: s is an empty string "" after removing non-alphanumeric characters.
/// Since an empty string reads the same forward and backward, it is a palindrome.
///
///  
///
/// Constraints:
///
/// * 1 <= s.length <= 2 * 105
/// * s consists only of printable ASCII characters.

pub struct Solution {}

// immutable
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cs: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let i = 0;
        let j = (cs.len() as i32 - 1).max(0) as usize;

        Self::equals(&cs, i, j)
    }

    fn equals(cs: &Vec<char>, i: usize, j: usize) -> bool {
        if i < j {
            cs[i] == cs[j] && Self::equals(cs, i + 1, j - 1)
        } else {
            true
        }
    }
}

// // mutable
// impl Solution {
//     pub fn is_palindrome(s: String) -> bool {
//         let cs: Vec<char> = s
//             .to_lowercase()
//             .chars()
//             .filter(|c| c.is_alphanumeric())
//             .collect();
//
//         let mut i = 0;
//         let mut j = (cs.len() as i32 - 1).max(0) as usize;
//
//         while i < j {
//             if cs[i] != cs[j] {
//                 return false;
//             }
//             i = i + 1;
//             j = j - 1;
//         }
//
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_palindrome1() {
        let input = String::from("A man, a plan, a canal: Panama");
        let output = true;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_is_palindrome2() {
        let input = String::from("race a car");
        let output = false;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_is_palindrome3() {
        let input = String::from(" ");
        let output = true;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_is_palindrome4() {
        let input = String::from("0P");
        let output = false;
        assert_eq!(Solution::is_palindrome(input), output);
    }
}
