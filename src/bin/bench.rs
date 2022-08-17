use std::io::Write;
use std::time::Instant;

use connect4::bitboard::Bitboard;
use connect4::evaluation::analyze;

const RUNS: usize = 30;

fn main() {
    let mut stdout = std::io::stdout();
    let mut board = Bitboard::from("41245376333225777136115215667");

    let mut avg = 0;

    for i in 0..RUNS {
        // Progress bar
        write!(stdout, "[{}>{}]\r", "=".repeat(i), "-".repeat(RUNS - i - 1)).unwrap();
        stdout.flush().unwrap();

        let now = Instant::now();
        analyze(&mut board);
        avg += now.elapsed().as_millis();
    }
    println!("");

    println!("Each iteration roughly took: {}ms", avg / RUNS as u128);
}
