use anyhow::{anyhow, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

mod helpers;

// DAILY: Add new solvers here
solvers! {
    (day01, task1, task2)
    (day02, task1, task2)
    (day03, task1, task2)
    (day04, task1, task2)
    (day05, task1, task2)
    (day06, task1, task2)
    (day07, task1, task2)
    (day08, task1, task2)
}

// DAILY: Add new reference solutions here
reference_solutions! {
    (day01, task1, task2)
}

// DAILY: Add new renderers here
renderers! {
    (day05, task1, task2)
}

/// This is a solver for Advent of Code 2021 tasks.
#[derive(Parser)]
#[clap()]
pub struct Options {
    /// The day of the challenge, can be 1-25
    #[clap()]
    pub day: u8,

    /// The task on the day, can be 1 or 2
    #[clap()]
    pub task: u8,

    /// The path to the challenge input data
    #[clap()]
    pub data: PathBuf,

    /// Run the reference solution of mine
    #[clap(short, long)]
    pub reference: bool,

    /// Render the task visually, if available
    #[clap(long)]
    pub render: bool,
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let input_file_path = opts.data;
    let data = fs::read_to_string(&input_file_path).map_err(|err| {
        anyhow!(
            "Unable to open '{}': {}",
            &input_file_path.into_os_string().into_string().unwrap(),
            err
        )
    })?;

    if opts.render {
        let artifacts = run_renderer(opts.day, opts.task, &data)?;
        println!("─ Rendering artifacts: ─────────────────────────");
        for artifact in artifacts {
            println!("{}", artifact);
        }
        println!("────────────────────────────────────────────────");
        return Ok(());
    }

    let result = match opts.reference {
        true => run_reference_solutions(opts.day, opts.task, &data)?,
        false => run_solver(opts.day, opts.task, &data)?,
    };

    println!("─ Result: ──────────────────────────────────────");
    println!("{}", result);
    println!("────────────────────────────────────────────────");

    Ok(())
}
