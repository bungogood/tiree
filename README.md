# Tiree

[![Build](../../actions/workflows/build.yaml/badge.svg)](../../actions/workflows/build.yaml)

**Sudoku** (数独, sūdoku, digit-single) (/suːˈdoʊkuː/, /-ˈdɒk-/, /sə-/, originally called **Number Place**) is a logic-based, combinatorial number-placement puzzle. The objective is to fill a 9×9 grid with digits so that each column, each row, and each of the nine 3×3 subgrids that compose the grid (also called "boxes", "blocks", or "regions") contain all of the digits from 1 to 9. The puzzle setter provides a partially completed grid, which for a well-posed puzzle has a single solution.

Completed games are always an example of a Latin square which include an additional constraint on the contents of individual regions. For example, the same single integer may not appear twice in the same row, column, or any of the nine 3×3 subregions of the 9×9 playing board.

## Usage

```
Usage of Tiree:
tiree [-s|-u|-a|-c] [-f infile] [-o outfile] sudoku

  -s --solve         Returns solution to soduku
  -u --unsolve       Returns minium clue for soduku
  -a --all-solutions Returns all solutions to soduku
  -c --compress      Compresses sudokus
  -v --verbose       Gives a more detailed output
  -h --help          Gives all possible options
```

Unsolved sudokus are of the from:

```
..43..2.9..5..9..1.7..6..43..6..2.8719...74...5..83...6.....1.5..35.869..4291.3..
004300209005009001070060043006002087190007400050083000600000105003508690042910300
```

Where:

- Each puzzle is 81 characters
- `0` or `.` is an unfilled cell
- `1-9` is a filled cell

**_NOTE:_** Both input and output files contain a single trailing newline.

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

## Examples

```
$ tiree -v -f sudokus/1000000.txt
File: sudokus/1000000.txt Contains: 1000000 sudokus
First Puzzle:
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
..43..2.9..5..9..1.7..6..43..6..2.8719...74...5..83...6.....1.5..35.869..4291.3..
Solution:
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
864371259325849761971265843436192587198657432257483916689734125713528694542916378
Time Taken: 14.77s, Speed: 67690.29/s, Avg: 14.77µs
SHA-256 Hash: 4cb8f2b293a420e4397fc2bb1c541297e0a4fc43335f75e13ccc9833d0558cf3
```

## Solver

To check the solutions use `sha256sum <outfile>` and check the hash against the following:

| file                      | sha256sum                                                          |
| ------------------------- | ------------------------------------------------------------------ |
| `all_17_clue_sudokus.txt` | `0bc8dda364db7b99f389b42383e37b411d9fa022204d124cb3c8959eba252f05` |
| `hard_sudokus.txt`        | `b3df4de0e6f9d94b923ff2474db4da792c37e17ed4ad8dca2537fb4d65d35c83` |
| `ppcg_sudoku_testing.txt` | `Not Yet Found`                                                    |
| `1000000.txt`             | `4cb8f2b293a420e4397fc2bb1c541297e0a4fc43335f75e13ccc9833d0558cf3` |
| `3000000.txt`             | `0e17c05d1856d96d7b103b3f9320a6f41cf1908d6465ae6287f257f2fb2d63e4` |
| `9000000.txt`             | `817be7617a8dc46c44976d6732a8e5161a620c326acc6679b60b0b2889580ea6` |

## Unsolver

This aims to find the minimum clue sudoku.

## References

- Sudoku [Wikipedia](https://en.wikipedia.org/wiki/Sudoku)
- Fastest Sudoku solver [Challenge](https://codegolf.stackexchange.com/questions/190727/the-fastest-sudoku-solver)
- Minimum-Clue Sudoku Unsolver [Challemnge](https://codegolf.stackexchange.com/questions/48509/build-a-minimum-clue-sudoku-unsolver)
- Sudoku Compression [Challenge](https://codegolf.stackexchange.com/questions/41523/sudoku-compression)
- Latin Square Compression [Challenge](https://codegolf.stackexchange.com/questions/85239/latin-square-compression)
- [1 million](https://www.kaggle.com/datasets/bryanpark/sudoku) Sudoku games by [Kyubyong Park](https://www.kaggle.com/bryanpark)
- [3 Million](https://www.kaggle.com/datasets/rohanrao/sudoku) Sudoku puzzles with ratings by [David Radcliffe](https://www.kaggle.com/radcliffe)
- [9 Million](https://www.kaggle.com/datasets/rohanrao/sudoku) Sudoku Puzzles and Solutions by [Rohan Rao](https://www.kaggle.com/rohanrao)
