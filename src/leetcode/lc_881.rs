struct Solution {}

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut left = 0;
        let mut right = people.len() as i32 - 1;
        let mut boats = 0;

        while left <= right {
            if people[left as usize] + people[right as usize] <= limit {
                left += 1;
            }
            right -= 1;
            boats += 1;
        }

        boats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rescue_boats() {
        // Test case 1: people = [1,2], limit = 3
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);

        // Test case 2: people = [3,2,2,1], limit = 3
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);

        // Test case 3: people = [3,5,3,4], limit = 5
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
