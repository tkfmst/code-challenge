/// 171. Excel Sheet Column Number
///
/// Given a string columnTitle that represents the column title as appear in an Excel sheet, return its corresponding column number.
///
/// For example:
///
/// A -> 1
/// B -> 2
/// C -> 3
/// ...
/// Z -> 26
/// AA -> 27
/// AB -> 28
/// ...
///
///  
///
/// Example 1:
///
/// Input: columnTitle = "A"
/// Output: 1
///
/// Example 2:
///
/// Input: columnTitle = "AB"
/// Output: 28
///
/// Example 3:
///
/// Input: columnTitle = "ZY"
/// Output: 701
///
///  
///
/// Constraints:
///
/// * 1 <= columnTitle.length <= 7
/// * columnTitle consists only of uppercase English letters.
/// * columnTitle is in the range ["A", "FXSHRXW"].

pub struct Solution {}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let init = (0u32, 0i32);
        let tuple = column_title.chars().rev().fold(init, |acc, x| {
            ((acc.0 + 1), (acc.1 + ((x as i32) - 64) * 26i32.pow(acc.0)))
        });
        tuple.1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_title_to_number_1() {
        let input = String::from("A");
        let output = 1;
        assert_eq!(Solution::title_to_number(input), output);
    }

    #[test]
    fn test_title_to_number_2() {
        let input = String::from("AB");
        let output = 28;
        assert_eq!(Solution::title_to_number(input), output);
    }

    #[test]
    fn test_title_to_number_3() {
        let input = String::from("ZY");
        let output = 701;
        assert_eq!(Solution::title_to_number(input), output);
    }
}
