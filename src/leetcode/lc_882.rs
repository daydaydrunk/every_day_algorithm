use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), 0)); // (distance, node)
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;

        while let Some((Reverse(d), u)) = pq.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let d2 = d + w + 1;
                if d2 < dist[v] {
                    dist[v] = d2;
                    pq.push((Reverse(d2), v));
                }
            }
        }

        let mut reachable_nodes = 0;
        for d in &dist {
            if *d <= max_moves {
                reachable_nodes += 1;
            }
        }

        let mut used_edges = HashMap::new();
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            let a = max_moves - dist[u];
            let b = max_moves - dist[v];
            let used = a.max(0) + b.max(0);
            reachable_nodes += used.min(w);
            used_edges.insert((u, v), used.min(w));
        }

        reachable_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reachable_nodes() {
        // Test case 1: edges = [[0,1,10],[0,2,1],[1,2,2]], max_moves = 6, n = 3
        assert_eq!(
            Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
            13
        );

        // Test case 2: edges = [[0,1,4],[1,2,6],[0,2,8],[1,3,1]], max_moves = 10, n = 4
        assert_eq!(
            Solution::reachable_nodes(
                vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
                10,
                4
            ),
            23
        );
    }
}
