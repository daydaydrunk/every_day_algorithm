struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];
        let mut color = vec![-1; n + 1];

        // Build the graph
        for edge in dislikes {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        // Helper function for DFS
        fn dfs(node: usize, c: i32, graph: &Vec<Vec<usize>>, color: &mut Vec<i32>) -> bool {
            color[node] = c;
            for &neighbor in &graph[node] {
                if color[neighbor] == -1 {
                    if !dfs(neighbor, 1 - c, graph, color) {
                        return false;
                    }
                } else if color[neighbor] == c {
                    return false;
                }
            }
            true
        }

        // Try to color each component
        for i in 1..=n {
            if color[i] == -1 {
                if !dfs(i, 0, &graph, &mut color) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_bipartition() {
        // Test case 1: n = 4, dislikes = [[1,2],[1,3],[2,4]]
        assert_eq!(
            Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]),
            true
        );

        // Test case 2: n = 3, dislikes = [[1,2],[1,3],[2,3]]
        assert_eq!(
            Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            false
        );

        // Test case 3: n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
        assert_eq!(
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
            ),
            false
        );
    }
}
