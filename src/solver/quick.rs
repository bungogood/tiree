use std::vec;

use crate::solver::{neighbours, Solver};
use crate::sudoku::Sudoku;

pub struct Quick {
    neighbours: Vec<Vec<usize>>,
}

impl Quick {
    pub fn new() -> Self {
        Quick {
            neighbours: neighbours(),
        }
    }

    fn eliminate(&self, worklist: &mut Vec<(usize, u8)>, possible:&mut Vec<Vec<u8>>) -> bool {
        while let Some((x, p)) = worklist.pop() {
            for &n in self.neighbours[x].iter() {
                if possible[n].len() == 1 {
                    continue;
                }
                possible[n].retain(|&v| v != p);
                match possible[n][..] {
                    [] => return false,
                    [v] => worklist.push((n, v)),
                    _ => {}
                }
            }
        }
        true
    }

    fn proc(&self, worklist: &mut Vec<(usize, u8)>, possible:&mut Vec<Vec<u8>>) -> Option<Vec<u8>> {
        if !self.eliminate(worklist, possible) {
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
                    if let Some(pos) = self.proc(&mut vec![(shortest, v)], &mut replica) {
                        return Some(pos);
                    }
                }
                return None;
            }
        }
    }
}

impl Solver for Quick {
    fn solve(&self, puzzle: &mut Sudoku) -> bool {
        let mut filled: Vec<(usize, u8)> = puzzle
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(i, v)| (i, *v))
            .collect();
        let mut possible: Vec<Vec<u8>> = puzzle
            .iter()
            .map(|&v| if v == 0 { (1u8..=9).collect() } else { vec![v] })
            .collect();
        match self.proc(&mut filled, &mut possible) {
            None => false,
            Some(pos) => {
                pos.iter().enumerate().for_each(|(i, &v)| puzzle[i] = v);
                true
            }
        }
    }
}
