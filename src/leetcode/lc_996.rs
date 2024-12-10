use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        // Count frequency of each number
        let mut count = HashMap::new();
        for &num in nums.iter() {
            *count.entry(num).or_insert(0) += 1;
        }

        // Create adjacency map for valid pairs
        let mut graph = HashMap::new();
        for &i in count.keys() {
            for &j in count.keys() {
                let sum = i + j;
                let sqrt = (sum as f64).sqrt() as i64;
                if sqrt * sqrt == sum as i64 {
                    graph.entry(i).or_insert(Vec::new()).push(j);
                }
            }
        }

        // DFS with backtracking
        fn dfs(
            curr: i32,
            remaining: i32,
            count: &mut HashMap<i32, i32>,
            graph: &HashMap<i32, Vec<i32>>,
        ) -> i32 {
            if remaining == 0 {
                return 1;
            }

            let mut result = 0;
            if let Some(neighbors) = graph.get(&curr) {
                for &next in neighbors {
                    if count[&next] > 0 {
                        count.entry(next).and_modify(|e| *e -= 1);
                        result += dfs(next, remaining - 1, count, graph);
                        count.entry(next).and_modify(|e| *e += 1);
                    }
                }
            }
            result
        }

        // Start DFS from each number
        let mut result = 0;
        let keys: Vec<i32> = count.keys().cloned().collect();
        for &num in keys.iter() {
            count.entry(num).and_modify(|e| *e -= 1);
            result += dfs(num, nums.len() as i32 - 1, &mut count, &graph);
            count.entry(num).and_modify(|e| *e += 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squareful_perms() {
        // Test case 1: [1,17,8]
        assert_eq!(Solution::num_squareful_perms(vec![1, 17, 8]), 2);

        // Test case 2: [2,2,2]
        assert_eq!(Solution::num_squareful_perms(vec![2, 2, 2]), 1);

        // Test case 3: [1,1]
        assert_eq!(Solution::num_squareful_perms(vec![1, 1]), 1);
    }
}
