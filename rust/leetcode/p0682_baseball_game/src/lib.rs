/// 682. Baseball Game
/// Easy
///
/// You are keeping score for a baseball game with strange rules. The game consists of several rounds, where the scores of past rounds may affect future rounds' scores.
///
/// At the beginning of the game, you start with an empty record. You are given a list of strings ops, where ops[i] is the ith operation you must apply to the record and is one of the following:
///
/// An integer x - Record a new score of x.
/// "+" - Record a new score that is the sum of the previous two scores. It is guaranteed there will always be two previous scores.
/// "D" - Record a new score that is double the previous score. It is guaranteed there will always be a previous score.
/// "C" - Invalidate the previous score, removing it from the record. It is guaranteed there will always be a previous score.
///
/// Return the sum of all the scores on the record.
///
///
/// Example 1:
///
/// Input: ops = ["5","2","C","D","+"]
/// Output: 30
/// Explanation:
/// "5" - Add 5 to the record, record is now [5].
/// "2" - Add 2 to the record, record is now [5, 2].
/// "C" - Invalidate and remove the previous score, record is now [5].
/// "D" - Add 2 * 5 = 10 to the record, record is now [5, 10].
/// "+" - Add 5 + 10 = 15 to the record, record is now [5, 10, 15].
/// The total sum is 5 + 10 + 15 = 30.
///
/// Example 2:
///
/// Input: ops = ["5","-2","4","C","D","9","+","+"]
/// Output: 27
/// Explanation:
/// "5" - Add 5 to the record, record is now [5].
/// "-2" - Add -2 to the record, record is now [5, -2].
/// "4" - Add 4 to the record, record is now [5, -2, 4].
/// "C" - Invalidate and remove the previous score, record is now [5, -2].
/// "D" - Add 2 * -2 = -4 to the record, record is now [5, -2, -4].
/// "9" - Add 9 to the record, record is now [5, -2, -4, 9].
/// "+" - Add -4 + 9 = 5 to the record, record is now [5, -2, -4, 9, 5].
/// "+" - Add 9 + 5 = 14 to the record, record is now [5, -2, -4, 9, 5, 14].
/// The total sum is 5 + -2 + -4 + 9 + 5 + 14 = 27.
///
/// Example 3:
///
/// Input: ops = ["1"]
/// Output: 1
///
///
/// Constraints:
///
/// * 1 <= ops.length <= 1000
/// * ops[i] is "C", "D", "+", or a string representing an integer in the range [-3 * 10^4, 3 * 10^4].
/// * For operation "+", there will always be at least two previous scores on the record.
/// * For operations "C" and "D", there will always be at least one previous score on the record.

pub struct Solution {}
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut results = vec![];

        for op in ops {
            match op.as_str() {
                "D" => {
                    results.push(results[results.len() - 1] * 2);
                }
                "C" => {
                    results.pop();
                }
                "+" => {
                    results.push(results[results.len() - 1] + results[results.len() - 2]);
                }
                s => {
                    results.push(s.parse::<i32>().unwrap());
                }
            };
        }

        results.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cal_points_1() {
        let ops = vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string(),
        ];
        let output = 30;
        assert_eq!(Solution::cal_points(ops), output);
    }

    #[test]
    fn test_cal_points_2() {
        let ops = vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string(),
        ];
        let output = 27;
        assert_eq!(Solution::cal_points(ops), output);
    }
}
