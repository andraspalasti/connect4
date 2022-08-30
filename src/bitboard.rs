/// Board implementation stolen from: https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
#[derive(Clone)]
pub struct Bitboard {
    boards: [u64; 2],
    height: [u64; 7],
    moves: [usize; Bitboard::WIDTH * Bitboard::HEIGHT],
    move_count: usize,
}

pub enum Token {
    Red,
    Yellow,
    Empty,
}

impl Bitboard {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub fn new() -> Self {
        Self {
            boards: [0, 0],
            height: [0, 7, 14, 21, 28, 35, 42],
            moves: [0; Self::WIDTH * Self::HEIGHT],
            move_count: 0,
        }
    }

    pub fn make_move(&mut self, col: usize) {
        self.boards[self.move_count % 2] ^= 1 << self.height[col];
        self.moves[self.move_count] = col;
        self.height[col] += 1;
        self.move_count += 1;
    }

    pub fn undo_move(&mut self) {
        self.move_count -= 1;
        let col = self.moves[self.move_count];
        self.height[col] -= 1;
        self.boards[self.move_count % 2] ^= 1 << self.height[col];
    }

    /// Checks if the last move that has been made has won the game
    pub fn has_won(&self) -> bool {
        let bitboard = self.boards[(self.move_count + 1) % 2];
        for dir in [1, 7, 6, 8] {
            let bb = bitboard & (bitboard >> dir);
            if bb & (bb >> (2 * dir)) != 0 {
                return true;
            }
        }
        false
    }

    /// Lists all possible moves
    pub fn list_moves(&self) -> Vec<usize> {
        let mut moves = Vec::with_capacity(Self::WIDTH);
        const TOP: u64 = 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000;
        for col in 0..Self::WIDTH {
            if (TOP & (1 << self.height[col])) == 0 {
                moves.push(col)
            }
        }
        moves
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    /// Returns the moves that were made on the board
    pub fn moves(&self) -> &[usize] {
        &self.moves[..self.move_count]
    }

    pub fn boards(&self) -> &[u64; 2] {
        &self.boards
    }

    pub fn get(&self, row: usize, col: usize) -> Token {
        let pos = 1 << (5 - row + col * 7);
        if self.boards[0] & pos != 0 {
            Token::Red
        } else if self.boards[1] & pos != 0 {
            Token::Yellow
        } else {
            Token::Empty
        }
    }
}

impl From<&str> for Bitboard {
    fn from(moves: &str) -> Self {
        let mut bitboard = Bitboard::new();
        for c in moves.chars() {
            let move_ = c.to_digit(10).unwrap() as usize;
            bitboard.make_move(move_ - 1);
        }
        bitboard
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
        assert!(bitboard.has_won() == false);
        for col in 0..3 {
            bitboard.make_move(col);
            bitboard.make_move(0);
        }

        bitboard.make_move(3);
        assert!(bitboard.has_won());
    }

    #[test]
    fn move_list_test() {
        let mut bitboard = Bitboard::new();
        assert!(bitboard.list_moves().into_iter().eq(0..7));
        bitboard.height.iter_mut().for_each(|h| *h += 6);
        assert!(bitboard.list_moves().is_empty());
    }
}
