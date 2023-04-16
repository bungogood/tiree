use std::{fmt::Display, str::FromStr};

const SIZE: usize = 3;

#[derive(Clone)]
pub struct Sudoku {
    state: [u8; 81],
}

pub type Neighbours = [[usize; 20]; 81];

pub fn gen_neighbours() -> Neighbours {
    let mut pos: Neighbours = [[0; 20]; 81];
    (0..81).for_each(|index| {
        pos[index] = get_neighbours(index);
    });
    pos
}

pub fn get_neighbours(index: usize) -> [usize; 20] {
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

impl Sudoku {
    fn solve_rec(&mut self, index: usize, neighbours: Neighbours) -> bool {
        if index == 81 {
            return true;
        } else if self.state[index] != 0 {
            return self.solve_rec(index + 1, neighbours);
        }

        for v in 1..=9 {
            if neighbours[index].iter().any(|&ni| self.state[ni] == v) {
                continue;
            }

            self.state[index] = v;
            if self.solve_rec(index + 1, neighbours) {
                return true;
            }
        }

        self.state[index] = 0;
        return false;
    }

    pub fn solve(&mut self, neighbours: Neighbours) -> bool {
        self.solve_rec(0, neighbours)
    }

    const fn itorc(index: usize) -> (usize, usize) {
        (index / SIZE.pow(2), index % SIZE.pow(2))
    }

    const fn rctoi(row: usize, col: usize) -> usize {
        row * SIZE.pow(2) + col
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self { state: [0; 81] }
    }
}

impl FromStr for Sudoku {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let s = raw.replace(".", "0");
        let mut state = [0; 81];
        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10);
            match digit {
                Some(d) if d <= 9 => state[i] = d as u8,
                Some(_) => return Err(format!("Invalid digit at index {}", i)),
                None => return Err(format!("Non-digit character at index {}", i)),
            }
        }
        Ok(Sudoku { state: state })
    }
}

fn divisor(left: char, mid: char, right: char) -> String {
    format!(
        "{}{}{}",
        left,
        vec!["─".repeat(SIZE * 2 + 1); SIZE].join(&mid.to_string()),
        right
    )
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = divisor('┌', '┬', '┐');
        let mid = divisor('├', '┼', '┤');
        let bot = divisor('└', '┴', '┘');

        let mut out = String::new();

        out.push_str(&top);
        out.push('\n');
        for (i, &v) in self.state.iter().enumerate() {
            let (r, c) = Sudoku::itorc(i);
            if c == 0 {
                out.push_str("│ ");
            }

            if v != 0 {
                out.push_str(format!("{} ", v).as_str());
            } else {
                out.push_str(". ")
            }

            if c % SIZE == SIZE - 1 {
                out.push_str("│ ");
            }

            if c == SIZE.pow(2) - 1 {
                out.push('\n');
                if r % SIZE == SIZE - 1 && r != SIZE.pow(2) - 1 {
                    out.push_str(&mid);
                    out.push('\n');
                };
            }
        }
        out.push_str(&bot);
        write!(f, "{}", out)
    }
}
