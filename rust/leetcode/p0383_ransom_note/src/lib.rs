/// 383. Ransom Note
/// Easy
///
/// Given two strings ransomNote and magazine, return true if ransomNote can be constructed from magazine and false otherwise.
///
/// Each letter in magazine can only be used once in ransomNote.
///
///
///
/// Example 1:
///
/// Input: ransomNote = "a", magazine = "b"
/// Output: false
///
/// Example 2:
///
/// Input: ransomNote = "aa", magazine = "ab"
/// Output: false
///
/// Example 3:
///
/// Input: ransomNote = "aa", magazine = "aab"
/// Output: true
///
///
///
/// Constraints:
///
/// * 1 <= ransomNote.length, magazine.length <= 10^5
/// * ransomNote and magazine consist of lowercase English letters.

pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts: [u16; 26] = [0; 26];

        for &b in magazine.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        for &b in ransom_note.as_bytes() {
            if counts[(b - b'a') as usize] == 0 {
                return false;
            }
            counts[(b - b'a') as usize] -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_construct_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        let output = false;
        assert_eq!(Solution::can_construct(ransom_note, magazine), output);
    }

    #[test]
    fn test_can_construct_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        let output = false;
        assert_eq!(Solution::can_construct(ransom_note, magazine), output);
    }

    #[test]
    fn test_can_construct_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        let output = true;
        assert_eq!(Solution::can_construct(ransom_note, magazine), output);
    }

    #[test]
    fn test_can_construct_4() {
        let ransom_note = "aab".to_string();
        let magazine = "baa".to_string();
        let output = true;
        assert_eq!(Solution::can_construct(ransom_note, magazine), output);
    }

    #[test]
    fn test_can_construct_5() {
        let ransom_note = "bg".to_string();
        let magazine = "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj".to_string();
        let output = true;
        assert_eq!(Solution::can_construct(ransom_note, magazine), output);
    }
}
