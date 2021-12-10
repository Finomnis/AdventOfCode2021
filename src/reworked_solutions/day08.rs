use std::collections::HashMap;

pub fn parse_input(input_data: &str) -> String {
    input_data.to_string()
}

pub fn task1(input_data: &str) -> usize {
    input_data
        .trim()
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|vals| matches!(vals.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

pub fn decode(input_line: &str) -> i64 {
    let (digits, encoded) = input_line.split_once("|").unwrap();

    let digits_histogram = digits
        .split_whitespace()
        .fold(HashMap::new(), |hist, digit| {
            digit.chars().fold(hist, |mut hist, ch| {
                *hist.entry(ch).or_insert(0i64) += 1;
                hist
            })
        });

    encoded
        .split_whitespace()
        .map(|number| number.chars().map(|ch| digits_histogram[&ch]).sum())
        .map(|digit_identifier| match digit_identifier {
            42 => 0,
            17 => 1,
            34 => 2,
            39 => 3,
            30 => 4,
            37 => 5,
            41 => 6,
            25 => 7,
            49 => 8,
            45 => 9,
            i => panic!("Unknown identifier {}!", i),
        })
        .fold(0, |sum, digit| sum * 10 + digit)
}

pub fn task2(input_data: &str) -> i64 {
    input_data.trim().lines().map(decode).sum()
}

crate::aoc_tests! {
    task1: {
        simple => 26,
        complex => 534,
    },
    task2: {
        simple => 61229,
        complex => 1070188,
    }
}
