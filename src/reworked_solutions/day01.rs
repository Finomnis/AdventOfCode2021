pub fn parse_input(input_data: &str) -> Vec<u32> {
    input_data
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn task1(input_data: &[u32]) -> usize {
    input_data.windows(2).filter(|v| v[1] > v[0]).count()
}

pub fn task2(input_data: &[u32]) -> usize {
    task1(
        &input_data
            .windows(3)
            .map(|v| v.iter().sum())
            .collect::<Vec<_>>(),
    )
}

crate::aoc_tests! {
    task1: {
        (simple, "7")
        (complex, "1655")
    },
    task2: {
        (simple, "5")
        (complex, "1683")
    }
}
