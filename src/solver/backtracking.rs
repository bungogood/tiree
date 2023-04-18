use std::vec;

use crate::solver::{neighbours, Solver};
use crate::sudoku::Sudoku;

pub struct Backtracking {
    neighbours: Vec<Vec<usize>>,
}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking {
            neighbours: neighbours(),
        }
    }

    fn eliminate(&self, queue: &mut Vec<(usize, u8)>, possible: &mut Vec<Vec<u8>>) -> bool {
        while let Some((i, v)) = queue.pop() {
            for &n in self.neighbours[i].iter() {
                match possible[n][..] {
                    [p] if p == v => return false,
                    [_] => continue,
                    _ => {
                        possible[n].retain(|&p| p != v);
                        if let [p] = possible[n][..] {
                            queue.push((n, p))
                        }
                    }
                }
            }
        }
        true
    }

    fn proc(&self, queue: &mut Vec<(usize, u8)>, possible: &mut Vec<Vec<u8>>) -> Option<Sudoku> {
        if !self.eliminate(queue, possible) {
            return None;
        }

        match possible
            .iter()
            .enumerate()
            .filter(|(_, v)| v.len() > 1)
            .min_by_key(|(_, v)| v.len())
        {
            None => return Some(possible.iter().map(|v| v[0]).collect()),
            Some((shortest, pos)) => {
                for &v in pos {
                    let mut replica = &mut possible.clone();
                    replica[shortest] = vec![v];
                    if let Some(state) = self.proc(&mut vec![(shortest, v)], &mut replica) {
                        return Some(state);
                    }
                }
                return None;
            }
        }
    }
}

impl Solver for Backtracking {
    fn solve(&self, puzzle: &Sudoku) -> Option<Sudoku> {
        let mut filled = vec![];
        let mut possible = puzzle
            .iter()
            .enumerate()
            .map(|(i, &v)| match v {
                0 => (1u8..=9).collect(),
                v => {
                    filled.push((i, v));
                    vec![v]
                }
            })
            .collect();
        return self.proc(&mut filled, &mut possible);
    }
}
