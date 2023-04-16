use std::str::FromStr;

mod sudoku;
mod backtracking;

use sudoku::{
    Sudoku,
    Solver
};

fn run(puzzle: Sudoku, solver: &dyn Solver) {
    let mut s = puzzle.clone();
    let start = std::time::Instant::now();
    let solved = solver.solve(&mut s);
    let time_taken = start.elapsed();

    if solved {
        println!("Solved:");
        println!("{}", s);
        println!("Time Taken: {:.2?}!", time_taken);
    }
}

fn main() {
    let problem = "....754..........8.8.19....3....1.6........34....6817.2.4...6.39......2.53.2.....";
    let puzzle = Sudoku::from_str(problem).expect("Invalid Sudoku");
    let bt = backtracking::Backtracking::new();
    run(puzzle, &bt);
}
