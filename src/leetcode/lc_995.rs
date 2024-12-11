struct Solution {}

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        // Track flip operations
        let mut flips = vec![0; n];
        let mut current_flips = 0;
        let mut result = 0;

        for i in 0..n {
            // If we're k positions after a flip, remove its effect
            if i >= k && flips[i - k] == 1 {
                current_flips ^= 1;
            }

            // Check if current position needs a flip
            let current_bit = nums[i] ^ (current_flips);
            if current_bit == 0 {
                // Can't flip if there's not enough elements ahead
                if i + k > n {
                    return -1;
                }
                flips[i] = 1;
                current_flips ^= 1;
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_k_bit_flips() {
        // Test case 1: nums = [0,1,0], k = 1
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);

        // Test case 2: nums = [1,1,0], k = 2
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);

        // Test case 3: nums = [0,0,0,1,0,1,1,0], k = 3
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
