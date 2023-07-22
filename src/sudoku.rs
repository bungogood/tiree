use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

const SIZE: usize = 3;

#[derive(Clone)]
pub struct Sudoku {
    state: Vec<u8>,
}

impl Sudoku {
    pub const fn itorc(index: usize) -> (usize, usize) {
        (index / SIZE.pow(2), index % SIZE.pow(2))
    }

    pub const fn rctoi(row: usize, col: usize) -> usize {
        row * SIZE.pow(2) + col
    }

    pub fn human(&self) -> String {
        let s: String = self.state.iter().map(|v| v.to_string()).collect();
        s.replace("0", ".")
    }

    pub fn pretty(&self) -> String {
        let top = divisor('┌', '┬', '┐');
        let mid = divisor('├', '┼', '┤');
        let bot = divisor('└', '┴', '┘');

        let mut out = String::new();

        out.push_str(&top);
        out.push('\n');
        for (i, &v) in self.state.iter().enumerate() {
            let (r, c) = Sudoku::itorc(i);
            if c == 0 {
                out.push_str("│");
            }

            match v {
                0 => out.push_str(" ."),
                _ => out.push_str(format!(" {v}").as_str()),
            }

            if c % SIZE == SIZE - 1 {
                out.push_str(" │");
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
        out
    }
    pub fn _pretty(&self) -> String {
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

            match v {
                0 => out.push_str(". "),
                _ => out.push_str(format!("{v} ").as_str()),
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
        out
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self { state: vec![0; 81] }
    }
}

impl Deref for Sudoku {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

// will need to check for length
impl FromIterator<u8> for Sudoku {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        return Self {
            state: iter.into_iter().collect(),
        };
    }
}

impl FromStr for Sudoku {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut state = vec![0; 81];

        let s = raw.replace(".", "0");
        for (i, c) in s.chars().enumerate() {
            match c.to_digit(10) {
                Some(v) => state[i] = v as u8,
                None => return Err(format!("Invalid character: {}", c)),
            }
        }

        Ok(Sudoku { state: state })
    }
}

fn divisor(left: char, mid: char, right: char) -> String {
    format!(
        "{left}{}{right}",
        vec!["─".repeat(SIZE * 2 + 1); SIZE].join(&mid.to_string())
    )
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.state.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", s)
    }
}
