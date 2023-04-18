# Sudoku
![Rust](https://github.com/bungogood/sudoku/actions/workflows/rust.yml/badge.svg)

```
Puzzle
┌───────┬───────┬───────┐
│ . . 4 │ 3 . . │ 2 . 9 │ 
│ . . 5 │ . . 9 │ . . 1 │ 
│ . 7 . │ . 6 . │ . 4 3 │ 
├───────┼───────┼───────┤
│ . . 6 │ . . 2 │ . 8 7 │ 
│ 1 9 . │ . . 7 │ 4 . . │ 
│ . 5 . │ . 8 3 │ . . . │ 
├───────┼───────┼───────┤
│ 6 . . │ . . . │ 1 . 5 │ 
│ . . 3 │ 5 . 8 │ 6 9 . │ 
│ . 4 2 │ 9 1 . │ 3 . . │ 
└───────┴───────┴───────┘
Solved
┌───────┬───────┬───────┐
│ 8 6 4 │ 3 7 1 │ 2 5 9 │ 
│ 3 2 5 │ 8 4 9 │ 7 6 1 │ 
│ 9 7 1 │ 2 6 5 │ 8 4 3 │ 
├───────┼───────┼───────┤
│ 4 3 6 │ 1 9 2 │ 5 8 7 │ 
│ 1 9 8 │ 6 5 7 │ 4 3 2 │ 
│ 2 5 7 │ 4 8 3 │ 9 1 6 │ 
├───────┼───────┼───────┤
│ 6 8 9 │ 7 3 4 │ 1 2 5 │ 
│ 7 1 3 │ 5 2 8 │ 6 9 4 │ 
│ 5 4 2 │ 9 1 6 │ 3 7 8 │ 
└───────┴───────┴───────┘
Time Taken: 48.33µs
File: sudokus/1000000.txt Contains: 1000000 sudokus 
Time Taken: 19.05s, Speed: 52505.91/s, Avg: 19.05µs
```

## References
Puzzles from:
- [Kaggle](https://www.kaggle.com/datasets/bryanpark/sudoku) Dataset
- [Code Golf](https://codegolf.stackexchange.com/questions/190727/the-fastest-sudoku-solver) Challenge
