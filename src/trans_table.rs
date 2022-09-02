use crate::{MAX_SCORE, MIN_SCORE};

/// Transposition Table
/// An entry is stored like this:
/// |    key    |  value   |  flag   |
/// |  55 bits  |  8 bits  |  1 bits |
pub struct TransTable {
    positions: Vec<u64>,
}

const FLAG_MASK: u64 = 0b1;
const VALUE_MASK: u64 = 0b1111_1111_0;
const KEY_MASK: u64 = !(FLAG_MASK | VALUE_MASK);

impl TransTable {
    pub fn new(capacity: usize) -> Self {
        TransTable {
            positions: vec![0; capacity],
        }
    }

    pub fn get(&self, key: u64) -> (i32, bool) {
        let tt_entry = self.positions[key as usize % self.positions.len()];
        if tt_entry & KEY_MASK != key {
            return (MAX_SCORE, true);
        }
        (
            (tt_entry & VALUE_MASK) as i32 + MIN_SCORE,
            tt_entry & FLAG_MASK != 0,
        )
    }

    pub fn put(&mut self, key: u64, value: i32, is_upper: bool) {
        let idx = key as usize % self.positions.len();
        if is_upper {
            self.positions[idx] = (key << 9) | (value - MIN_SCORE) as u64 | FLAG_MASK;
        } else {
            self.positions[idx] = (key << 9) | (value - MIN_SCORE) as u64;
        }
    }

    pub fn clear(&mut self) {
        self.positions.fill(0);
    }
}
