// 867. Transpose Matrix
// Easy
//
// Given a 2D integer array matrix, return the transpose of matrix.
//
// The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
//
// ![hint_transpose](./img/hint_transpose.png)
//
// Example 1:
//
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[1,4,7],[2,5,8],[3,6,9]]
//
// Example 2:
//
// Input: matrix = [[1,2,3],[4,5,6]]
// Output: [[1,4],[2,5],[3,6]]
//
//
// Constraints:
//
// * m == matrix.length
// * n == matrix[i].length
// * 1 <= m, n <= 1000
// * 1 <= m * n <= 105
// * -10^9 <= matrix[i][j] <= 10^9

pub struct Solution {}
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut flipped: Vec<Vec<i32>> = vec![vec![]; matrix[0].len()];

        for m in 0..matrix.len() {
            for n in 0..matrix[0].len() {
                flipped[n].push(matrix[m][n]);
            }
        }

        flipped
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_transpose_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(Solution::transpose(matrix), output);
    }

    #[test]
    fn test_transpose_2() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let output = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), output);
    }
}
