use std::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub trait Solver {
    fn solve(&mut self, sudoku: &Sudoku) -> Option<Sudoku>;
    fn guesses(&self) -> usize;
}

pub const SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sudoku([char; 81]);

impl Default for Sudoku {
    fn default() -> Self {
        Self(['.'; 81])
    }
}

impl Deref for Sudoku {
    type Target = [char; 81];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<char> for Sudoku {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        return Self(iter.into_iter().collect::<Vec<char>>().try_into().unwrap());
    }
}

impl FromStr for Sudoku {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut state = Self::default();

        let s = raw.replace("0", ".");
        for (i, c) in s.chars().enumerate() {
            state[i] = c;
        }

        Ok(state)
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &cell in self.iter() {
            write!(f, "{}", cell)?
        }
        Ok(())
    }
}

fn divisor(left: char, mid: char, right: char) -> String {
    format!(
        "{left}{}{right}",
        vec!["─".repeat(SIZE * 2 + 1); SIZE].join(&mid.to_string())
    )
}

impl Sudoku {
    pub fn out(&self) -> String {
        let s: String = self.iter().map(|v| v.to_string()).collect();
        s.replace(".", "0")
    }

    pub fn pretty(&self) -> String {
        let top = divisor('┌', '┬', '┐');
        let mid = divisor('├', '┼', '┤');
        let bot = divisor('└', '┴', '┘');

        let mut out = String::new();

        out.push_str(&top);
        out.push('\n');
        for (i, &v) in self.iter().enumerate() {
            let (r, c) = (i / SIZE.pow(2), i % SIZE.pow(2));
            if c == 0 {
                out.push_str("│");
            }

            match v {
                '0' => out.push_str(" ."),
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
}
