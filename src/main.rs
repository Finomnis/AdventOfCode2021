use anyhow::{anyhow, Result};
use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

use advent_of_code_2021::*;

macro_rules! solvers {
    ( $( ($day:ident, $task:ident) )* ) => {
        fn run_solver(day: u8, task: u8, data: &[u8]) -> Result<String> {
            let day_str = format!("day{:0>2}", day);
            let task_str = format!("task{}", task);

            match (day_str.as_str(), task_str.as_str()) {
                $(
                    (stringify!($day), stringify!($task)) => {
                        println!(
                            "Running solver {}::{} ...",
                            stringify!($day),
                            stringify!($task)
                        );
                        Ok($day::$task(data))
                    },
                )*
                _ => Err(anyhow!(
                    "Unable to find solver for day {}, task {}!",
                    day,
                    task
                ))
            }
        }
    };
}

// DAILY: Add new solvers here
solvers! {
    (day01, task1)
    (day01, task2)
}

/// This is a solver for Advent of Code 2021 tasks.
#[derive(Parser)]
#[clap()]
pub struct Options {
    /// The day of the challenge, can be 1-24
    #[clap()]
    pub day: u8,

    /// The task on the day, can be 1 or 2
    #[clap()]
    pub task: u8,

    #[clap()]
    pub data: PathBuf,
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let input_file_path = opts.data;
    let mut data = Vec::new();
    File::open(&input_file_path)
        .map_err(|err| {
            anyhow!(
                "Unable to open '{}': {}",
                &input_file_path.into_os_string().into_string().unwrap(),
                err
            )
        })?
        .read_to_end(&mut data)?;

    let result = run_solver(opts.day, opts.task, &data)?;

    println!("─ Result: ──────────────────────────────────────");
    println!("{}", result);
    println!("────────────────────────────────────────────────");

    Ok(())
}
