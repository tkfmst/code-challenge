#![allow(non_snake_case)]
/// 374. Guess Number Higher or Lower
/// Easy
///
/// We are playing the Guess Game. The game is as follows:
///
/// I pick a number from 1 to n. You have to guess which number I picked.
///
/// Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
///
/// You call a pre-defined API int guess(int num), which returns three possible results:
/// * -1: Your guess is higher than the number I picked (i.e. num > pick).
/// * 1: Your guess is lower than the number I picked (i.e. num < pick).
/// * 0: your guess is equal to the number I picked (i.e. num == pick).
///
/// Return the number that I picked.
///
///
/// Example 1:
///
/// Input: n = 10, pick = 6
/// Output: 6
///
/// Example 2:
///
/// Input: n = 1, pick = 1
/// Output: 1
///
/// Example 3:
///
/// Input: n = 2, pick = 1
/// Output: 1
///
///
/// Constraints:
/// * 1 <= n <= 2^31 - 1
/// * 1 <= pick <= n

pub struct Solution {}
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    pub unsafe fn guessNumber(n: i32) -> i32 {
        Self::helper(1, n)
    }
    fn helper(start: i32, end: i32) -> i32 {
        if start == end {
            return start as i32;
        }

        let mid = start + (end - start) / 2;
        unsafe {
            match guess(mid) {
                -1 => Self::helper(start, mid - 1),
                1 => Self::helper(mid + 1, end),
                _ => mid,
            }
        }
    }
}

// for test
unsafe fn guess(num: i32) -> i32 {
    // // ex1
    // let pick = 6;

    // // ex2, 3
    // let pick = 1;

    // ex4
    let pick = 2;
    if num < pick {
        1
    } else if num > pick {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[test]
    // fn test_guess_number_1() {
    //     let n = 10;
    //     let pick: i32 = 6;
    //     let output = unsafe { Solution::guessNumber(n) };
    //     assert_eq!(output, pick);
    // }
    //
    // #[test]
    // fn test_guess_number_2() {
    //     let n = 1;
    //     let pick: i32 = 1;
    //     let output = unsafe { Solution::guessNumber(n) };
    //     assert_eq!(output, pick);
    // }
    //
    // #[test]
    // fn test_guess_number_3() {
    //     let n = 2;
    //     let pick: i32 = 1;
    //     let output = unsafe { Solution::guessNumber(n) };
    //     assert_eq!(output, pick);
    // }

    #[test]
    fn test_guess_number_4() {
        let n = 2;
        let pick: i32 = 2;
        let output = unsafe { Solution::guessNumber(n) };
        assert_eq!(output, pick);
    }
}
