struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        // Edge cases - lengths must match
        if s.len() != goal.len() {
            return false;
        }

        // Concatenate s with itself and check if goal is substring
        let doubled = s.clone() + &s;
        doubled.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "abced".to_string()),
            false
        );
    }
}
