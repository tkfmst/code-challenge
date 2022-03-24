/// 881. Boats to Save People
/// Medium
///
/// You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.
///
/// Return the minimum number of boats to carry every given person.
///
///
/// Example 1:
///
/// Input: people = [1,2], limit = 3
/// Output: 1
/// Explanation: 1 boat (1, 2)
///
/// Example 2:
///
/// Input: people = [3,2,2,1], limit = 3
/// Output: 3
/// Explanation: 3 boats (1, 2), (2) and (3)
///
/// Example 3:
///
/// Input: people = [3,5,3,4], limit = 5
/// Output: 4
/// Explanation: 4 boats (3), (3), (4), (5)
///
///
/// Constraints:
///
/// * 1 <= people.length <= 5 * 10^4
/// * 1 <= people[i] <= limit <= 3 * 10^4

pub struct Solution {}
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let sorted = {
            let mut tmp = people;
            tmp.sort();
            tmp
        };

        let mut i = 0;
        let mut j = sorted.len() - 1;

        while i < j {
            if sorted[i] + sorted[j] <= limit {
                i += 1;
            }
            j -= 1;
        }

        (sorted.len() - i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_rescue_boats_1() {
        let people = vec![1, 2];
        let limit = 3;
        let output = 1;
        assert_eq!(Solution::num_rescue_boats(people, limit), output);
    }

    #[test]
    fn test_num_rescue_boats_2() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        let output = 3;
        assert_eq!(Solution::num_rescue_boats(people, limit), output);
    }

    #[test]
    fn test_num_rescue_boats_3() {
        let people = vec![3, 5, 3, 4];
        let limit = 5;
        let output = 4;
        assert_eq!(Solution::num_rescue_boats(people, limit), output);
    }

    #[test]
    fn test_num_rescue_boats_4() {
        let people = vec![21, 40, 16, 24, 30];
        let limit = 50;
        let output = 3;
        assert_eq!(Solution::num_rescue_boats(people, limit), output);
    }

    #[test]
    fn test_num_rescue_boats_5() {
        let people = vec![3, 2, 3, 2, 2];
        let limit = 6;
        let output = 3;
        assert_eq!(Solution::num_rescue_boats(people, limit), output);
    }
}
