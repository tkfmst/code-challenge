/// 70. Climbing Stairs
/// Easy
///
/// You are climbing a staircase. It takes n steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
///
///  
///
/// Example 1:
///
/// Input: n = 2
/// Output: 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps
///
/// Example 2:
///
/// Input: n = 3
/// Output: 3
/// Explanation: There are three ways to climb to the top.
/// 1. 1 step + 1 step + 1 step
/// 2. 1 step + 2 steps
/// 3. 2 steps + 1 step
///

#[derive(Debug)]
pub struct Solution {}

/// Solution part1
/// 組み合わせ問題として解く
// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         calc(n as u128, 0) as i32
//     }
// }
//
// fn calc(n: u128, k: u128) -> u128 {
//     if n < k {
//         return 0;
//     }
//     comb(n, k) + calc(n - 1, k + 1)
// }
//
// fn comb(n: u128, k: u128) -> u128 {
//     if n < k {
//         panic!(
//             "combination args is invalid:n={} < k={} in `comb(n, k)`",
//             n, k
//         );
//     }
//
//     if k == 0 || n == k {
//         return 1;
//     }
//
//     let denom = ((n + 1 - k)..(n + 1)).fold(1, |acc, i| acc * i);
//     let numer = (1..(k + 1)).fold(1, |acc, i| acc * i);
//     denom / numer
// }

/// Solution part2
/// f(n) = f(n-1) + f(n-2)
///
/// e.g.
/// n=10の時、最初に1段昇り、残りを上る = f(n-1)
///           最初に2段昇り、残りを上る = f(n-2)
/// ∴ f(10) = f(9) + f(8)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(35), 14930352);
        assert_eq!(Solution::climb_stairs(44), 1134903170);
    }
}
