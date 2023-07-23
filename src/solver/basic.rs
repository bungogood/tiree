use crate::sudoku::{Solver, Sudoku};

type Bits = u32;
const ALL: Bits = 0x1ff;

type RowColSub = (usize, usize, usize);

pub struct SolverBasic {
    rows: [Bits; 9],
    cols: [Bits; 9],
    subs: [Bits; 9],
    todo: Vec<RowColSub>,
    limit: usize,
    min_heuristic: bool,
    num_todo: usize,
    guesses: usize,
    num_solutions: usize,
}

impl SolverBasic {
    pub fn new(limit: usize, min_heuristic: bool) -> Self {
        Self {
            rows: [ALL; 9],
            cols: [ALL; 9],
            subs: [ALL; 9],
            todo: vec![],
            num_todo: 0,
            guesses: 0,
            limit: limit,
            min_heuristic: min_heuristic,
            num_solutions: 0,
        }
    }

    fn init(&mut self, puzzle: &Sudoku, solution: &mut Sudoku) -> bool {
        self.rows.fill(ALL);
        self.cols.fill(ALL);
        self.subs.fill(ALL);
        self.guesses = 0;
        self.num_solutions = 0;

        // Copy initial clues to the solution since our todo list won't include these cells.
        self.todo.clear();

        for row in 0..9 {
            for col in 0..9 {
                let cell = row * 9 + col;
                let sub = (row / 3) * 3 + col / 3;
                solution[cell] = puzzle[cell];
                if puzzle[row * 9 + col] == '.' {
                    // Blank cell: add to the todo list.
                    self.todo.push((row, col, sub));
                } else {
                    // A given clue: clear availability bits for row, col, and box.
                    let value = 1u32 << (puzzle[cell] as u32 - b'1' as u32);
                    if self.rows[row] & value != 0
                        && self.cols[col] & value != 0
                        && self.subs[sub] & value != 0
                    {
                        self.rows[row] ^= value;
                        self.cols[col] ^= value;
                        self.subs[sub] ^= value;
                    } else {
                        return false;
                    }
                }
            }
        }
        self.num_todo = self.todo.len() - 1;
        true
    }

    fn mcv(&mut self, todo_index: usize) {
        let sublist = &mut self.todo[todo_index..];
        if let Some(min_element_index) = sublist
            .iter()
            .enumerate()
            .min_by_key(|&(_, &(row, col, sub))| {
                let candidates = self.rows[row] & self.cols[col] & self.subs[sub];
                candidates.count_ones()
            })
            .map(|(index, _)| index)
        {
            sublist.swap(0, min_element_index);
        }
    }

    fn satisfy(&mut self, todo_index: usize, solution: &mut Sudoku) -> bool {
        if self.min_heuristic {
            self.mcv(todo_index);
        }

        let (row, col, sub) = self.todo[todo_index];

        let mut candidates = self.rows[row] & self.cols[col] & self.subs[sub];
        // println!("canditates: {}", candidates.count_ones());

        while candidates != 0 {
            let ci = candidates.trailing_zeros() as u8;
            let candidate = 1 << ci;

            // Only count assignment as a guess if there's more than one candidate.
            if candidates ^ candidate != 0 {
                self.guesses += 1;
            }

            // Clear the candidate from available candidate sets for row, col, box.
            self.rows[row] ^= candidate;
            self.cols[col] ^= candidate;
            self.subs[sub] ^= candidate;

            solution[row * 9 + col] = (b'1' + ci) as char;
            // Recursively solve remaining cells and back out with the last solution.
            if todo_index < self.num_todo {
                self.satisfy(todo_index + 1, solution);
            } else {
                self.num_solutions += 1;
            }

            if self.num_solutions == self.limit {
                return true;
            }

            // Restore the candidate to available candidate sets for row, col, box.
            self.rows[row] ^= candidate;
            self.cols[col] ^= candidate;
            self.subs[sub] ^= candidate;

            candidates ^= candidate;
        }
        return false;
    }
}

impl Solver for SolverBasic {
    fn solve(&mut self, puzzle: &Sudoku) -> Option<Sudoku> {
        let mut solution = Sudoku::default();
        if self.init(puzzle, &mut solution) && self.satisfy(0, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    fn guesses(&self) -> usize {
        self.guesses
    }
}
