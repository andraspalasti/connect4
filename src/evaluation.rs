use super::bitboard::Bitboard;

const ILLEGAL_MOVE: i32 = 100;
const SIZE: i32 = (Bitboard::WIDTH * Bitboard::HEIGHT) as i32;

pub fn minimax(board: &mut Bitboard) -> i32 {
    // const SIZE: i32 = (Bitboard::WIDTH * Bitboard::HEIGHT) as i32;
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

/// Minimax search with alpha beta pruning
/// alpha is the best score red is assured of
/// beta is the best score yellow is assured of
pub fn alpha_beta(board: &mut Bitboard, mut alpha: i32, mut beta: i32) -> i32 {
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
            value = value.max(alpha_beta(board, alpha, beta));
            board.undo_move();

            if beta < value {
                break;
            }
            alpha = value.max(alpha)
        }
        value
    } else {
        let mut value = SIZE;
        for move_ in moves {
            board.make_move(move_);
            value = value.min(alpha_beta(board, alpha, beta));
            board.undo_move();

            if value < alpha {
                break;
            }
            beta = value.min(beta)
        }
        value
    }
}

pub fn analyze<EF>(mut board: Bitboard, evaluate: EF) -> [i32; Bitboard::WIDTH]
where
    EF: Fn(&mut Bitboard) -> i32,
{
    let mut result = [ILLEGAL_MOVE; Bitboard::WIDTH];
    for move_ in board.list_moves() {
        board.make_move(move_);
        result[move_] = evaluate(&mut board);
        board.undo_move();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_eval_test() {
        let board = Bitboard::from("41245376333225777136115215667766214");

        let minimax_result = analyze(board.clone(), |b| minimax(b));
        let alpha_beta_result = analyze(board, |b| alpha_beta(b, -SIZE, SIZE));

        assert_eq!(minimax_result, [100, 0, 4, -7, 4, 100, 100]);
        assert_eq!(minimax_result, alpha_beta_result);
    }
}
