struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut tower = vec![vec![0.0; 101]; 101];
        tower[0][0] = poured as f64;

        for r in 0..=query_row as usize {
            for c in 0..=r {
                if tower[r][c] > 1.0 {
                    let overflow = (tower[r][c] - 1.0) / 2.0;
                    tower[r][c] = 1.0;
                    tower[r + 1][c] += overflow;
                    tower[r + 1][c + 1] += overflow;
                }
            }
        }

        tower[query_row as usize][query_glass as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_champagne_tower() {
        // Test case 1: poured = 1, query_row = 1, query_glass = 1
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);

        // Test case 2: poured = 2, query_row = 1, query_glass = 1
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);

        // Test case 3: poured = 100000009, query_row = 33, query_glass = 17
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
    }
}
