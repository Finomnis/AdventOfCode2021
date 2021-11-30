pub fn task1(_input_data_raw: &[u8]) -> String {
    String::from("AAA")
}

pub fn task2(_input_data_raw: &[u8]) -> String {
    String::from("")
}

crate::aoc_tests! {
    task1: {
        (simple, "day01.txt", "AAA")
        (complex, "day01.txt", "AAA")
    },
    task2: {
        (simple, "day01.txt", "")
        (complex, "day01.txt", "")
    }
}
