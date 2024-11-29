use std::collections::HashMap;
use std::collections::VecDeque;

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

    // BFS algorithm
    fn bfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Mark the starting node as visited and enqueue it
        visited.insert(start, true);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);

            // Iterate through all adjacent nodes
            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor, true);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let bfs_result = graph.bfs(1);
        assert_eq!(bfs_result, vec![1, 2, 3, 4, 5]);
    }
}
