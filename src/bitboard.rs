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
            height: [0, 7, 15, 24, 30, 35, 42],
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
