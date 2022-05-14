/// 743. Network Delay Time
/// Medium
///
/// You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
///
/// We will send a signal from a given node k. Return the time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.
///
///
/// Example 1:
/// ![931_example_1](./img/931_example_1.png)
///
/// Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
/// Output: 2
///
/// Example 2:
///
/// Input: times = [[1,2,1]], n = 2, k = 1
/// Output: 1
///
/// Example 3:
///
/// Input: times = [[1,2,1]], n = 2, k = 2
/// Output: -1
///
///
/// Constraints:
///   * 1 <= k <= n <= 100
///   * 1 <= times.length <= 6000
///   * times[i].length == 3
///   * 1 <= ui, vi <= n
///   * ui != vi
///   * 0 <= wi <= 100
///   * All the pairs (ui, vi) are unique. (i.e., no multiple edges.)

pub struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let graph: Vec<Vec<(usize, i32)>> = {
            let mut v = vec![vec![]; n + 1];
            for t in times {
                v[t[0] as usize].push((t[1] as usize, t[2]));
            }
            v
        };

        // Dijkstra's algorithm
        let mut dists = vec![i32::MAX; n + 1];
        dists[k] = 0;
        let mut visited = vec![false; n + 1];
        let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), k));

        while let Some((Reverse(dist), u)) = pq.pop() {
            if visited[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let new_dist = w + dist;
                if new_dist < dists[v] {
                    dists[v] = new_dist;
                    pq.push((Reverse(new_dist), v));
                }
            }
            visited[u] = true;
        }

        if !visited[1..].iter().all(|&a| a) {
            return -1;
        }
        *dists[1..].iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_network_delay_time_1() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        let output = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), output);
    }

    #[test]
    fn test_network_delay_time_2() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        let output = 1;
        assert_eq!(Solution::network_delay_time(times, n, k), output);
    }

    #[test]
    fn test_network_delay_time_3() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        let output = -1;
        assert_eq!(Solution::network_delay_time(times, n, k), output);
    }
}
