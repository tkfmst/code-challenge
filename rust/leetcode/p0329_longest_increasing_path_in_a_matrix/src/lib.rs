/// 329. Longest Increasing Path in a Matrix
/// Hard
///
/// Given an m x n integers matrix, return the length of the longest increasing path in matrix.
///
/// From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
///
///
/// Example 1:
/// ![grid1](./img/grid1.jpg)
///
/// Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
/// Output: 4
/// Explanation: The longest increasing path is [1, 2, 6, 9].
///
/// Example 2:
/// ![tmp-grid](./img/tmp-grid.jpg)
///
/// Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
/// Output: 4
/// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
///
/// Example 3:
///
/// Input: matrix = [[1]]
/// Output: 1
///
///
/// Constraints:
/// * m == matrix.length
/// * n == matrix[i].length
/// * 1 <= m, n <= 200
/// * 0 <= matrix[i][j] <= 2^31 - 1

pub struct Solution {}
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut res = 0;
        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                res = res.max(Self::max_path(i, j, &mut dp, &matrix));
            }
        }
        res
    }

    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn max_path(x: usize, y: usize, dp: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>) -> i32 {
        if dp[x][y] > 0 {
            return dp[x][y];
        }
        let mut path = 1;
        for d in Self::DIRECTIONS.iter() {
            let i = x as i32 + d.0;
            let j = y as i32 + d.1;
            if i >= 0 && i < dp.len() as i32 && j >= 0 && j < dp[0].len() as i32 {
                if matrix[i as usize][j as usize] > matrix[x][y] {
                    path = path.max(Self::max_path(i as usize, j as usize, dp, matrix) + 1);
                }
            }
        }
        dp[x][y] = path;
        path
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_increasing_path_1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        let output = 4;
        assert_eq!(Solution::longest_increasing_path(matrix), output);
    }

    #[test]
    fn test_longest_increasing_path_2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        let output = 4;
        assert_eq!(Solution::longest_increasing_path(matrix), output);
    }

    #[test]
    fn test_longest_increasing_path_3() {
        let matrix = vec![vec![1]];
        let output = 1;
        assert_eq!(Solution::longest_increasing_path(matrix), output);
    }
}
