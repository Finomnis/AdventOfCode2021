# AdventOfCode2021

[![Build Status](https://img.shields.io/github/workflow/status/Finomnis/AdventOfCode2021/CI/main)](https://github.com/Finomnis/AdventOfCode2021/actions/workflows/ci.yml?query=branch:main)

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

## Development

Recommended development environment:

- VSCode
- Plugin `rust-analyzer` (NOT the official `Rust` plugin)

This enables syntax highlighting, auto-completion, in-line type hints,
error highlighting and buttons to run specific tests only (directly at the tests).

## Adding new solvers

When a new challenge is released, perform the following steps:

- Copy input data to `input_data`
- Add a new solver as `src/solvers/dayXX.rs`
- Add the solver to the compilation in `src/solvers/mod.rs`
- Register the solver in the command line tool, in `src/main.rs`

Then, populate the solver with:

- The solver for the task, as `task1()` or `task2()`
- Tests:
  - The `aoc_tests` macro makes this trivially easy,
    just write:
    ```
    crate::aoc_tests! {
        task1: {
            (
                <test_name>,
                <input_file>,
                <expected_result>
            )
        },
        task2: {
            // For example:
            (simple, "day01_simple.txt", "abcde")
            (complex, "day01_complex.txt", "result")
        }
    }
    ```
  - Add the small test from challenge text right away
  - Add the larger test with the real challenge input data once the answer was accepted on the website
