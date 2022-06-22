/// 598. Range Addition II
/// Easy
///
/// You are given an m x n matrix M initialized with all 0's and an array of operations ops, where ops[i] = [ai, bi] means M[x][y] should be incremented by one for all 0 <= x < ai and 0 <= y < bi.
///
/// Count and return the number of maximum integers in the matrix after performing all the operations.
///
///  
///
/// Example 1:
///
/// Input: m = 3, n = 3, ops = [[2,2],[3,3]]
/// Output: 4
/// Explanation: The maximum integer in M is 2, and there are four of it in M. So return 4.
///
/// Example 2:
///
/// Input: m = 3, n = 3, ops = [[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3]]
/// Output: 4
///
/// Example 3:
///
/// Input: m = 3, n = 3, ops = []
/// Output: 9
///
///  
///
/// Constraints:
/// * 1 <= m, n <= 4 * 10^4
/// * 0 <= ops.length <= 10^4
/// * ops[i].length == 2
/// * 1 <= ai <= m
/// * 1 <= bi <= n

pub struct Solution {}
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n;
        }

        let i = ops.iter().map(|a| a[0]).min().unwrap();
        let j = ops.iter().map(|a| a[1]).min().unwrap();

        i * j
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_count_1() {
        let m = 3;
        let n = 3;
        let ops = vec![vec![2, 2], vec![3, 3]];
        let output = 4;
        assert_eq!(Solution::max_count(m, n, ops), output);
    }

    #[test]
    fn test_max_count_2() {
        let m = 3;
        let n = 3;
        let ops = vec![
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
        ];
        let output = 4;
        assert_eq!(Solution::max_count(m, n, ops), output);
    }

    #[test]
    fn test_max_count_3() {
        let m = 3;
        let n = 3;
        let ops = vec![];
        let output = 9;
        assert_eq!(Solution::max_count(m, n, ops), output);
    }
}
