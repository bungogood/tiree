use std::vec;

use crate::solver::{Neighbours, Solver};
use crate::sudoku::Sudoku;

struct State {
    worklist: Vec<usize>,
    // connections: Vec<Vec<usize>>,
    possible: Vec<Vec<u8>>,
    placed: usize,
}

pub struct Quick {
    neighbours: Neighbours,
}

impl Quick {
    pub fn new() -> Self {
        Quick {
            neighbours: Neighbours::new(),
        }
    }

    fn eliminate(&self, state: &mut State) -> bool {
        while let Some(x) = state.worklist.pop() {
            for &n in self.neighbours[x].iter() {
                if state.possible[n].len() == 1 {
                    continue;
                }
                let value = state.possible[x][0];
                state.possible[n].retain(|&v| v != value);
                match state.possible[n].len() {
                    0 => return false,
                    1 => {
                        state.placed += 1;
                        state.worklist.push(n)
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn proc(&self, state: &mut State) -> Option<Vec<u8>> {
        if !self.eliminate(state) {
            return None;
        }

        match state
        .possible
        .iter()
        .enumerate()
        .filter(|(_, v)| v.len() > 1)
        .min_by_key(|(_, v)| v.len()) {
            None => return Some(state.possible.iter().map(|v| v[0]).collect()),
            Some((shortest, pos)) => {
                for &v in pos {
                    let mut dup = State {
                        worklist: vec![shortest], // all filled squared
                        // connections: state.connections.clone(),
                        possible: state.possible.clone(),
                        placed: state.placed,
                    };
                    dup.possible[shortest] = vec![v];
                    if let Some(pos) = self.proc(&mut dup) {
                        return Some(pos);
                    }
                }
                return None
            }
        }
    }
}

impl Solver for Quick {
    fn solve(&self, puzzle: &mut Sudoku) -> bool {
        let filled: Vec<usize> = puzzle
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(i, _)| i)
            .collect();
        let placed = filled.len();
        let possible: Vec<Vec<u8>> = puzzle
            .iter()
            .map(|&v| if v == 0 {
                (1u8..=9).collect()
            } else {
                vec![v]
            })
            .collect();
        let mut state = State {
            worklist: filled,
            // connections: self.neighbours.clone,
            possible: possible,
            placed: placed
        };
        match self.proc(&mut state) {
            None => false,
            Some(pos) => {
                pos.iter().enumerate().for_each(|(i, &v)| puzzle[i] = v);
                true
            }
        }
    }
}
