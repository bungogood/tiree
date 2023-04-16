mod backtracking;
pub use backtracking::Backtracking;

use crate::sudoku::Sudoku;

pub trait Solver {
    fn solve(&self, sudoku: &mut Sudoku) -> bool;
}
