// 354. Russian Doll Envelopes
// Hard
//
// You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
//
// One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
//
// Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
//
// Note: You cannot rotate an envelope.
//
//
// Example 1:
//
// Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
// Output: 3
// Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
//
// Example 2:
//
// Input: envelopes = [[1,1],[1,1],[1,1]]
// Output: 1
//
//
// Constraints:
// * 1 <= envelopes.length <= 105
// * envelopes[i].length == 2
// * 1 <= wi, hi <= 105

pub struct Solution {}
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|wh1, wh2| wh1[0].cmp(&wh2[0]).then(wh2[1].cmp(&wh1[1])));
        envelopes
            .into_iter()
            .map(|wh| wh[1])
            .fold(vec![], |mut dp, num| {
                match dp.binary_search(&num) {
                    Err(i) if i == dp.len() => dp.push(num),
                    Err(i) => dp[i] = num,
                    _ => (),
                }
                dp
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_envelopes_1() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        let output = 3;
        assert_eq!(Solution::max_envelopes(envelopes), output);
    }

    #[test]
    fn test_max_envelopes_2() {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        let output = 1;
        assert_eq!(Solution::max_envelopes(envelopes), output);
    }
}
