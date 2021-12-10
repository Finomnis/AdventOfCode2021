pub fn parse_input(input_data: &str) -> Vec<i64> {
    input_data
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn task1(input_data: &[i64]) -> i64 {
    let mut crabs = input_data.to_vec();
    crabs.sort_unstable();

    let middle = crabs[crabs.len() / 2];
    println!("Splitting element: {}", middle);

    crabs.iter().map(|el| (middle - el).abs()).sum()
}

pub fn task2(input_data: &[i64]) -> i64 {
    let cost = |dist| (dist * (dist + 1)) / 2;

    let input_min = *input_data.iter().min().unwrap();
    let input_max = *input_data.iter().max().unwrap();

    (input_min..=input_max)
        .map(|target| {
            input_data
                .iter()
                .map(move |&pos| cost((pos - target).abs()))
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

crate::aoc_tests! {
    task1: {
        (simple, "37")
        (complex, "345197")
    },
    task2: {
        (simple, "168")
        (complex, "96361606")
    }
}
