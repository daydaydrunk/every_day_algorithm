struct Solution {}

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut target = target;
        let mut operations = 0;

        while target > start_value {
            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
            operations += 1;
        }

        operations + (start_value - target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broken_calc() {
        // Test case 1: start_value = 2, target = 3
        assert_eq!(Solution::broken_calc(2, 3), 2);

        // Test case 2: start_value = 5, target = 8
        assert_eq!(Solution::broken_calc(5, 8), 2);

        // Test case 3: start_value = 3, target = 10
        assert_eq!(Solution::broken_calc(3, 10), 3);

        // Test case 4: start_value = 1024, target = 1
        assert_eq!(Solution::broken_calc(1024, 1), 1023);
    }
}
