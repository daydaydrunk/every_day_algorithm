use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue = VecDeque::new();
        let mut fresh_count = 0;
        let mut minutes = 0;

        // Find initial rotten oranges and count fresh ones
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 2 {
                    queue.push_back((i, j));
                } else if grid[i][j] == 1 {
                    fresh_count += 1;
                }
            }
        }

        // Directions: up, right, down, left
        let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        // BFS
        while !queue.is_empty() && fresh_count > 0 {
            let level_size = queue.len();

            for _ in 0..level_size {
                if let Some((row, col)) = queue.pop_front() {
                    for (dr, dc) in &dirs {
                        let new_row = row as i32 + dr;
                        let new_col = col as i32 + dc;

                        if new_row >= 0
                            && new_row < rows as i32
                            && new_col >= 0
                            && new_col < cols as i32
                            && grid[new_row as usize][new_col as usize] == 1
                        {
                            grid[new_row as usize][new_col as usize] = 2;
                            queue.push_back((new_row as usize, new_col as usize));
                            fresh_count -= 1;
                        }
                    }
                }
            }
            minutes += 1;
        }

        if fresh_count > 0 {
            -1
        } else {
            minutes
        }
    }
}
