use std::{
    collections::{HashMap, HashSet},
    fmt,
    str::FromStr,
};

use itertools::Itertools;

use crate::helpers::input_parsing::ParseError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SignalNumber {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl fmt::Display for SignalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SignalNumber::A => 'a',
                SignalNumber::B => 'b',
                SignalNumber::C => 'c',
                SignalNumber::D => 'd',
                SignalNumber::E => 'e',
                SignalNumber::F => 'f',
                SignalNumber::G => 'g',
            }
        )
    }
}

#[derive(PartialEq)]
pub struct SignalPattern {
    signals: HashSet<SignalNumber>,
}
impl fmt::Debug for SignalPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.to_string())
    }
}

impl ToString for SignalPattern {
    fn to_string(&self) -> String {
        self.signals
            .iter()
            .map(|s| format!("{}", s))
            .sorted()
            .collect::<String>()
    }
}

impl FromStr for SignalPattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signals = s
            .chars()
            .map(|ch| match ch {
                'a' => SignalNumber::A,
                'b' => SignalNumber::B,
                'c' => SignalNumber::C,
                'd' => SignalNumber::D,
                'e' => SignalNumber::E,
                'f' => SignalNumber::F,
                'g' => SignalNumber::G,
                _ => panic!("Invalid digit '{}' encountered!", ch),
            })
            .collect();
        Ok(Self { signals })
    }
}

impl SignalPattern {
    fn len(&self) -> usize {
        self.signals.len()
    }

    fn to_digit(&self) -> i64 {
        match self.to_string().as_str() {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            other => panic!("Invalid digit '{}' encountered!", other),
        }
    }
}

#[derive(Debug, Default)]
struct SignalMapping {
    forward: HashMap<SignalNumber, SignalNumber>,
    inverse: HashMap<SignalNumber, SignalNumber>,
}

impl SignalMapping {
    fn add(&mut self, origin: SignalNumber, target: SignalNumber) {
        //println!("Inserting {}=>{} into {:?}", origin, target, self);
        let previous = self.forward.insert(origin, target);
        assert!(previous.is_none());
        let previous = self.inverse.insert(target, origin);
        assert!(previous.is_none());
    }

    fn new() -> Self {
        Self::default()
    }

    fn decode(&self, pattern: &SignalPattern) -> SignalPattern {
        SignalPattern {
            signals: pattern
                .signals
                .iter()
                .map(|s| *self.forward.get(s).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct InputLine {
    input_patterns: Vec<SignalPattern>,
    output_values: Vec<SignalPattern>,
}

impl FromStr for InputLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input_patterns, output_values) = s
            .split('|')
            .map(|a| {
                a.trim()
                    .split_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<SignalPattern>>()
            })
            .tuples()
            .next()
            .unwrap();
        Ok(Self {
            input_patterns,
            output_values,
        })
    }
}

pub fn parse_input(input_data: &str) -> Vec<InputLine> {
    input_data
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn task1(input_data: &[InputLine]) -> usize {
    input_data
        .iter()
        .map(|line| {
            line.output_values
                .iter()
                .filter(|vals| matches!(vals.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn patterns_with_length_contain(
    patterns: &[SignalPattern],
    len: usize,
    signal: SignalNumber,
) -> bool {
    patterns
        .iter()
        .filter(|pattern| pattern.len() == len)
        .any(|pattern| pattern.signals.contains(&signal))
}

fn create_decoding(patterns: &[SignalPattern]) -> SignalMapping {
    let mut signal_counter = HashMap::new();

    let mut signal_mapping = SignalMapping::new();

    // Count in how many digits the signals appear
    for pattern in patterns {
        for &digit in &pattern.signals {
            *signal_counter.entry(digit).or_insert(0usize) += 1;
        }
    }

    //println!("Patterns: {:?}", patterns);
    //println!("Counter: {:?}", signal_counter);

    // Initial round, determine all signals that are unique in the
    // amount of digits they appear in
    for (&key, value) in &signal_counter {
        match value {
            9 => signal_mapping.add(key, SignalNumber::F),
            6 => signal_mapping.add(key, SignalNumber::B),
            4 => signal_mapping.add(key, SignalNumber::E),
            8 => {
                if patterns_with_length_contain(patterns, 2, key) {
                    // Has to be C if it is part of the '1' digit
                    signal_mapping.add(key, SignalNumber::C);
                } else {
                    signal_mapping.add(key, SignalNumber::A);
                }
            }
            7 => {
                if patterns_with_length_contain(patterns, 4, key) {
                    // Has to be D if it is part of the '4' digit
                    signal_mapping.add(key, SignalNumber::D);
                } else {
                    signal_mapping.add(key, SignalNumber::G);
                }
            }
            _ => panic!(
                "Unexpected digit count: '{}' encountered {} times",
                key, value
            ),
        }
    }

    //println!("{:?}", signal_mapping);
    signal_mapping
}

pub fn task2(input_data: &[InputLine]) -> i64 {
    input_data
        .iter()
        .map(|line| {
            let decoder = create_decoding(&line.input_patterns);
            line.output_values
                .iter()
                .map(|o| decoder.decode(o))
                .map(|p| p.to_digit())
                .fold(0, |res, digit| res * 10 + digit)
        })
        .sum()
}

crate::aoc_tests! {
    task1: {
        (simple, "26")
        (complex, "534")
    },
    task2: {
        (simple, "61229")
        (complex, "1070188")
    }
}
