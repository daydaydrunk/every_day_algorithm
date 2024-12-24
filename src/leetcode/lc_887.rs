struct Solution {}

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; k + 1];

        let mut m = 0;
        while dp[k][m] < n {
            m += 1;
            for i in 1..=k {
                dp[i][m] = dp[i][m - 1] + dp[i - 1][m - 1] + 1;
            }
        }

        m as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_egg_drop() {
        // Test case 1: k = 1, n = 2
        assert_eq!(Solution::super_egg_drop(1, 2), 2);

        // Test case 2: k = 2, n = 6
        assert_eq!(Solution::super_egg_drop(2, 6), 3);

        // Test case 3: k = 3, n = 14
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}
