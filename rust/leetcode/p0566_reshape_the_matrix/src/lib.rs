/// 566. Reshape the Matrix
/// Easy
///
/// In MATLAB, there is a handy function called reshape which can reshape an m x n matrix into a new one with a different size r x c keeping its original data.
///
/// You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
///
/// The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
///
/// If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.
///
///
/// Example 1:
/// ![reshape1-grid](./img/reshape1-grid.jpg)
///
/// Input: mat = [[1,2],[3,4]], r = 1, c = 4
/// Output: [[1,2,3,4]]
///
/// Example 2:
/// ![reshape2-grid](./img/reshape2-grid.jpg)
///
/// Input: mat = [[1,2],[3,4]], r = 2, c = 4
/// Output: [[1,2],[3,4]]
///
///
/// Constraints:
/// * m == mat.length
/// * n == mat[i].length
/// * 1 <= m, n <= 100
/// * -1000 <= mat[i][j] <= 1000
/// * 1 <= r, c <= 300

pub struct Solution {}
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;

        if m * n != r * c {
            return mat;
        }

        mat.iter()
            .flat_map(|i| i.iter().copied())
            .collect::<Vec<_>>()
            .chunks(c)
            .map(|c| c.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_matrix_reshape_1() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let r = 1;
        let c = 4;
        let output = vec![vec![1, 2, 3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, r, c), output);
    }

    #[test]
    fn test_matrix_reshape_2() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let r = 2;
        let c = 4;
        let output = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, r, c), output);
    }
}
