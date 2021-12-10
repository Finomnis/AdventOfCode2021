pub fn parse_input(input_data: &str) -> String {
    input_data.to_string()
}

fn closing_char(opening: char) -> Option<char> {
    match opening {
        '{' => Some('}'),
        '<' => Some('>'),
        '[' => Some(']'),
        '(' => Some(')'),
        ']' | ')' | '>' | '}' => None,
        c => panic!("Invalid char '{}' encountered!", c),
    }
}

enum AnalyzerResult {
    Invalid(char),
    LeftOver(String),
    Good,
}

fn analyze_line(input_data: &str) -> AnalyzerResult {
    let mut expected = vec![];
    for char in input_data.chars() {
        match closing_char(char) {
            Some(closing) => {
                expected.push(closing);
                continue;
            }
            None => {
                if let Some(expected_closing) = expected.pop() {
                    if char == expected_closing {
                        continue;
                    }
                }
            }
        };

        // println!("{} - Found invalid symbol \"{}\"", input_data, char);
        return AnalyzerResult::Invalid(char);
    }

    if expected.is_empty() {
        AnalyzerResult::Good
    } else {
        let completion = expected.into_iter().rev().collect::<String>();
        // println!("{} - Complete by adding \"{}\"", input_data, completion);
        AnalyzerResult::LeftOver(completion)
    }
}

pub fn task1(input_data: &str) -> i64 {
    input_data
        .trim()
        .lines()
        .map(|s| match analyze_line(s) {
            AnalyzerResult::Invalid(')') => 3,
            AnalyzerResult::Invalid(']') => 57,
            AnalyzerResult::Invalid('}') => 1197,
            AnalyzerResult::Invalid('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn task2_get_score(line: &str) -> Option<i64> {
    match analyze_line(line) {
        AnalyzerResult::LeftOver(leftover) => Some(
            leftover
                .chars()
                .map(|ch| match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
                .fold(0, |res, val| res * 5 + val),
        ),
        _ => None,
    }
}

pub fn task2(input_data: &str) -> i64 {
    let mut output_data = input_data
        .trim()
        .lines()
        .filter_map(task2_get_score)
        .collect::<Vec<_>>();

    output_data.sort_unstable();
    output_data[output_data.len() / 2]
}

crate::aoc_tests! {
    task1: {
        (simple, "day10_simple.txt", "26397")
        (complex, "day10_complex.txt", "390993")
    },
    task2: {
        (simple, "day10_simple.txt", "288957")
        (complex, "day10_complex.txt", "2391385187")
    }
}
