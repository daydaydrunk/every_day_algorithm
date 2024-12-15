struct Solution {}

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn at_most_k(nums: &Vec<i32>, k: i32) -> i32 {
            let mut count = std::collections::HashMap::new();
            let mut left = 0;
            let mut result = 0;
            let mut k = k;

            for right in 0..nums.len() {
                if *count.entry(nums[right]).or_insert(0) == 0 {
                    k -= 1;
                }
                *count.entry(nums[right]).or_insert(0) += 1;

                while k < 0 {
                    *count.entry(nums[left]).or_insert(0) -= 1;
                    if count[&nums[left]] == 0 {
                        k += 1;
                    }
                    left += 1;
                }

                result += (right - left + 1) as i32;
            }

            result
        }

        at_most_k(&nums, k) - at_most_k(&nums, k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarrays_with_k_distinct() {
        // Test case 1: nums = [1,2,1,2,3], k = 2
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );

        // Test case 2: nums = [1,2,1,3,4], k = 3
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );

        // Test case 3: nums = [1,2,1,2,1,2,1,2], k = 2
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 1, 2, 1, 2], 2),
            20
        );
    }
}
