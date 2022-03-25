// 1029. Two City Scheduling
// Medium
//
// A company is planning to interview 2n people. Given the array costs where costs[i] = [aCosti, bCosti], the cost of flying the ith person to city a is aCosti, and the cost of flying the ith person to city b is bCosti.
//
// Return the minimum cost to fly every person to a city such that exactly n people arrive in each city.
//
//
// Example 1:
//
// Input: costs = [[10,20],[30,200],[400,50],[30,20]]
// Output: 110
// Explanation:
// The first person goes to city A for a cost of 10.
// The second person goes to city A for a cost of 30.
// The third person goes to city B for a cost of 50.
// The fourth person goes to city B for a cost of 20.
//
// The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.
//
// Example 2:
//
// Input: costs = [[259,770],[448,54],[926,667],[184,139],[840,118],[577,469]]
// Output: 1859
//
// Example 3:
//
// Input: costs = [[515,563],[451,713],[537,709],[343,819],[855,779],[457,60],[650,359],[631,42]]
// Output: 3086
// Constraints:
// * 2 * n == costs.length
// * 2 <= costs.length <= 100
// * costs.length is even.
// * 1 <= aCosti, bCosti <= 1000

pub struct Solution {}
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        // a-b = Refund when changing from A to B
        let mut costs_and_refund: Vec<(i32, i32, i32)> =
            costs.iter().map(|v| (v[0], v[1], (v[0] - v[1]))).collect();

        costs_and_refund.sort_by(|a, b| (&b.2).cmp(&a.2));

        for i in 0..costs_and_refund.len() {
            if i < costs.len() / 2 {
                result += costs_and_refund[i].1;
            } else {
                result += costs_and_refund[i].0;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_city_sched_cost_1() {
        let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
        let output = 110;
        assert_eq!(Solution::two_city_sched_cost(costs), output);
    }

    #[test]
    fn test_two_city_sched_cost_2() {
        let costs = vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469],
        ];
        let output = 1859;
        assert_eq!(Solution::two_city_sched_cost(costs), output);
    }

    #[test]
    fn test_two_city_sched_cost_3() {
        let costs = vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42],
        ];
        let output = 3086;
        assert_eq!(Solution::two_city_sched_cost(costs), output);
    }
}
