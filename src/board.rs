// Bitboard representation:
//  .  .  .  .  .  .  .  TOP
//  5 12 19 26 33 40 47
//  4 11 18 25 32 39 46
//  3 10 17 24 31 38 45
//  2  9 16 23 30 37 44
//  1  8 15 22 29 36 43
//  0  7 14 21 28 35 42  BOTTOM
type Bitboard = u64;

const BOTTOM: Bitboard = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;
const TOP: Bitboard = BOTTOM << Board::HEIGHT;

/// Board implementation stolen from: https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
#[derive(Clone)]
pub struct Board {
    boards: [Bitboard; 2],
    heights: [u64; 7],
    moves: [usize; Board::WIDTH * Board::HEIGHT],
    move_count: usize,
}

pub enum Token {
    Red,
    Yellow,
    Empty,
}

impl Board {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub fn new() -> Self {
        Self {
            boards: [0, 0],
            heights: [0, 7, 14, 21, 28, 35, 42],
            moves: [0; Self::WIDTH * Self::HEIGHT],
            move_count: 0,
        }
    }

    /// Puts a token into the specified column, you have to check
    /// that you actually can place a token in the column
    pub fn make_move(&mut self, col: usize) {
        self.boards[self.move_count & 1] |= 1 << self.heights[col];
        self.moves[self.move_count] = col;
        self.heights[col] += 1;
        self.move_count += 1;
    }

    /// Undos the last move, you have to check that there
    /// has been at least one move made to use this function
    pub fn undo_move(&mut self) {
        self.move_count -= 1;
        let col = self.moves[self.move_count];
        self.heights[col] -= 1;
        self.boards[self.move_count & 1] ^= 1 << self.heights[col];
    }

    /// Checks if you can put a token in the specified column
    pub fn can_play(&self, col: usize) -> bool {
        (TOP & (1 << self.heights[col])) == 0
    }

    /// Checks if the last move that has been made won the game
    pub fn has_won(&self) -> bool {
        Self::is_win(self.boards[(self.move_count + 1) & 1])
    }

    /// Checks if there is a win on the specified bitboard
    pub fn is_win(bb: Bitboard) -> bool {
        const H: usize = Board::HEIGHT;
        const H1: usize = H + 1;
        const H2: usize = H1 + 1;

        let vert = (bb >> 1) & bb; // vertical |
        let hori = (bb >> H1) & bb; // horizontal -
        let diag1 = (bb >> H) & bb; // diagonal1 \
        let diag2 = (bb >> H2) & bb; // diagonal1 /

        (vert & (vert >> 2))
            | (hori & (hori >> (2 * H1)))
            | (diag1 & (diag1 >> (2 * H)))
            | (diag2 & (diag2 >> (2 * H2)))
            != 0
    }

    /// Returns the number of moves that have been made
    pub fn move_count(&self) -> usize {
        self.move_count
    }

    /// Returns the moves that were made on the board
    pub fn moves(&self) -> &[usize] {
        &self.moves[..self.move_count]
    }

    pub fn key(&self) -> u64 {
        ((self.boards[0] | self.boards[1]) + BOTTOM) | self.boards[self.move_count & 1]
    }

    /// Returns the kind of the token at the specified position
    pub fn get(&self, row: usize, col: usize) -> Token {
        let pos = 1 << (5 - row + col * 7);
        if (self.boards[0] & pos) != 0 {
            Token::Red
        } else if (self.boards[1] & pos) != 0 {
            Token::Yellow
        } else {
            Token::Empty
        }
    }

    pub fn move_score(&self, col: usize) -> u32 {
        winning_mask(self.boards[self.move_count & 1] | 1 << self.heights[col]).count_ones()
    }

    pub fn boards(&self) -> &[Bitboard; 2] {
        &self.boards
    }
}

/// Returns the possible token positions in a bitmask
pub fn possible_mask(b: &Board) -> Bitboard {
    ((b.boards[0] | b.boards[1]) + BOTTOM) & (!TOP)
}

/// Returns a bitmask of the specified column
pub fn col_mask(col: usize) -> Bitboard {
    const COLUMN: Bitboard = (1 << (Board::HEIGHT + 1)) - 1;
    COLUMN << (col * (Board::HEIGHT + 1))
}

/// Returns a bitmask of the missing tokens
pub fn winning_mask(bb: Bitboard) -> Bitboard {
    const H: usize = Board::HEIGHT;
    const H1: usize = H + 1;
    const H2: usize = H1 + 1;

    // vertical |
    let mut res = (bb << 3) & (bb << 2) & (bb << 1);

    // horizontal -
    let mut tmp = (bb << H1) & (bb << (H1 * 2));
    res |= tmp & (bb << (H1 * 3));
    res |= (bb >> H1) & tmp;
    tmp = (bb >> (H1 * 2)) & (bb >> H1);
    res |= tmp & (bb << H1);
    res |= (bb >> (H1 * 3)) & tmp;

    // diagonal /
    tmp = (bb << H2) & (bb << (H2 * 2));
    res |= (tmp) & (bb << (H2 * 3));
    res |= (bb >> H2) & tmp;
    tmp = (bb >> (H2 * 2)) & (bb >> H2);
    res |= tmp & (bb << H2);
    res |= (bb >> (H2 * 3)) & tmp;

    // diagonal \
    tmp = (bb << H) & (bb << (H * 2));
    res |= tmp & (bb << (H * 3));
    res |= (bb >> H) & tmp;
    tmp = (bb >> (H * 2)) & (bb >> H);
    res |= tmp & (bb << H);
    res |= (bb >> (H * 3)) & tmp;

    return res & (!TOP);
}

/// Returns a mask of the non losing moves for the current player
pub fn non_losing_moves(b: &Board) -> Bitboard {
    let mut possible = possible_mask(b);
    let opponent_winning = winning_mask(b.boards[(b.move_count ^ 1) & 1]);
    let forcing = possible & opponent_winning;
    if forcing != 0 {
        // check if there is more than one forcing move
        if forcing & (forcing - 1) != 0 {
            return 0; // There is no good move
        } else {
            possible = forcing;
        }
    }
    possible & (!(opponent_winning >> 1))
}

impl From<&str> for Board {
    fn from(moves: &str) -> Self {
        let mut board = Board::new();
        for c in moves.chars() {
            let col = c.to_digit(10).unwrap() as usize;

            if board.has_won() {
                break;
            } else if Board::WIDTH <= col || !board.can_play(col) {
                panic!("Illegal move at {}", board.move_count());
            }

            board.make_move(col);
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_win_test() {
        // vertical |
        let board = Board::from("0101010");
        assert!(Board::is_win(board.boards[0]));
        assert!(!Board::is_win(board.boards[1]));

        // horizontal -
        let board = Board::from("3323431");
        assert!(Board::is_win(board.boards[0]));
        assert!(!Board::is_win(board.boards[1]));

        // diagonal /
        let board = Board::from("01123223433");
        assert!(Board::is_win(board.boards[0]));
        assert!(!Board::is_win(board.boards[1]));

        // diagonal \
        let board = Board::from("35345445633");
        assert!(Board::is_win(board.boards[0]));
        assert!(!Board::is_win(board.boards[1]));
    }
}
