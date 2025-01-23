pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let mut x_count = 0;
    let mut o_count = 0;

    for row in &board {
        for ch in row.chars() {
            if ch == 'X' {
                x_count += 1;
            } else if ch == 'O' {
                o_count += 1;
            }
        }
    }

    if o_count != x_count && o_count != x_count - 1 {
        return false;
    }

    if is_winner(&board, 'X') && o_count != x_count - 1 {
        return false;
    }

    if is_winner(&board, 'O') && o_count != x_count {
        return false;
    }

    true
}

fn is_winner(board: &Vec<String>, player: char) -> bool {
    for i in 0..3 {
        if board[i].chars().all(|ch| ch == player) {
            return true;
        }
        if (0..3).all(|j| board[j].chars().nth(i).unwrap() == player) {
            return true;
        }
    }

    if (0..3).all(|i| board[i].chars().nth(i).unwrap() == player) {
        return true;
    }

    if (0..3).all(|i| board[i].chars().nth(2 - i).unwrap() == player) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tic_tac_toe() {
        assert_eq!(
            valid_tic_tac_toe(vec![
                "O  ".to_string(),
                "   ".to_string(),
                "   ".to_string()
            ]),
            false
        );
        assert_eq!(
            valid_tic_tac_toe(vec![
                "XOX".to_string(),
                " X ".to_string(),
                "   ".to_string()
            ]),
            false
        );
        assert_eq!(
            valid_tic_tac_toe(vec![
                "XXX".to_string(),
                "   ".to_string(),
                "OOO".to_string()
            ]),
            false
        );
        assert_eq!(
            valid_tic_tac_toe(vec![
                "XOX".to_string(),
                "O O".to_string(),
                "XOX".to_string()
            ]),
            true
        );
    }
}
