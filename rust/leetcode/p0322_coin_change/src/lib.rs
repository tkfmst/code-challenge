/// 322. Coin Change
/// Medium
///
/// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
///
/// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
///
/// You may assume that you have an infinite number of each kind of coin.
///
///  
///
/// Example 1:
///
/// Input: coins = [1,2,5], amount = 11
/// Output: 3
/// Explanation: 11 = 5 + 5 + 1
///
/// Example 2:
///
/// Input: coins = [2], amount = 3
/// Output: -1
///
/// Example 3:
///
/// Input: coins = [1], amount = 0
/// Output: 0
///
///  
///
/// Constraints:
///
/// 1 <= coins.length <= 12
/// 1 <= coins[i] <= 2^31 - 1
/// 0 <= amount <= 10^4

pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;

        for a in 1..=amount {
            for &c in coins.iter() {
                if a >= c {
                    dp[a as usize] = dp[a as usize].min(1 + dp[(a - c) as usize]);
                }
            }
        }

        if dp[amount as usize] != amount + 1 {
            dp[amount as usize]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_coin_change_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let output = 3;
        assert_eq!(Solution::coin_change(coins, amount), output);
    }

    #[test]
    fn test_coin_change_2() {
        let coins = vec![2];
        let amount = 3;
        let output = -1;
        assert_eq!(Solution::coin_change(coins, amount), output);
    }

    #[test]
    fn test_coin_change_3() {
        let coins = vec![1];
        let amount = 0;
        let output = 0;
        assert_eq!(Solution::coin_change(coins, amount), output);
    }

    #[test]
    fn test_coin_change_4() {
        let coins = vec![2];
        let amount = 1;
        let output = -1;
        assert_eq!(Solution::coin_change(coins, amount), output);
    }

    #[test]
    fn test_coin_change_5() {
        let coins = vec![186, 419, 83, 408];
        let amount = 6249;
        let output = 20;
        assert_eq!(Solution::coin_change(coins, amount), output);
    }
}
