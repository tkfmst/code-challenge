// 785. Is Graph Bipartite?
// Medium
//
// There is an undirected graph with n nodes, where each node is numbered between 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes that node u is adjacent to. More formally, for each v in graph[u], there is an undirected edge between node u and node v. The graph has the following properties:
// * There are no self-edges (graph[u] does not contain u).
// * There are no parallel edges (graph[u] does not contain duplicate values).
// * If v is in graph[u], then u is in graph[v] (the graph is undirected).
// * The graph may not be connected, meaning there may be two nodes u and v such that there is no path between them.
//
// A graph is bipartite if the nodes can be partitioned into two independent sets A and B such that every edge in the graph connects a node in set A and a node in set B.
//
// Return true if and only if it is bipartite.
//
//
// Example 1:
//
// Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
// Output: false
// Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
//
// Example 2:
//
// Input: graph = [[1,3],[0,2],[1,3],[0,2]]
// Output: true
// Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
//
//
// Constraints:
// * graph.length == n
// * 1 <= n <= 100
// * 0 <= graph[u].length < n
// * 0 <= graph[u][i] <= n - 1
// * graph[u] does not contain u.
// * All the values of graph[u] are unique.
// * If graph[u] contains v, then graph[v] contains u.

pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut set_a: HashSet<i32> = HashSet::new();
        let mut set_b: HashSet<i32> = HashSet::new();
        let mut counted: HashSet<i32> = HashSet::new();

        for u in 0..graph.len() {
            if !Self::helper(u as i32, &graph, &mut set_a, &mut set_b, &mut counted) {
                return false;
            }
        }

        true
    }
    fn helper(
        u: i32,
        graph: &Vec<Vec<i32>>,
        set_a: &mut HashSet<i32>,
        set_b: &mut HashSet<i32>,
        counted: &mut HashSet<i32>,
    ) -> bool {
        if counted.contains(&(u as i32)) {
            return true;
        }

        println!("u: {:?}, set_a: {:?}, set_b:{:?}", u, set_a, set_b);
        counted.insert(u as i32);

        if graph[u as usize]
            .iter()
            .all(|n| (!set_a.contains(n) && !set_b.contains(n)) || set_a.contains(n))
        {
            set_b.insert(u);
            for i in 0..graph[u as usize].len() {
                set_a.insert(graph[u as usize][i]);
            }
        } else if graph[u as usize]
            .iter()
            .all(|n| (!set_a.contains(n) && !set_b.contains(n)) || set_b.contains(n))
        {
            set_a.insert(u);
            for i in 0..graph[u as usize].len() {
                set_b.insert(graph[u as usize][i]);
            }
        } else {
            return false;
        }

        for i in 0..graph[u as usize].len() {
            if !Self::helper(graph[u as usize][i], graph, set_a, set_b, counted) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_bipartite_1() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        let output = false;
        assert_eq!(Solution::is_bipartite(graph), output);
    }

    #[test]
    fn test_is_bipartite_2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        let output = true;
        assert_eq!(Solution::is_bipartite(graph), output);
    }

    #[test]
    fn test_is_bipartite_3() {
        let graph = vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]];
        let output = true;
        assert_eq!(Solution::is_bipartite(graph), output);
    }
}
