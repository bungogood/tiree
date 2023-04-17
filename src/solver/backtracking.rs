use crate::solver::{Neighbours, Solver};
use crate::sudoku::Sudoku;

const SIZE: usize = 3;

// https://en.wikipedia.org/wiki/Backtracking

pub struct Backtracking {
    neighbours: Neighbours,
}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking {
            neighbours: Neighbours::new(),
        }
    }

    fn backtrack(&self, puzzle: &mut Sudoku, index: usize) -> bool {
        if index == SIZE.pow(4) {
            return true;
        } else if puzzle[index] != 0 {
            return self.backtrack(puzzle, index + 1);
        }

        for v in 1..=SIZE.pow(2) {
            if self.neighbours[index]
                .iter()
                .any(|&ni| puzzle[ni] == v as u8)
            {
                continue;
            }

            puzzle[index] = v as u8;
            if self.backtrack(puzzle, index + 1) {
                return true;
            }
        }

        puzzle[index] = 0;
        return false;
    }
}

impl Solver for Backtracking {
    fn solve(&self, puzzle: &mut Sudoku) -> bool {
        self.backtrack(puzzle, 0)
    }
}
