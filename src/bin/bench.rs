use std::io::Write;
use std::time::Instant;

use connect4::bitboard::Bitboard;
use connect4::evaluation::{alpha_beta, analyze};

const RUNS: usize = 50;

fn main() {
    let mut stdout = std::io::stdout();
    let board = Bitboard::from("412453763332257771361152156");

    let mut avg = 0;

    for i in 0..RUNS {
        // Progress bar
        write!(stdout, "[{}>{}]\r", "=".repeat(i), "-".repeat(RUNS - i - 1)).unwrap();
        stdout.flush().unwrap();

        let b_clone = board.clone();
        let now = Instant::now();
        analyze(b_clone, |b| alpha_beta(b, -42, 42));
        avg += now.elapsed().as_millis();
    }
    println!("");

    println!("Each iteration roughly took: {}ms", avg / RUNS as u128);
}
