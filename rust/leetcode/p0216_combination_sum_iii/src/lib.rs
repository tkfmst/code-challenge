/// 216. Combination Sum III
/// Medium
///
/// Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
///
/// * Only numbers 1 through 9 are used.
/// * Each number is used at most once.
///
/// Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
///
///
/// Example 1:
///
/// Input: k = 3, n = 7
/// Output: [[1,2,4]]
/// Explanation:
/// 1 + 2 + 4 = 7
/// There are no other valid combinations.
///
/// Example 2:
///
/// Input: k = 3, n = 9
/// Output: [[1,2,6],[1,3,5],[2,3,4]]
/// Explanation:
/// 1 + 2 + 6 = 9
/// 1 + 3 + 5 = 9
/// 2 + 3 + 4 = 9
/// There are no other valid combinations.
///
/// Example 3:
///
/// Input: k = 4, n = 1
/// Output: []
/// Explanation: There are no valid combinations.
/// Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.
///
///
/// Constraints:
/// * 2 <= k <= 9
/// * 1 <= n <= 60

pub struct Solution {}
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn comb(k: usize, n: i32, mut cur: i32, l: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if l.len() == k {
                return;
            }

            while cur <= 9 {
                let mut c = l.clone();
                let s = c.iter().sum::<i32>() + cur;

                if n < s {
                    break;
                }

                c.push(cur);
                if s == n && c.len() == k {
                    res.push(c.clone());
                    break;
                }

                comb(k, n, cur + 1, &mut c, res);
                cur += 1;
            }
        }

        let (mut l, mut res) = (vec![], vec![]);
        comb(k as usize, n, 1, &mut l, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum3_1() {
        let k = 3;
        let n = 7;
        let output = vec![vec![1, 2, 4]];
        assert_eq!(Solution::combination_sum3(k, n), output);
    }

    #[test]
    fn test_combination_sum3_2() {
        let k = 4;
        let n = 1;
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum3(k, n), output);
    }
}
