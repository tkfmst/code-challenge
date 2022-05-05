/// 506. Relative Ranks
/// Easy
///
/// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
///
/// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:
///
/// * The 1st place athlete's rank is "Gold Medal".
/// * The 2nd place athlete's rank is "Silver Medal".
/// * The 3rd place athlete's rank is "Bronze Medal".
/// * For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
///
/// Return an array answer of size n where answer[i] is the rank of the ith athlete.
///
///
/// Example 1:
///
/// Input: score = [5,4,3,2,1]
/// Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
/// Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].
///
/// Example 2:
///
/// Input: score = [10,3,8,9,4]
/// Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
/// Explanation: The placements are [1st, 5th, 3rd, 2nd, 4th].
///
///
/// Constraints:
/// * n == score.length
/// * 1 <= n <= 10^4
/// * 0 <= score[i] <= 10^6
/// * All the values in score are unique.

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        let mut hash = HashMap::with_capacity(score.len());

        let mut score_mut = score.clone();
        score_mut.sort_by(|a, b| b.cmp(a));

        for (rnk, scr) in score_mut.iter().enumerate() {
            hash.insert(scr, rnk);
        }

        for i in 0..score.len() {
            if let Some(rnk) = hash.get(&score[i]) {
                res.push(match rnk {
                    0 => "Gold Medal".to_string(),
                    1 => "Silver Medal".to_string(),
                    2 => "Bronze Medal".to_string(),
                    _ => (rnk + 1).to_string(),
                })
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_relative_ranks_1() {
        let score = vec![5, 4, 3, 2, 1];
        let output = vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"];
        assert_eq!(Solution::find_relative_ranks(score), output);
    }

    #[test]
    fn test_find_relative_ranks_2() {
        let score = vec![10, 3, 8, 9, 4];
        let output = vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"];
        assert_eq!(Solution::find_relative_ranks(score), output);
    }
}
