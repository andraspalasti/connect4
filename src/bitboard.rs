/// Board implementation stolen from: https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
pub struct Bitboard {
    boards: [u64; 2],
    height: [u64; 7],
    moves: Vec<usize>,
}

impl Bitboard {
    pub fn new() -> Self {
        Self {
            boards: [0, 0],
            height: [0, 7, 14, 21, 28, 35, 42],
            moves: Vec::new(),
        }
    }

    pub fn make_move(&mut self, col: usize) {
        let move_ = 1 << self.height[col];
        self.boards[self.moves.len() & 1] ^= move_;
        self.moves.push(col);
        self.height[col] += 1;
    }

    pub fn undo_move(&mut self) {
        let col = self.moves.pop().expect("No move to undo");
        self.height[col] -= 1;
        let last_move = 1 << self.height[col];
        self.boards[self.moves.len() & 1] ^= last_move;
    }

    /// Checks if the last move that has been made has won the game
    pub fn has_won(&self) -> bool {
        Self::is_win(self.boards[(self.moves.len() - 1) & 1])
    }

    /// Lists all possible moves
    pub fn list_moves(&self) -> Vec<usize> {
        let mut moves = Vec::with_capacity(7);
        const TOP: usize = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;
        for col in 0..7 {
            if (TOP & (1 << self.height[col])) == 0 {
                moves.push(col)
            }
        }
        moves
    }

    /// Checks if there is a winning position on the bitboard
    pub fn is_win(bitboard: u64) -> bool {
        for dir in [1, 7, 6, 8] {
            let bb = bitboard & (bitboard >> dir);
            if bb & (bb >> (2 * dir)) != 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_test() {
        let mut bitboard = Bitboard::new();

        bitboard.make_move(0);
        assert!(bitboard.boards[0] == 0b1);

        bitboard.make_move(0);
        assert!(bitboard.boards[0] == 0b1);
        assert!(bitboard.boards[1] == 0b10);
    }

    #[test]
    fn undo_test() {
        let mut bitboard = Bitboard::new();
        bitboard.make_move(0);
        bitboard.make_move(0);
        bitboard.undo_move();
        bitboard.undo_move();
        assert!(bitboard.boards[0] == 0b0);
        assert!(bitboard.boards[1] == 0b0);
    }

    #[test]
    fn check_win_test() {
        let mut bitboard = Bitboard::new();
        for col in 0..3 {
            bitboard.make_move(col);
            bitboard.make_move(0);
        }

        bitboard.make_move(3);
        assert!(bitboard.has_won());
        assert!(Bitboard::is_win(bitboard.boards[0]));
        assert!(!Bitboard::is_win(bitboard.boards[1]));

        bitboard.make_move(0);
        assert!(bitboard.has_won());
        assert!(Bitboard::is_win(bitboard.boards[0]));
        assert!(Bitboard::is_win(bitboard.boards[1]));
    }

    #[test]
    fn move_list_test() {
        let mut bitboard = Bitboard::new();
        assert!(bitboard.list_moves().into_iter().eq(0..7));
        bitboard.height.iter_mut().for_each(|h| *h += 6);
        assert!(bitboard.list_moves().is_empty());
    }
}
