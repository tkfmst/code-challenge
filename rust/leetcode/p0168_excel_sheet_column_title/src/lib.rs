/// 168. Excel Sheet Column Title
///
/// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
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
/// Input: columnNumber = 1
/// Output: "A"
///
/// Example 2:
///
/// Input: columnNumber = 28
/// Output: "AB"
///
/// Example 3:
///
/// Input: columnNumber = 701
/// Output: "ZY"
///
///  
///
/// Constraints:
///
/// * 1 <= columnNumber <= 231 - 1

pub struct Solution {}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut acc: Vec<char> = vec![];
        let mut x = column_number;
        while x > 0 {
            let mut y = x % 26;
            if y == 0 {
                y = 26;
            }
            acc.push((y + 64) as u8 as char);
            x = (x - y) / 26;
        }

        println!("{:?}", acc);

        acc.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_convert_to_title_1() {
        let input = 1;
        let output = String::from("A");
        assert_eq!(Solution::convert_to_title(input), output);
    }

    #[test]
    fn test_convert_to_title_2() {
        let input = 28;
        let output = String::from("AB");
        assert_eq!(Solution::convert_to_title(input), output);
    }

    #[test]
    fn test_convert_to_title_3() {
        let input = 701;
        let output = String::from("ZY");
        assert_eq!(Solution::convert_to_title(input), output);
    }

    #[test]
    fn test_convert_to_title_4() {
        let input = 26;
        let output = String::from("Z");
        assert_eq!(Solution::convert_to_title(input), output);
    }

    #[test]
    fn test_convert_to_title_5() {
        let input = 27;
        let output = String::from("AA");
        assert_eq!(Solution::convert_to_title(input), output);
    }

    #[test]
    fn test_convert_to_title_6() {
        let input = 2147483647;
        let output = String::from("FXSHRXW");
        assert_eq!(Solution::convert_to_title(input), output);
    }
}
