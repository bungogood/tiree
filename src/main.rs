use clap::Parser;
use crypto_hash::Algorithm;
use solver::{Backtracking, Solver};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};
use sudoku::Sudoku;

mod solver;
mod sudoku;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional name to operate on
    sudoku: Option<String>,

    /// Input file
    #[arg(short = 'f')]
    infile: Option<PathBuf>,

    /// Output file
    #[arg(short = 'o')]
    outfile: Option<PathBuf>,

    /// Returns solution to soduku
    #[arg(short, long, conflicts_with = "unsolve", default_value = "true")]
    solve: bool,

    /// Returns minium clue for soduku
    #[arg(short, long, conflicts_with = "solve")]
    unsolve: bool,

    /// Returns solution to soduku
    #[arg(short, long)]
    verbose: bool,
}

fn run_file(
    filepath: &str,
    solver: &dyn Solver,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines().peekable();
    let num_sudokus: u32 = lines_iter.next().unwrap()?.trim().parse()?;

    let mut out = format!("{}\n", num_sudokus);

    println!("File: {filepath} Contains: {num_sudokus} sudokus ");

    if verbose {
        if let Some(Ok(first)) = lines_iter.peek() {
            let mut puzzle = Sudoku::from_str(&first).expect("Invalid Sudoku");
            println!("First Puzzle:");
            println!("{}", puzzle.pretty());
            println!("{}", puzzle.human());
            match solver.solve(&mut puzzle) {
                Some(sln) => {
                    println!("Solution:");
                    println!("{}", sln.pretty());
                    println!("{}", sln.human());
                }
                None => println!("Failed"),
            }
        }
    }

    let start = std::time::Instant::now();
    for line in lines_iter {
        let mut puzzle = Sudoku::from_str(&line?).expect("Invalid Sudoku");
        match solver.solve(&mut puzzle) {
            Some(solution) => out.push_str(format!("{},{}\n", puzzle, solution).as_str()),
            None => println!("Failed"),
        }
    }
    let time_taken = start.elapsed();
    let sudokus_rate = num_sudokus as f64 / time_taken.as_secs_f64();
    let avg_time = time_taken / num_sudokus;
    println!("Time Taken: {time_taken:.2?}, Speed: {sudokus_rate:.2}/s, Avg: {avg_time:.2?}");

    let out_bytes = out.as_bytes();
    let sha256sum = crypto_hash::hex_digest(Algorithm::SHA256, out_bytes);
    let md5sum = crypto_hash::hex_digest(Algorithm::MD5, out_bytes);
    println!("SHA-256 Hash: {}", sha256sum);
    println!("MD5 Hash:     {}", md5sum);
    Ok(out)
}

fn run(puzzle: Sudoku, solver: &dyn Solver) {
    let mut s = puzzle.clone();
    let start = std::time::Instant::now();
    let solved = solver.solve(&mut s);
    let time_taken = start.elapsed();

    println!("Puzzle:");
    println!("{}", puzzle.pretty());
    println!("{}", puzzle.human());
    match solved {
        Some(sln) => {
            println!("Solution:");
            println!("{}", sln.pretty());
            println!("{}", sln.human());
        }
        None => println!("Failed"),
    }

    println!("Time Taken: {:.2?}", time_taken);
}

fn main() {
    let args = Args::parse();

    let bt = Backtracking::new();

    if let Some(problem) = args.sudoku.as_deref() {
        let puzzle = Sudoku::from_str(problem).expect("Invalid Sudoku");
        run(puzzle, &bt);
    } else if let Some(infile) = args.infile.as_deref() {
        if let Ok(out) = run_file(infile.to_str().unwrap(), &bt, args.verbose) {
            if let Some(outfile) = args.outfile.as_deref() {
                std::fs::write(outfile, out).expect("Unable to write file");
            }
        } else {
            println!("Unable to read: {}", infile.display());
        }
    } else {
        println!("No input provided");
    }
}
