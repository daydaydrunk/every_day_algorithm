use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = alice_sizes.iter().sum();
        let sum_b: i32 = bob_sizes.iter().sum();
        let delta = (sum_a - sum_b) / 2;

        let set_a: HashSet<i32> = alice_sizes.into_iter().collect();

        for b in bob_sizes {
            let a = b + delta;
            if set_a.contains(&a) {
                return vec![a, b];
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fair_candy_swap() {
        // Test case 1: alice_sizes = [1,1], bob_sizes = [2,2]
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
            vec![1, 2]
        );

        // Test case 2: alice_sizes = [1,2], bob_sizes = [2,3]
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2], vec![2, 3]),
            vec![1, 2]
        );

        // Test case 3: alice_sizes = [2], bob_sizes = [1,3]
        assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);

        // Test case 4: alice_sizes = [1,2,5], bob_sizes = [2,4]
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]),
            vec![5, 4]
        );
    }
}
