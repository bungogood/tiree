use std::vec;

use crate::sudoku::{Solver, Sudoku};

const ALL: u32 = 0x1ff;

type RowColSub = (usize, usize, usize);

pub struct SolverBasic {
    rows: [u32; 9],
    cols: [u32; 9],
    subs: [u32; 9],
    todo: Vec<RowColSub>,
    numtodo: usize,
    numguesses: usize,
}

impl SolverBasic {
    pub fn new() -> Self {
        SolverBasic {
            rows: [ALL; 9],
            cols: [ALL; 9],
            subs: [ALL; 9],
            todo: vec![],
            numtodo: 0,
            numguesses: 0,
        }
    }

    fn init(&mut self, input: &Sudoku, solution: &mut Sudoku) {
        self.rows.fill(ALL);
        self.cols.fill(ALL);
        self.subs.fill(ALL);
        self.numguesses = 0;

        // copy initial clues to solution; todo list won't include these cells
        self.todo.clear();

        for row in 0..9 {
            for col in 0..9 {
                solution[row * 9 + col] = input[row * 9 + col];
                let sub = (row / 3) * 3 + col / 3;
                if input[row * 9 + col] == '.' {
                    // blank cell: add to the todo list
                    self.todo.push((row, col, sub));
                } else {
                    // a given clue: clear availability bits for row, col, and sub
                    let value = 1 << (input[row * 9 + col] as u8 - '1' as u8);
                    self.rows[row] ^= value;
                    self.cols[col] ^= value;
                    self.subs[sub] ^= value;
                }
            }
        }
        self.numtodo = self.todo.len() - 1;
    }

    // Returns true if a solution is found, updates *solution to reflect assignments
    // made on solution path. Also updates numguesses to reflect the number of
    // guesses made during search.
    fn satisfy(&mut self, todoindex: usize, solution: &mut Sudoku) -> bool {
        // println!("todoindex: {}", todoindex);
        let (row, col, sub) = self.todo[todoindex];

        let mut candidates = self.rows[row] & self.cols[col] & self.subs[sub];
        while candidates != 0 {
            let ci = candidates.trailing_zeros() as u8;
            let candidate = candidates & (1 << candidates.trailing_zeros());

            // only count assignment as a guess if there's more than one candidate.
            if candidates ^ candidate != 0 {
                self.numguesses += 1;
            }

            // clear the candidate from available candidate sets for row, col, box
            self.rows[row] ^= candidate;
            self.cols[col] ^= candidate;
            self.subs[sub] ^= candidate;

            // recursively solve remaining todo cells and back out with the solution.
            if todoindex == self.numtodo || self.satisfy(todoindex + 1, solution) {
                solution[row * 9 + col] = ('1' as u8 + ci) as char;
                return true;
            }

            // restore the candidate to available candidate sets for row, col, box
            self.rows[row] |= candidate;
            self.cols[col] |= candidate;
            self.subs[sub] |= candidate;

            candidates = candidate ^ candidates;
        }
        return false;
    }
}

impl Solver for SolverBasic {
    fn solve(&mut self, puzzle: &Sudoku) -> Option<Sudoku> {
        let mut solution = Sudoku::default();
        self.init(puzzle, &mut solution);
        if self.satisfy(0, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }
}
