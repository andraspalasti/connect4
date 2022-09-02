use crate::trans_table::TransTable;

use super::board::Board;

const BOARD_SIZE: i32 = (Board::HEIGHT * Board::WIDTH) as i32;

pub const MAX_SCORE: i32 = BOARD_SIZE + 1;
pub const MIN_SCORE: i32 = -MAX_SCORE;

const MOVE_EXPLORATION_ORDER: [usize; Board::WIDTH] = [3, 2, 4, 1, 5, 0, 6];

const ILLEGAL_MOVE: i32 = 100;

pub struct Solver {
    num_nodes: usize,
    trans_table: TransTable,
}

impl Solver {
    pub fn new(tt_capacity: usize) -> Self {
        Self {
            num_nodes: 0,
            trans_table: TransTable::new(tt_capacity),
        }
    }

    fn negamax(&mut self, board: &mut Board, mut alpha: i32, mut beta: i32) -> i32 {
        self.num_nodes += 1;
        let move_count = board.move_count() as i32;

        if board.has_won() {
            return -(MAX_SCORE - move_count);
        }

        if move_count == BOARD_SIZE {
            return 0;
        }

        let max = MAX_SCORE - move_count - 1;
        if max < beta {
            beta = max;
            if beta <= alpha {
                return beta;
            }
        }

        let min = -(MAX_SCORE - move_count - 1);
        if alpha < min {
            alpha = min;
            if beta <= alpha {
                return alpha;
            }
        }

        let key = board.key();
        let (value, is_upper) = self.trans_table.get(key);
        match is_upper {
            true if value < beta => {
                beta = value;
                if beta <= alpha {
                    return beta;
                }
            }
            false if alpha < value => {
                alpha = value;
                if beta <= alpha {
                    return alpha;
                }
            }
            _ => (),
        }

        for col in MOVE_EXPLORATION_ORDER {
            if board.can_play(col) {
                board.make_move(col);
                let score = -self.negamax(board, -beta, -alpha);
                board.undo_move();

                if beta <= score {
                    self.trans_table.put(key, score, false);
                    return score;
                }

                if alpha < score {
                    alpha = score
                }
            }
        }
        self.trans_table.put(key, value, true);
        alpha
    }

    pub fn analyze(&mut self, mut board: Board) -> [i32; Board::WIDTH] {
        let mut result = [ILLEGAL_MOVE; Board::WIDTH];
        for col in 0..Board::WIDTH {
            if board.can_play(col) {
                board.make_move(col);
                result[col] = -self.negamax(&mut board, MIN_SCORE, MAX_SCORE);
                board.undo_move();
            }
        }
        result
    }

    pub fn reset(&mut self) {
        self.num_nodes = 0;
        self.trans_table.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_eval_test() {
        let board = Board::from("33333344226000000666664");

        let result = Solver::new(8).analyze(board);

        assert_eq!(result, [100, -18, -18, 100, -18, -18, 100]);
    }
}
