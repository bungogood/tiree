use std::{fmt::Display, str::FromStr};

const SIZE: usize = 3;

#[derive(Clone)]
pub struct Sudoku {
    pub state: [u8; 81]
}

pub trait Solver {
    fn solve(&self, sudoku: &mut Sudoku) -> bool;
}

impl Sudoku {
    pub const fn itorc(index: usize) -> (usize, usize) {
        (index / SIZE.pow(2), index % SIZE.pow(2))
    }

    pub const fn rctoi(row: usize, col: usize) -> usize {
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
