# AdventOfCode2021
My solutions for the https://adventofcode.com/2021 challenge.

## Build & run

- Install Cargo + Rust: https://rustup.rs
- Run solver:
  ```
  cargo run --release -- <day> <task> <data-file>
  ```
  with:
    - <day>: The day of the challenge, from 1 to 24
    - <task>: The task on the day, either 1 or 2
    - <data-file>: The path to a file containing the challenge input data

  Alternatively, building and running can be split into two steps:
  ```
  cargo build --release
  ./target/release/advent-of-code-2021 <day> <task> <data-file>
  ```

## Running tests

To run tests for all existing solvers, run:
```
cargo test --release
```
