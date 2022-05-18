/// 1192. Critical Connections in a Network
/// Hard
///
/// There are n servers numbered from 0 to n - 1 connected by undirected server-to-server connections forming a network where connections[i] = [ai, bi] represents a connection between servers ai and bi. Any server can reach other servers directly or indirectly through the network.
///
/// A critical connection is a connection that, if removed, will make some servers unable to reach some other server.
///
/// Return all critical connections in the network in any order.
///
///
/// #[cfg(test)mple 1:
///
/// Input: n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
/// Output: [[1,3]]
/// Explanation: [[3,1]] is also accepted.
///
/// Example 2:
///
/// Input: n = 2, connections = [[0,1]]
/// Output: [[0,1]]
///
///
/// Constraints:
///
/// * 2 <= n <= 105
/// * n - 1 <= connections.length <= 105
/// * 0 <= ai, bi <= n - 1
/// * ai != bi
/// * There are no repeated connections.

pub struct Solution {}

use std::collections::HashMap;
type GraphUnweightedAL = HashMap<usize, Vec<usize>>;

impl Solution {
    fn make_graph(n: usize, conn: &Vec<Vec<usize>>) -> GraphUnweightedAL {
        let mut map = HashMap::new();
        (0..n).for_each(|i| {
            map.insert(i, vec![]);
        });
        conn.into_iter().for_each(|edge| {
            let (u, v) = (edge[0], edge[1]);
            (*map.get_mut(&u).unwrap()).push(v.clone());
            (*map.get_mut(&v).unwrap()).push(u.clone());
        });
        map
    }

    fn find_bridges_util(
        u: &usize,
        g: &GraphUnweightedAL,
        parent: &mut Vec<Option<usize>>,
        vis: &mut Vec<bool>,
        dfn: &mut Vec<usize>,
        low: &mut Vec<usize>,
        time: &mut usize,
        result: &mut Vec<Vec<usize>>,
    ) {
        vis[*u] = true;
        *time += 1;
        dfn[*u] = *time;
        low[*u] = *time;
        for v in g.get(u).unwrap() {
            if !vis[*v] {
                parent[*v] = Some(*u);
                Self::find_bridges_util(v, g, parent, vis, dfn, low, time, result);
                // by this time we know if v has a back edge to an earlier component
                low[*u] = std::cmp::min(low[*u], low[*v]);
                // this basically means that there is a cycle below u and lowest point
                // reachable in the dfs tree from v is below u
                if low[*v] > dfn[*u] {
                    result.push(vec![*u, *v]);
                }
                continue;
            }
            // if the already visited neighbor is not the parent of v, then that means
            // there is a cycle
            if parent[*u] != Some(*v) {
                low[*u] = std::cmp::min(low[*u], dfn[*v]);
            }
        }
    }

    fn find_bridges(g: &GraphUnweightedAL) -> Vec<Vec<usize>> {
        let parent = &mut vec![None; g.len()];
        let dfn = &mut vec![g.len() + 1; g.len()];
        let low = &mut vec![g.len() + 1; g.len()];
        let vis = &mut vec![false; g.len()];
        let mut result = vec![];
        Self::find_bridges_util(&0, &g, parent, vis, dfn, low, &mut 0, &mut result);
        result
    }

    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let conn = connections
            .into_iter()
            .map(|e| vec![e[0] as usize, e[1] as usize])
            .collect();

        Self::find_bridges(&(Self::make_graph(n as usize, &conn)))
            .into_iter()
            .map(|e| vec![e[0] as i32, e[1] as i32])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_critical_connections_1() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        let output = vec![vec![1, 3]];
        assert_eq!(Solution::critical_connections(n, connections), output);
    }

    #[test]
    fn test_critical_connections_2() {
        let n = 2;
        let connections = vec![vec![0, 1]];
        let output = vec![vec![0, 1]];
        assert_eq!(Solution::critical_connections(n, connections), output);
    }
}
