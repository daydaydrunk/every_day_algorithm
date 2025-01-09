struct Solution {}

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // diff array to track score changes at each K
        let mut diff = vec![0; n];

        // For each index i
        for i in 0..n {
            // Current value at index i
            let value = nums[i] as usize;

            if value <= i {
                // Case 1: when value <= i
                diff[0] += 1; // Starts contributing at K=0
                if i + 1 < n {
                    diff[i + 1 - value] -= 1; // Stops at K = i+1-value
                }
                if i + 1 < n {
                    diff[i + 1] += 1; // Starts again at K = i+1
                }
            } else {
                // Case 2: when value > i
                if i + 1 < n {
                    diff[i + 1] += 1; // Starts contributing at K = i+1
                }
                if i + 1 - value + n < n {
                    diff[i + 1 - value + n] -= 1; // Stops at K = i+1-value+n
                }
            }
        }

        // Find K with maximum accumulated score
        let mut curr_score = 0;
        let mut max_score = 0;
        let mut best_k = 0;

        for k in 0..n {
            curr_score += diff[k];
            if curr_score > max_score {
                max_score = curr_score;
                best_k = k;
            }
        }

        best_k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_rotation() {
        assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
        assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
    }
}
