pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        // Find rook position
        let (mut rx, mut ry) = (0, 0);
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    rx = i;
                    ry = j;
                    break;
                }
            }
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut captures = 0;

        for (dx, dy) in directions {
            let mut x = rx as i32;
            let mut y = ry as i32;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                match board[x as usize][y as usize] {
                    'p' => {
                        captures += 1;
                        break;
                    }
                    'B' => break,
                    _ => {}
                }
                x += dx;
                y += dy;
            }
        }

        captures
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rook_captures() {
        // Test case 1: Normal case with 2 captures
        let board1 = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board1), 3);

        // Test case 2: Blocked by bishops
        let board2 = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'p', 'B', 'R', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board2), 0);

        // Test case 3: Rook at corner
        let board3 = vec![
            vec!['R', '.', '.', '.', '.', '.', '.', '.'],
            vec!['p', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board3), 1);
    }
}
