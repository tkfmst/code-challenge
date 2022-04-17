/// 463. Island Perimeter
/// Easy
///
/// You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.
///
/// Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
///
/// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
///
///
/// Example 1:
/// ![island.png](./img/island.png)
///
/// Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
/// Output: 16
/// Explanation: The perimeter is the 16 yellow stripes in the image above.
///
/// Example 2:
///
/// Input: grid = [[1]]
/// Output: 4
///
/// Example 3:
///
/// Input: grid = [[1,0]]
/// Output: 4
///
///
/// Constraints:
///
/// * row == grid.length
/// * col == grid[i].length
/// * 1 <= row, col <= 100
/// * grid[i][j] is 0 or 1.
/// * There is exactly one island in grid.

pub struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut land = 0;
        let mut neighbor = 0;

        let row_len = grid.len();
        let col_len = grid[0].len();

        // ↓: i, →: j
        for i in 0..row_len {
            for j in 0..col_len {
                if grid[i][j] == 1 {
                    land += 1;
                    if i < row_len - 1 && grid[i + 1][j] == 1 {
                        neighbor += 1;
                    }
                    if j < col_len - 1 && grid[i][j + 1] == 1 {
                        neighbor += 1;
                    }
                }
            }
        }

        // Why is my neighbor x2?
        // Because...
        //  □ ← with this bottom perimeter
        //  □ ← and this upper perimeter
        //
        // Same for left and right
        land * 4 - neighbor * 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_island_perimeter_1() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        let output = 16;
        assert_eq!(Solution::island_perimeter(grid), output);
    }

    #[test]
    fn test_island_perimeter_2() {
        let grid = vec![vec![1, 0]];
        let output = 4;
        assert_eq!(Solution::island_perimeter(grid), output);
    }
}
