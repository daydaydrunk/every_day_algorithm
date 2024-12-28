struct Solution {}

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut r = r0;
        let mut c = c0;
        let mut steps = 1;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_index = 0;

        result.push(vec![r, c]);

        while result.len() < (rows * cols) as usize {
            for _ in 0..2 {
                for _ in 0..steps {
                    r += directions[dir_index].0;
                    c += directions[dir_index].1;

                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        result.push(vec![r, c]);
                    }
                }
                dir_index = (dir_index + 1) % 4;
            }
            steps += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix_iii() {
        // Test case 1: rows = 1, cols = 4, r0 = 0, c0 = 0
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );

        // Test case 2: rows = 5, cols = 6, r0 = 1, c0 = 4
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
