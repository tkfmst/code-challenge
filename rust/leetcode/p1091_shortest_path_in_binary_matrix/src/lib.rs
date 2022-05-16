/// 1091. Shortest Path in Binary Matrix
/// Medium
///
/// Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
///
/// A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
///
/// All the visited cells of the path are 0.
/// All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
///
/// The length of a clear path is the number of visited cells of this path.
///
///
/// Example 1:
/// ![example1_1](./img/example1_1.png)
/// Input: grid = [[0,1],[1,0]]
/// Output: 2
///
/// Example 2:
/// ![example2_1](./img/example2_1.png)
/// Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
/// Output: 4
///
/// Example 3:
///
/// Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
/// Output: -1
///
///
/// Constraints:
/// * n == grid.length
/// * n == grid[i].length
/// * 1 <= n <= 100
/// * grid[i][j] is 0 or 1

pub struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let (r, c) = (n, n);
        let mut result = vec![vec![None; n]; n];
        let mut vd = VecDeque::new();
        if grid[0][0] == 0 {
            vd.push_back(((0, 0), 1));
            result[0][0] = Some(1);
        }

        while let Some((pair, len)) = vd.pop_front() {
            // i & j is 8-directionally
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    if (0..r as i32).contains(&(pair.0 + i))
                        && (0..c as i32).contains(&(pair.1 + j))
                    {
                        let q = ((pair.0 + i) as usize, (pair.1 + j) as usize);
                        if grid[q.0][q.1] == 0 && result[q.0][q.1].is_none() {
                            result[q.0][q.1] = Some(len + 1);
                            vd.push_back(((pair.0 + i, pair.1 + j), len + 1));
                        }
                    }
                }
            }
        }

        result[r - 1][c - 1].unwrap_or(-1)
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_path_binary_matrix_1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let output = 2;
        assert_eq!(Solution::shortest_path_binary_matrix(grid), output);
    }

    #[test]
    fn test_shortest_path_binary_matrix_2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let output = 4;
        assert_eq!(Solution::shortest_path_binary_matrix(grid), output);
    }

    #[test]
    fn test_shortest_path_binary_matrix_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let output = -1;
        assert_eq!(Solution::shortest_path_binary_matrix(grid), output);
    }
}
