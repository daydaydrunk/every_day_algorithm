struct Solution {}

impl Solution {
    pub fn orderly_queue(mut s: String, k: i32) -> String {
        if k == 1 {
            let mut smallest = s.clone();
            s.push_str(s.clone().as_str()); // Concatenate string with itself
            for i in 1..smallest.len() {
                let rotation = &s[i..i + smallest.len()];
                if rotation < &smallest {
                    smallest = rotation.to_string();
                }
            }
            smallest
        } else {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            chars.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderly_queue() {
        // Test case 1: s = "cba", k = 1
        assert_eq!(
            Solution::orderly_queue("cba".to_string(), 1),
            "acb".to_string()
        );

        // Test case 2: s = "baaca", k = 3
        assert_eq!(
            Solution::orderly_queue("baaca".to_string(), 3),
            "aaabc".to_string()
        );

        // Test case 3: s = "abc", k = 2
        assert_eq!(
            Solution::orderly_queue("abc".to_string(), 2),
            "abc".to_string()
        );
    }
}
