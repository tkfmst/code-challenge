// 1046. Last Stone Weight
// Easy
//
// You are given an array of integers stones where stones[i] is the weight of the ith stone.
//
// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
//
// * If x == y, both stones are destroyed, and
// * If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
//
// At the end of the game, there is at most one stone left.
//
// Return the smallest possible weight of the left stone. If there are no stones left, return 0.
//
//
// Example 1:
//
// Input: stones = [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
//
// Example 2:
//
// Input: stones = [1]
// Output: 1
//
//
// Constraints:
// * 1 <= stones.length <= 30
// * 1 <= stones[i] <= 1000
pub struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(stones.len());
        for s in stones {
            heap.push(s);
        }

        while heap.len() > 1 {
            let s1 = heap.pop().unwrap();
            let s2 = heap.pop().unwrap();
            if s1 == s2 {
                continue;
            } else {
                let ss = s1 - s2;
                heap.push(ss);
            }
        }

        if heap.len() == 1 {
            heap.pop().unwrap()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_last_stone_weight_1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let output = 1;
        assert_eq!(Solution::last_stone_weight(stones), output);
    }

    #[test]
    fn test_last_stone_weight_2() {
        let stones = vec![1];
        let output = 1;
        assert_eq!(Solution::last_stone_weight(stones), output);
    }

    #[test]
    fn test_last_stone_weight_3() {
        let stones = vec![1, 2, 7];
        let output = 4;
        assert_eq!(Solution::last_stone_weight(stones), output);
    }
}
