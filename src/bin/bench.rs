use std::io::Write;
use std::time::Instant;

use connect4::{Solver, Board};

const RUNS: usize = 1;

fn main() {
    let mut solver = Solver::new(8*1024*1024);

    let mut stdout = std::io::stdout();
    let board = Board::from("333332242134");

    let mut avg = 0;

    for i in 0..RUNS {
        // Progress bar
        write!(stdout, "[{}>{}]\r", "=".repeat(i), "-".repeat(RUNS - i - 1)).unwrap();
        stdout.flush().unwrap();

        let b_clone = board.clone();
        let now = Instant::now();
        solver.analyze(b_clone);
        avg += now.elapsed().as_millis();
        solver.reset();
    }
    println!("");

    println!("Each iteration roughly took: {}ms", avg / RUNS as u128);
}
