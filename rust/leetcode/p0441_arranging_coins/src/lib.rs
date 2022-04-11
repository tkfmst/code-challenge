/// 441. Arranging Coins
/// Easy
///
/// You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the ith row has exactly i coins. The last row of the staircase may be incomplete.
///
/// Given the integer n, return the number of complete rows of the staircase you will build.
///
///
/// Example 1:
/// ![arrangecoins1](./img/arrangecoins1-grid.jpg)
///
/// Input: n = 5
/// Output: 2
/// Explanation: Because the 3rd row is incomplete, we return 2.
///
/// Example 2:
/// ![arrangecoins2](./img/arrangecoins2-grid.jpg)
///
/// Input: n = 8
/// Output: 3
/// Explanation: Because the 4th row is incomplete, we return 3.
///
///
/// Constraints:
/// * 1 <= n <= 2^31 - 1

pub struct Solution {}
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // // answer 1
        // let n_i64 = n as i64;
        // let mut l = 0;
        // let mut r = n_i64;
        //
        // while l <= r {
        //     println!("l: {}, r: {}", l, r);
        //     let step = l + (r - l) / 2;
        //
        //     let coins = step * (step + 1) / 2;
        //
        //     if coins == n_i64 {
        //         return step as i32;
        //     }
        //
        //     if n_i64 < coins {
        //         r = step - 1;
        //     } else {
        //         l = step + 1;
        //     }
        // }
        //
        // r as i32

        // answer 2
        // k * (k + 1) / 2 <= N
        // (k + 1/2)^2 - 1/4 <= 2N
        //
        // ∴ k = sqrt(2N + 1/4) - 1/2
        let k = f64::sqrt(2.0 * n as f64 + 0.25) - 0.5;
        println!("{}", k);
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_arrange_coins_1() {
        let n = 5;
        let output = 2;
        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn test_arrange_coins_2() {
        let n = 8;
        let output = 3;
        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn test_arrange_coins_3() {
        let n = 1;
        let output = 1;
        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn test_arrange_coins_4() {
        let n = 2147483647;
        let output = 65535;
        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn test_arrange_coins_5() {
        let n = 1804289383;
        let output = 60070;
        assert_eq!(Solution::arrange_coins(n), output);
    }

    #[test]
    fn test_arrange_coins_6() {
        let n = 2146467959;
        let output = 65519;
        assert_eq!(Solution::arrange_coins(n), output);
    }
}
