use crate::sudoku::{
    Sudoku,
    Solver
};

const SIZE: usize = 3;

pub type Neighbours = [[usize; 20]; 81];

pub struct Backtracking {
    neighbours: Neighbours
}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking { neighbours: Self::init_neighbours() }
    }

    fn init_neighbours() -> Neighbours {
        let mut pos: Neighbours = [[0; 20]; 81];
        (0..81).for_each(|index| {
            pos[index] = Self::get_neighbours(index);
        });
        pos
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


    fn solve_rec(&self, puzzle: &mut Sudoku, index: usize) -> bool {
        if index == 81 {
            return true;
        } else if puzzle.state[index] != 0 {
            return self.solve_rec(puzzle, index + 1);
        }

        for v in 1..=9 {
            if self.neighbours[index].iter().any(|&ni| puzzle.state[ni] == v) {
                continue;
            }

            puzzle.state[index] = v;
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