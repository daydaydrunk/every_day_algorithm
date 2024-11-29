use std::collections::HashMap;

// Define the Graph structure
struct Graph {
    adj_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    // Create a new graph
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: i32, dest: i32) {
        self.adj_list.entry(src).or_insert(Vec::new()).push(dest);
        self.adj_list.entry(dest).or_insert(Vec::new()).push(src); // Undirected graph
    }

    // DFS algorithm
    fn dfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashMap::new();
        let mut result = Vec::new();
        self.dfs_util(start, &mut visited, &mut result);
        result
    }

    // Utility function for DFS
    fn dfs_util(&self, node: i32, visited: &mut HashMap<i32, bool>, result: &mut Vec<i32>) {
        visited.insert(node, true);
        result.push(node);

        if let Some(neighbors) = self.adj_list.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    self.dfs_util(neighbor, visited, result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let dfs_result = graph.dfs(1);
        assert_eq!(dfs_result, vec![1, 2, 4, 5, 3]);
    }
}
