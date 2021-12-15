pub mod accu_iter;
pub mod input_parsing;
pub mod nested_iterator_chain;
pub mod rendering;
pub mod temporary_hashset;

#[macro_export]
macro_rules! aoc_tests {
    ( $( $suite:ident : { $( $name:ident => $expected_result:expr, )* } ),* ) => {
        $(
        #[cfg(test)]
        mod $suite {
            use std::path::{Path, PathBuf};
            use std::ffi::OsString;
            use std::fs;

            $(
            #[test]
            fn $name() {
                let data = {
                    let input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("input_data")
                        .join([
                            Path::new(file!()).file_stem().unwrap().to_os_string(),
                            "_".into(),
                            stringify!($name).into(),
                        ].into_iter().collect::<OsString>())
                        .with_extension("txt");
                    fs::read_to_string(&input_file).unwrap_or_else(
                        |e| panic!("Unable to open '{}': {}", input_file.into_os_string().into_string().unwrap(), e)
                    )
                };

                let input_data = super::parse_input(&data);
                let actual_result = super::$suite(&input_data);

                assert_eq!($expected_result, actual_result);
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
                        let t0 = std::time::Instant::now();
                        let input_data = solutions::$day::parse_input(data);
                        let t1 = std::time::Instant::now();
                        let solution = solutions::$day::$task(&input_data);
                        let t2 = std::time::Instant::now();
                        println!("   ... parse input: {} ms", (t1-t0).as_millis());
                        println!("   ... calculate: {} ms", (t2-t1).as_millis());
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

        mod renderers {
            $(
                pub mod $day;
            )*
        }

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
                        Ok(renderers::$day::$task(&input_data))
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
                        let t0 = std::time::Instant::now();
                        let input_data = reworked_solutions::$day::parse_input(data);
                        let t1 = std::time::Instant::now();
                        let solution = reworked_solutions::$day::$task(&input_data);
                        let t2 = std::time::Instant::now();
                        println!("   ... parse input: {} ms", (t1-t0).as_millis());
                        println!("   ... calculate: {} ms", (t2-t1).as_millis());
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
