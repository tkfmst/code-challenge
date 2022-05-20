// 63. Unique Paths II
// Medium
//
// You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m-1][n-1]). The robot can only move either down or right at any point in time.
//
// An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.
//
// Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
//
// The testcases are generated so that the answer will be less than or equal to 2 * 109.
//
//
// Example 1:
// ![rotobo1](./img/robot1.jpg)
//
// Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
// Output: 2
// Explanation: There is one obstacle in the middle of the 3x3 grid above.
// There are two ways to reach the bottom-right corner:
// 1. Right -> Right -> Down -> Down
// 2. Down -> Down -> Right -> Right
//
// Example 2:
// ![rotobo2](./img/robot2.jpg)
//
// Input: obstacleGrid = [[0,1],[0,0]]
// Output: 1
//
//
// Constraints:
// * m == obstacleGrid.length
// * n == obstacleGrid[i].length
// * 1 <= m, n <= 100
// * obstacleGrid[i][j] is 0 or 1.
pub struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 1;

        obstacle_grid.iter().enumerate().for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x == 0)
                .for_each(|(j, _)| {
                    if i > 0 {
                        dp[i][j] += dp[i - 1][j];
                    }
                    if j > 0 {
                        dp[i][j] += dp[i][j - 1];
                    }
                })
        });

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_unique_paths_with_obstacles_1() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let output = 2;
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), output);
    }

    #[test]
    fn test_unique_paths_with_obstacles_2() {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        let output = 1;
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), output);
    }

    #[test]
    fn test_unique_paths_with_obstacles_3() {
        let obstacle_grid = vec![vec![1]];
        let output = 0;
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), output);
    }

    #[test]
    fn test_unique_paths_with_obstacles_4() {
        let obstacle_grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let output = 7;
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), output);
    }
}
