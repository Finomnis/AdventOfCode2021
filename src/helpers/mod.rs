pub mod accu_iter;
pub mod input_parsing;

#[macro_export]
macro_rules! aoc_tests {
    ( $( $suite:ident : { $( ($name:ident, $input_file:expr, $expected_result:expr) )* } ),* ) => {
        $(
        #[cfg(test)]
        mod $suite {
            use std::path::PathBuf;
            use std::fs;

            $(
            #[test]
            fn $name() {
                let data = {
                    let input_file_path: &str = $input_file;
                    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("input_data")
                        .join(input_file_path);
                    fs::read_to_string(path).unwrap()
                };

                let expected_result: &str = $expected_result;
                let input_data = super::parse_input(&data);
                let actual_result = format!("{}", super::$suite(&input_data));

                assert_eq!(expected_result, actual_result);
            }
            )*
        }
        )*
    };
}

#[macro_export]
macro_rules! solutions {
    ( $( ($day:ident, $($task:ident),* ) )* ) => {

        mod solutions{
            $(
                pub mod $day;
            )*
        }

        fn run_solution(day: u8, task: u8, data: &str) -> Result<String> {
            let day_str = format!("day{:0>2}", day);
            let task_str = format!("task{}", task);

            match (day_str.as_str(), task_str.as_str()) {
                $($(
                    (stringify!($day), stringify!($task)) => {
                        println!(
                            "Running solver {}::{} ...",
                            stringify!($day),
                            stringify!($task)
                        );
                        let input_data = solutions::$day::parse_input(data);
                        let solution = solutions::$day::$task(&input_data);
                        Ok(format!("{}", solution))
                    },
                )*)*
                _ => Err(anyhow!(
                    "Unable to find solver for day {}, task {}!",
                    day,
                    task
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! renderers {
    ( $( ($day:ident, $($task:ident),* ) )* ) => {
        fn run_renderer(day: u8, task: u8, data: &str) -> Result<Vec<String>> {
            let day_str = format!("day{:0>2}", day);
            let task_str = format!("task{}", task);

            match (day_str.as_str(), task_str.as_str()) {
                $($(
                    (stringify!($day), stringify!($task)) => {
                        println!(
                            "Rendering {}::{} ...",
                            stringify!($day),
                            stringify!($task)
                        );
                        let input_data = solutions::$day::parse_input(data);
                        Ok(solutions::$day::render::$task(&input_data))
                    },
                )*)*
                _ => Err(anyhow!(
                    "Unable to find renderer for day {}, task {}!",
                    day,
                    task
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! reworked_solutions {
    ( $( ($day:ident, $($task:ident),* ) )* ) => {

        mod reworked_solutions {
            $(
                pub mod $day;
            )*
        }

        fn run_reworked_solutions(day: u8, task: u8, data: &str) -> Result<String> {
            let day_str = format!("day{:0>2}", day);
            let task_str = format!("task{}", task);

            match (day_str.as_str(), task_str.as_str()) {
                $($(
                    (stringify!($day), stringify!($task)) => {
                        println!(
                            "Running reworked solver {}::{} ...",
                            stringify!($day),
                            stringify!($task)
                        );
                        let input_data = reworked_solutions::$day::parse_input(data);
                        let solution = reworked_solutions::$day::$task(&input_data);
                        Ok(format!("{}", solution))
                    },
                )*)*
                _ => Err(anyhow!(
                    "Unable to find reworked solver for day {}, task {}!",
                    day,
                    task
                ))
            }
        }
    };
}
