use std::str::FromStr;

mod sudoku;

use sudoku::{
    Sudoku,
    Neighbours
};

fn run(puzzle: Sudoku, neighbours: Neighbours) {
    let mut s = puzzle.clone();
    let start = std::time::Instant::now();
    let solved = s.solve(neighbours);
    let time_taken = start.elapsed();

    if solved {
        println!("Solved:");
        println!("{}", s);
        println!("Time Taken: {:.2?}!", time_taken);
    }
}

fn main() {
    let neighbours = sudoku::gen_neighbours();
    let problem = "....754..........8.8.19....3....1.6........34....6817.2.4...6.39......2.53.2.....";
    let puzzle = Sudoku::from_str(problem).expect("Invalid Sudoku");
    run(puzzle, neighbours);
}
