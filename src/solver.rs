mod backtracking;

use std::ops::Index;
pub use backtracking::Backtracking;

use crate::sudoku::Sudoku;

const SIZE: usize = 3;

pub trait Solver {
    fn solve(&self, sudoku: &mut Sudoku) -> bool;
}

pub struct Neighbours {
    connections: [[usize; 20]; 81],
}

impl Neighbours {
    pub fn new() -> Self {
        let mut con: [[usize; 20]; 81] = [[0; 20]; 81];
        (0..81).for_each(|index| {
            con[index] = Self::get_neighbours(index);
        });
        Neighbours { connections: con }
    }

    fn get_neighbours(index: usize) -> [usize; 20] {
        let (r, c) = Sudoku::itorc(index);
        let sr = (r / SIZE) * SIZE;
        let sc = (c / SIZE) * SIZE;

        let mut pos = [0; 20];
        let mut count = 0;

        let mut check = |ni: usize| {
            if ni != index && !pos.contains(&ni) {
                pos[count] = ni;
                count += 1;
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
}

impl Index<usize> for Neighbours {
    type Output = [usize; 20];

    fn index(&self, index: usize) -> &Self::Output {
        &self.connections[index]
    }
}
