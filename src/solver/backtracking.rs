use crate::solver::{Neighbours, Solver};
use crate::sudoku::Sudoku;

const SIZE: usize = 3;

pub struct Backtracking {
    neighbours: Neighbours,
}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking {
            neighbours: Neighbours::new(),
        }
    }

    fn solve_rec(&self, puzzle: &mut Sudoku, index: usize) -> bool {
        if index == SIZE.pow(4) {
            return true;
        } else if puzzle.state[index] != 0 {
            return self.solve_rec(puzzle, index + 1);
        }

        for v in 1..=SIZE.pow(2) {
            if self.neighbours[index]
                .iter()
                .any(|&ni| puzzle.state[ni] == v as u8)
            {
                continue;
            }

            puzzle.state[index] = v as u8;
            if self.solve_rec(puzzle, index + 1) {
                return true;
            }
        }

        puzzle.state[index] = 0;
        return false;
    }
}

impl Solver for Backtracking {
    fn solve(&self, puzzle: &mut Sudoku) -> bool {
        self.solve_rec(puzzle, 0)
    }
}
