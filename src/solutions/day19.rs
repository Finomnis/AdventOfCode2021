use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::helpers::nested_iterator_chain::ChainNestedIterator;

mod parser {
    use super::{Pos, Scanner};
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, newline, u64},
        combinator::map,
        multi::{many1, separated_list0},
        sequence::{delimited, terminated, tuple},
        IResult,
    };

    fn scanner_header(input: &str) -> IResult<&str, u64> {
        delimited(tag("--- scanner "), u64, tuple((tag(" ---"), newline)))(input)
    }

    fn scanner_beacon(input: &str) -> IResult<&str, Pos> {
        map(
            tuple((terminated(i32, tag(",")), terminated(i32, tag(",")), i32)),
            |(x, y, z)| Pos(x, y, z),
        )(input)
    }

    fn scanner_beacons(input: &str) -> IResult<&str, Vec<Pos>> {
        separated_list0(newline, scanner_beacon)(input)
    }

    fn scanner(input: &str) -> IResult<&str, Scanner> {
        map(tuple((scanner_header, scanner_beacons)), Scanner::new)(input)
    }

    pub fn scanners(input: &str) -> IResult<&str, Vec<Scanner>> {
        separated_list0(many1(newline), scanner)(input)
    }
}

fn get_distance_hash(p0: &Pos, p1: &Pos) -> u32 {
    (p1.0 - p0.0).abs() as u32 + (p1.1 - p0.1).abs() as u32 + (p1.2 - p0.2).abs() as u32
}

#[derive(Debug)]
pub struct Pos(i32, i32, i32);

#[derive(Debug)]
pub struct Scanner {
    number: u64,
    beacons: Vec<Pos>,
    distances_per_beacon: Vec<HashSet<u32>>,
    distances: HashSet<u32>,
}
impl Scanner {
    pub fn new((number, beacons): (u64, Vec<Pos>)) -> Self {
        let distances_per_beacon = beacons
            .iter()
            .map(|beacon| {
                beacons
                    .iter()
                    .map(|other| get_distance_hash(beacon, other))
                    .filter(|dist| *dist != 0)
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        let distances = distances_per_beacon
            .iter()
            .chain_nested_iterator(|distances| distances.iter())
            .cloned()
            .collect::<HashSet<_>>();
        Self {
            number,
            beacons,
            distances,
            distances_per_beacon,
        }
    }

    pub fn overlap_score(&self, other: &Scanner) -> usize {
        self.distances.union(&other.distances).count()
    }
}

pub fn parse_input(input_data: &str) -> Vec<Scanner> {
    let (_, scanners) = parser::scanners(input_data.trim()).unwrap();
    scanners
}

pub fn task1(scanners: &[Scanner]) -> u64 {
    for (score, scanner1, scanner2) in scanners
        .iter()
        .tuple_combinations()
        .map(|(left, right)| {
            let score = left.overlap_score(right);
            (score, left, right)
        })
        .sorted_by_key(|t| Reverse(t.0))
    {
        println!(
            "Score: {}, ({}, {})",
            score, scanner1.number, scanner2.number
        );
    }

    0
}

pub fn task2(_scanners: &[Scanner]) -> u64 {
    0
}

crate::aoc_tests! {
    task1: {
        simple => 0,
        complex => 0,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
