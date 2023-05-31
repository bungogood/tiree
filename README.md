# Sudoku
[![Build](../../actions/workflows/build.yaml/badge.svg)](../../actions/workflows/build.yaml)

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

## Challenge

**_NOTE:_**  Both input and output files contain a single trailing newline.

Unsolved sudokus are of the from:
```
000000010400000000020000000000050407008000300001090000300400200050100000000806000
```
Where:
- Each puzzle is 81 characters
- `0` is an unkown value
- `1-9` is a filled cell

Input files of the form:
```
<num_puzzles>
<unsolved_puzzle#1>
<unsolved_puzzle#2>
...
<unsolved_puzzle#n>

```

Output files of the form:
```
<num_puzzles>
<unsolved_puzzle#1>,<solved_puzzle#1>
<unsolved_puzzle#2>,<solved_puzzle#2>
...
<unsolved_puzzle#n>,<solved_puzzle#n>

```

To check the solutions use `sha256sum <output.txt>` and check the hash against the following:

| file | sha256sum |
|-----|-----------|
| `all-17-clue.txt` | `0bc8dda364db7b99f389b42383e37b411d9fa022204d124cb3c8959eba252f05` |
| `hard.txt`        | `b3df4de0e6f9d94b923ff2474db4da792c37e17ed4ad8dca2537fb4d65d35c83` |
| `1000000.txt`     | `4cb8f2b293a420e4397fc2bb1c541297e0a4fc43335f75e13ccc9833d0558cf3` |

## Files

To unzip sudokus:
```bash
unzip sudokus.zip
```

To zip sudokus:
```bash
zip -r sudokus.zip sudokus
```

## References
Puzzles from:
- [Kaggle](https://www.kaggle.com/datasets/bryanpark/sudoku) Dataset
- [Code Golf](https://codegolf.stackexchange.com/questions/190727/the-fastest-sudoku-solver) Challenge
