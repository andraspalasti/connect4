use std::time::Instant;

use connect4::{Board, Solver};

fn main() {
    let mut solver = Solver::new(8 * 1024 * 1024);

    let boards = vec!["5022001440", "21640460120446"];

    for board in boards {
        let now = Instant::now();
        solver.analyze(Board::from(board));
        println!(
            "'{}' took {}ms, explored {} nodes",
            board,
            now.elapsed().as_millis(),
            solver.explored_nodes()
        );
        solver.reset();
    }
}
