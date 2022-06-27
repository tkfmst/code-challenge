/// 605. Can Place Flowers
/// Easy
///
/// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
///
/// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.
///
///
/// Example 1:
///
/// Input: flowerbed = [1,0,0,0,1], n = 1
/// Output: true
///
/// Example 2:
///
/// Input: flowerbed = [1,0,0,0,1], n = 2
/// Output: false
///
///
/// Constraints:
///
/// * 1 <= flowerbed.length <= 2 * 10^4
/// * flowerbed[i] is 0 or 1.
/// * There are no two adjacent flowers in flowerbed.
/// * 0 <= n <= flowerbed.length

pub struct Solution {}
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut flowerbed = flowerbed;
        flowerbed.insert(0, 0);
        flowerbed.push(0);

        for i in 1..(flowerbed.len() - 1) {
            if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }
        }
        n <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_place_flowers_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let output = true;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), output);
    }

    #[test]
    fn test_can_place_flowers_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let output = false;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), output);
    }
}
