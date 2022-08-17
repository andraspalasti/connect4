use super::bitboard::Bitboard;

const ILLEGAL_MOVE: i32 = 100;

pub fn minimax(board: &mut Bitboard) -> i32 {
    const SIZE: i32 = (Bitboard::WIDTH * Bitboard::HEIGHT) as i32;
    let move_count = board.move_count() as i32;

    let red_turn = board.move_count() % 2 == 0;

    // Check if the previous move has won the game
    if board.has_won() {
        let mut value = SIZE + 1 - move_count;
        // if red_turn == true that means yellow has won with its last move
        if red_turn {
            value *= -1;
        }
        return value;
    }

    // Check for a draw
    if SIZE == move_count {
        return 0;
    }

    let moves = board.list_moves();
    if red_turn {
        let mut value = -SIZE;
        for move_ in moves {
            board.make_move(move_);
            value = value.max(minimax(board));
            board.undo_move();
        }
        value
    } else {
        let mut value = SIZE;
        for move_ in moves {
            board.make_move(move_);
            value = value.min(minimax(board));
            board.undo_move();
        }
        value
    }
}

pub fn analyze(board: &mut Bitboard) -> [i32; Bitboard::WIDTH] {
    let mut result = [ILLEGAL_MOVE; Bitboard::WIDTH];
    for move_ in board.list_moves() {
        board.make_move(move_);
        result[move_] = minimax(board);
        board.undo_move();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_eval_test() {
        let mut board = Bitboard::from("41245376333225777136115215667766214");

        let eval = minimax(&mut board);
        assert_eq!(eval.abs() - 1, (42 - 36));
        assert_eq!(analyze(&mut board), [100, 0, 4, -7, 4, 100, 100])
    }
}
