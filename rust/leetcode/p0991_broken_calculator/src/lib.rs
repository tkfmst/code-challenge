/// 991. Broken Calculator
/// Medium
///
/// There is a broken calculator that has the integer startValue on its display initially. In one operation, you can:
///
/// multiply the number on display by 2, or
/// subtract 1 from the number on display.
///
/// Given two integers startValue and target, return the minimum number of operations needed to display target on the calculator.
///
///
/// Example 1:
///
/// Input: startValue = 2, target = 3
/// Output: 2
/// Explanation: Use double operation and then decrement operation {2 -> 4 -> 3}.
///
/// Example 2:
///
/// Input: startValue = 5, target = 8
/// Output: 2
/// Explanation: Use decrement and then double {5 -> 4 -> 8}.
///
/// Example 3:
///
/// Input: startValue = 3, target = 10
/// Output: 3
/// Explanation: Use double, decrement and double {3 -> 6 -> 5 -> 10}.
///
///
/// Constraints:
/// * 1 <= x, y <= 10^9

pub struct Solution {}

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut i = 0;

        let mut target_mut = target;

        while start_value != target_mut {
            if start_value > target_mut {
                target_mut += 1;
            } else if start_value < target_mut {
                if target_mut % 2 == 0 {
                    target_mut /= 2;
                } else {
                    target_mut += 1;
                }
            }
            i += 1;
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_broken_calc_1() {
        let start_value = 2;
        let target = 3;
        let output = 2;
        assert_eq!(Solution::broken_calc(start_value, target), output);
    }

    #[test]
    fn test_broken_calc_2() {
        let start_value = 5;
        let target = 8;
        let output = 2;
        assert_eq!(Solution::broken_calc(start_value, target), output);
    }

    #[test]
    fn test_broken_calc_3() {
        let start_value = 3;
        let target = 10;
        let output = 3;
        assert_eq!(Solution::broken_calc(start_value, target), output);
    }
}
