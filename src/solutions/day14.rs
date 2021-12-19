use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct PuzzleInput {
    start: String,
    rules: HashMap<(char, char), char>,
}

pub fn parse_input(input_data: &str) -> PuzzleInput {
    let mut lines = input_data.trim().lines();

    let start = lines.next().unwrap().to_string();
    assert_eq!("", lines.next().unwrap());

    let re = Regex::new(r"^(\S)(\S) -> (\S)$").unwrap();

    let rules = lines
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                (
                    captures[1].chars().next().unwrap(),
                    captures[2].chars().next().unwrap(),
                ),
                captures[3].chars().next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    PuzzleInput { start, rules }
}

pub fn task1(input_data: &PuzzleInput) -> usize {
    let mut polymer = input_data.start.clone();

    for _ in 0..10 {
        polymer = polymer
            .chars()
            .zip(polymer.chars().skip(1).chain([' '].into_iter()))
            .map(|(left, right)| match input_data.rules.get(&(left, right)) {
                Some(middle) => left.to_string() + &middle.to_string(),
                None => left.to_string(),
            })
            .collect::<String>();
        //println!("{}", polymer);
    }

    // Count items
    let hist = polymer.chars().counts();

    let (min, max) = hist
        .iter()
        .map(|(_, &count)| count)
        .minmax()
        .into_option()
        .unwrap();

    max - min
}

pub fn task2(input_data: &PuzzleInput) -> usize {
    let mut pair_histo = input_data
        .start
        .chars()
        .zip(input_data.start.chars().skip(1))
        .counts();

    let mut letter_histo = input_data.start.chars().counts();

    for _ in 0..40 {
        pair_histo = pair_histo
            .iter()
            .flat_map(
                |(&(left, right), &count)| match input_data.rules.get(&(left, right)) {
                    Some(&middle) => {
                        *letter_histo.entry(middle).or_insert(0) += count;
                        vec![((left, middle), count), ((middle, right), count)].into_iter()
                    }
                    None => vec![((left, right), count)].into_iter(),
                },
            )
            .into_grouping_map()
            .sum();

        //println!();
        //println!("  Pairs: {:?}", pair_histo);
        //println!("  Letters: {:?}", letter_histo);
    }

    let (min, max) = letter_histo
        .iter()
        .map(|(_, &count)| count)
        .minmax()
        .into_option()
        .unwrap();

    max - min
}

crate::aoc_tests! {
    task1: {
        simple => 1588,
        complex => 2988,
    },
    task2: {
        simple => 2188189693529,
        complex => 3572761917024,
    }
}
