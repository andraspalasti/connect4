use crate::Board;

pub struct MoveSorter {
    // Stores the moves in ascending order
    moves: [(usize, u32); Board::WIDTH],
    // Stores the amount of moves that are present
    len: usize,
}

impl MoveSorter {
    pub fn new() -> Self {
        Self {
            moves: [(0, 0); Board::WIDTH],
            len: 0,
        }
    }

    pub fn add(&mut self, col: usize, score: u32) {
        let mut pos = self.len;
        while 0 < pos && score < self.moves[pos - 1].1 {
            self.moves[pos] = self.moves[pos - 1];
            pos -= 1;
        }
        self.moves[pos] = (col, score);
        self.len += 1;
    }

    pub fn next(&mut self) -> Option<usize> {
        if 0 < self.len {
            self.len -= 1;
            Some(self.moves[self.len].0)
        } else {
            None
        }
    }
}
