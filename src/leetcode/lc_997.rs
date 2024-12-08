struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // Create vectors to track in-degree and out-degree
        let mut trusted_by = vec![0; (n + 1) as usize];
        let mut trusts = vec![0; (n + 1) as usize];

        // Count trust relationships
        for relation in trust {
            let (a, b) = (relation[0] as usize, relation[1] as usize);
            trusts[a] += 1; // a trusts someone
            trusted_by[b] += 1; // b is trusted by someone
        }

        // Find the judge
        for i in 1..=n as usize {
            // Judge is trusted by everyone except themselves (n-1)
            // and doesn't trust anyone (0)
            if trusted_by[i] == n as i32 - 1 && trusts[i] == 0 {
                return i as i32;
            }
        }

        -1 // No judge found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_judge() {
        // Test case 1: n = 2, trust = [[1,2]]
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);

        // Test case 2: n = 3, trust = [[1,3],[2,3]]
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);

        // Test case 3: n = 3, trust = [[1,3],[2,3],[3,1]]
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );

        // Test case 4: n = 1, trust = []
        assert_eq!(Solution::find_judge(1, vec![]), 1);
    }
}
