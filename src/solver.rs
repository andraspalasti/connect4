use super::board::Board;

const BOARD_SIZE: i32 = (Board::HEIGHT * Board::WIDTH) as i32;
const MAX_SCORE: i32 = BOARD_SIZE + 1;
const MIN_SCORE: i32 = -MAX_SCORE;

const MOVE_EXPLORATION_ORDER: [usize; Board::WIDTH] = [3, 2, 4, 1, 5, 0, 6];

const ILLEGAL_MOVE: i32 = 100;

fn negamax(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let move_count = board.move_count() as i32;

    if board.has_won() {
        return -(MAX_SCORE - move_count);
    }

    if move_count == BOARD_SIZE {
        return 0;
    }

    for col in MOVE_EXPLORATION_ORDER {
        if board.can_play(col) {
            board.make_move(col);
            let score = -negamax(board, -beta, -alpha);
            board.undo_move();

            if beta <= score {
                return score;
            }

            if alpha < score {
                alpha = score
            }
        }
    }
    alpha
}

pub fn analyze(mut board: Board) -> [i32; Board::WIDTH] {
    let mut result = [ILLEGAL_MOVE; Board::WIDTH];
    for col in 0..Board::WIDTH {
        if board.can_play(col) {
            board.make_move(col);
            result[col] = -negamax(&mut board, MIN_SCORE, MAX_SCORE);
            board.undo_move();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_eval_test() {
        let board = Board::from("33333344226000000666664");

        let result = analyze(board);

        assert_eq!(result, [100, -18, -18, 100, -18, -18, 100]);
    }
}
