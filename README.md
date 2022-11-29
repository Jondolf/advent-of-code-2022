# 🎄 Advent of Code 2022

Here are my solutions to the [Advent of Code 2022](https://adventofcode.com/2022), this year in Rust.

## Running the solutions

Run all solutions:

```
cargo run
```

Run just one day's solution:

```
cargo run -- --day <day>
```

## Scaffolding new days

This project also contains a (pretty over-engineered) binary for generating everything required for the puzzles.
It creates a `mod.rs` file for the day with some template code and adds all of the necessary `mod` declarations and imports.

The binary also creates an `input.txt` file with the user's input fetched automatically.
Note that this requires the user to have the `AOC_SESSION` environment variable set to the user's session cookie (which you can get from the browser's network tab on the AoC website).

To scaffold a new day, run the command below:

```
cargo run --bin scaffold -- --day <day>
```
