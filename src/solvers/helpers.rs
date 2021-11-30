#[macro_export]
macro_rules! aoc_tests {
    ( $( $suite:ident : { $( ($name:ident, $input_file:expr, $expected_result:expr) )* } ),* ) => {
        $(
        #[cfg(test)]
        mod $suite {
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::prelude::*;

            $(
            #[test]
            fn $name() {
                let input_data = {
                    let input_file_path: &str = $input_file;
                    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("input_data")
                        .join(input_file_path);
                    let mut data = Vec::new();
                    File::open(path)
                        .unwrap()
                        .read_to_end(&mut data)
                        .unwrap();
                    data
                };

                let expected_result: &str = $expected_result;
                let actual_result = super::$suite(&input_data);

                assert_eq!(expected_result, actual_result);
            }
            )*
        }
        )*
    };
}
