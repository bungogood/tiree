mod backtracking;

pub use backtracking::Backtracking;

use crate::sudoku::Sudoku;

const SIZE: usize = 3;

pub trait Solver {
    fn solve(&self, sudoku: &Sudoku) -> Option<Sudoku>;
}

pub fn neighbours() -> Vec<Vec<usize>> {
    (0..81).map(|index| find_neighbours(index)).collect()
}

fn find_neighbours(index: usize) -> Vec<usize> {
    let (r, c) = Sudoku::itorc(index);
    let sr = (r / SIZE) * SIZE;
    let sc = (c / SIZE) * SIZE;

    let mut pos = vec![];

    let mut check = |ni: usize| {
        if ni != index && !pos.contains(&ni) {
            pos.push(ni);
        }
    };

    (0..9).for_each(|n| {
        check(Sudoku::rctoi(r, n));
        check(Sudoku::rctoi(n, c));
        let ri = n / SIZE;
        let ci = n % SIZE;
        check(Sudoku::rctoi(sr + ri, sc + ci));
    });
    pos
}
