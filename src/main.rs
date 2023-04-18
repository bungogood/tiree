use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

mod solver;
mod sudoku;

use solver::{Backtracking, Solver};
use sudoku::Sudoku;

fn run_file(filepath: &str, solver: &dyn Solver) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    lines_iter.next(); // Skip the first line

    let start = std::time::Instant::now();

    for line in lines_iter {
        let mut puzzle = Sudoku::from_str(&line?).expect("Invalid Sudoku");
        match solver.solve(&mut puzzle) {
            Some(_sudoku) => {}
            None => println!("Failed"),
        }
    }

    let time_taken = start.elapsed();
    println!("Time Taken: {:.2?}", time_taken);

    Ok(())
}

fn run(puzzle: Sudoku, solver: &dyn Solver) {
    let mut s = puzzle.clone();
    let start = std::time::Instant::now();
    let solved = solver.solve(&mut s);
    let time_taken = start.elapsed();

    println!("{puzzle}");
    match solved {
        Some(sln) => {
            println!("Solved");
            println!("{sln}");
        }
        None => println!("Failed"),
    }

    println!("Time Taken: {:.2?}", time_taken);
}

fn main() {
    let problem =
        "....754..........8.8.19....3....1.6........34....6817.2.4...6.39......2.53.2.....";
    let puzzle = Sudoku::from_str(problem).expect("Invalid Sudoku");
    let bt = Backtracking::new();
    run(puzzle, &bt);
}
