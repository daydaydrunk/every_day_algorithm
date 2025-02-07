// ...existing code...
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    use std::cmp::Ordering;

    // Precompute the positions of each character in the string s
    let mut positions = vec![Vec::new(); 26];
    for (idx, ch) in s.bytes().enumerate() {
        positions[(ch - b'a') as usize].push(idx as i32);
    }

    let mut count = 0;
    // Check each word to see if it is a subsequence of s
    for w in words {
        let mut last_pos = -1;
        let mut is_subsequence = true;

        for ch in w.bytes() {
            let ch_index = (ch - b'a') as usize;
            let list = &positions[ch_index];
            if list.is_empty() {
                is_subsequence = false;
                break;
            }

            // Binary search in list to find the smallest element > last_pos
            let (mut left, mut right) = (0, list.len() - 1);
            let mut nxt = -1;
            while left <= right {
                let mid = (left + right) / 2;
                match list[mid].cmp(&(last_pos + 1)) {
                    Ordering::Greater | Ordering::Equal => {
                        nxt = list[mid];
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                    Ordering::Less => {
                        left = mid + 1;
                    }
                }
            }
            if nxt == -1 {
                is_subsequence = false;
                break;
            }
            last_pos = nxt;
        }

        if is_subsequence {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_matching_subseq() {
        assert_eq!(
            num_matching_subseq(
                "abcde".to_string(),
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "acd".to_string(),
                    "ace".to_string()
                ]
            ),
            3
        );
        assert_eq!(
            num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec![
                    "ahjpjau".to_string(),
                    "ja".to_string(),
                    "ahbwzgqnuk".to_string(),
                    "tnmlanowax".to_string()
                ]
            ),
            2
        );
    }
}
// ...existing code...
