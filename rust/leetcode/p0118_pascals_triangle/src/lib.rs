/// 118. Pascal's Triangle
/// Easy
///
/// Given an integer numRows, return the first numRows of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
///  
///
/// Example 1:
/// ![PascalTriangleAnimated2.gif](./img/PascalTriangleAnimated2.gif)
///
/// Input: numRows = 5
/// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
///
/// Example 2:
///
/// Input: numRows = 1
/// Output: [[1]]
///
///  
///
/// Constraints:
///
/// - 1 <= numRows <= 30
///
pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => {
                let mut v = vec![vec![1], vec![1, 1]];
                Self::build(&mut v, num_rows - 2)
            }
        }
    }

    fn build(triangle: &mut Vec<Vec<i32>>, nums_rows: i32) -> Vec<Vec<i32>> {
        if nums_rows == 0 {
            triangle.to_vec()
        } else {
            let mut buf: Vec<i32> = vec![1];
            if let Some(last) = triangle.last() {
                for (i, val) in last.iter().enumerate() {
                    if let Some(n) = last.get(i + 1) {
                        buf.push(val + *n);
                    }
                }
            }
            buf.push(1);
            triangle.push(buf);
            Self::build(triangle, nums_rows - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate1() {
        let input = 5;
        let output = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(input), output);
    }

    #[test]
    fn test_generate2() {
        let input = 1;
        let output = vec![vec![1]];
        assert_eq!(Solution::generate(input), output);
    }
}
