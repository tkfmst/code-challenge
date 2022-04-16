/// 461. Hamming Distance
/// Easy
///
/// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
///
/// Given two integers x and y, return the Hamming distance between them.
///
///
/// Example 1:
///
/// Input: x = 1, y = 4
/// Output: 2
/// Explanation:
/// 1   (0 0 0 1)
/// 4   (0 1 0 0)
/// ↑   ↑
/// The above arrows point to positions where the corresponding bits are different.
///
/// Example 2:
///
/// Input: x = 3, y = 1
/// Output: 1
///
/// Constraints:
/// * 0 <= x, y <= 2^31 - 1
///

pub struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_hamming_distance_1() {
        let x = 1;
        let y = 4;
        let output = 2;
        assert_eq!(Solution::hamming_distance(x, y), output);
    }

    #[test]
    fn test_hamming_distance_2() {
        let x = 3;
        let y = 1;
        let output = 1;
        assert_eq!(Solution::hamming_distance(x, y), output);
    }
}
