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
    let num_sudokus: u32 = lines_iter.next().unwrap()?.trim().parse()?;

    println!("File: {filepath} Contains: {num_sudokus} sudokus ");

    let start = std::time::Instant::now();
    for line in lines_iter {
        let mut puzzle = Sudoku::from_str(&line?).expect("Invalid Sudoku");
        match solver.solve(&mut puzzle) {
            Some(_sudoku) => {}
            None => println!("Failed"),
        }
    }
    let time_taken = start.elapsed();
    let sudokus_rate = num_sudokus as f64 / time_taken.as_secs_f64();
    let avg_time = time_taken / num_sudokus;
    println!("Time Taken: {time_taken:.2?}, Speed: {sudokus_rate:.2}/s, Avg: {avg_time:.2?}");

    Ok(())
}

fn run(puzzle: Sudoku, solver: &dyn Solver) {
    let mut s = puzzle.clone();
    let start = std::time::Instant::now();
    let solved = solver.solve(&mut s);
    let time_taken = start.elapsed();

    println!("Puzzle");
    println!("{}", puzzle.pretty());
    // println!("{puzzle}");
    match solved {
        Some(sln) => {
            println!("Solved");
            println!("{}", sln.pretty());
            // println!("{sln}")
        }
        None => println!("Failed"),
    }

    println!("Time Taken: {:.2?}", time_taken);
}

fn main() {
    let problem =
        "..43..2.9..5..9..1.7..6..43..6..2.8719...74...5..83...6.....1.5..35.869..4291.3..";
    let puzzle = Sudoku::from_str(problem).expect("Invalid Sudoku");
    let bt = Backtracking::new();

    run(puzzle, &bt);

    // let filepath = "sudokus/hard.txt";
    // let filepath = "sudokus/all-17-clue.txt";
    let filepath = "sudokus/1000000.txt";

    if let Err(_) = run_file(filepath, &bt) {
        println!("Unable to read: {filepath}")
    }
}
