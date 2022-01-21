/// 119. Pascal's Triangle II
///
/// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
/// ![PascalTriangleAnimated2](./img/PascalTriangleAnimated2.gif)
///
/// Example 1:
///
/// Input: rowIndex = 3
/// Output: [1,3,3,1]
///
/// Example 2:
///
/// Input: rowIndex = 0
/// Output: [1]
///
/// Example 3:
///
/// Input: rowIndex = 1
/// Output: [1,1]
///
///
///
/// Constraints:
/// - 0 <= rowIndex <= 33
///
/// Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => vec![1],
            1 => vec![1, 1],
            _ => Self::next_row(vec![1, 1], row_index - 1),
        }
    }
    fn next_row(row: Vec<i32>, row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            row
        } else {
            let mut v: Vec<i32> = vec![1];

            for (i, val) in row.iter().enumerate() {
                if let Some(nxt) = row.get(i + 1) {
                    v.push(val + nxt);
                } else {
                    v.push(1)
                }
            }

            Self::next_row(v, row_index - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_row_1() {
        let input = 3;
        let output = vec![1, 3, 3, 1];
        assert_eq!(Solution::get_row(input), output);
    }

    #[test]
    fn test_get_row_2() {
        let input = 0;
        let output = vec![1];
        assert_eq!(Solution::get_row(input), output);
    }

    #[test]
    fn test_get_row_3() {
        let input = 1;
        let output = vec![1, 1];
        assert_eq!(Solution::get_row(input), output);
    }
}
