struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        // Helper function to count arrays with max less than or equal to bound
        fn count_subarrays(nums: &Vec<i32>, bound: i32) -> i32 {
            let mut count = 0;
            let mut curr = 0;

            for &num in nums {
                if num <= bound {
                    curr += 1;
                    count += curr;
                } else {
                    curr = 0;
                }
            }
            count
        }

        // Result = arrays with max ≤ right minus arrays with max < left
        count_subarrays(&nums, right) - count_subarrays(&nums, left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarray_bounded_max() {
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3),
            3
        );
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8),
            7
        );
    }
}
