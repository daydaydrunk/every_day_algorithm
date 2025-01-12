struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let target = (graph.len() - 1) as i32;
        let mut result = Vec::new();
        let mut path = vec![0]; // Start from node 0

        Self::dfs(0, target, &mut path, &graph, &mut result);
        result
    }

    fn dfs(
        current: i32,
        target: i32,
        path: &mut Vec<i32>,
        graph: &Vec<Vec<i32>>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // Found a valid path
        if current == target {
            result.push(path.clone());
            return;
        }

        // Explore all neighbors
        for &next in &graph[current as usize] {
            path.push(next);
            Self::dfs(next, target, path, graph, result);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_paths() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let expected = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(Solution::all_paths_source_target(graph), expected);

        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let expected = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(Solution::all_paths_source_target(graph), expected);
    }
}
