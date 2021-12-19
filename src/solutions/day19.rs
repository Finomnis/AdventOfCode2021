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

pub fn parse_input(input_data: &str) -> Vec<Scanner> {
    let (_, scanners) = parser::scanners(input_data.trim()).unwrap();
    scanners
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
    coords_per_axis: [Vec<i32>; 3],
    coord_delta_histo_per_axis: [HashSet<u32>; 3],
}
impl Scanner {
    pub fn new((number, beacons): (u64, Vec<Pos>)) -> Self {
        let mut coords_per_axis =
            beacons
                .iter()
                .fold([Vec::new(), Vec::new(), Vec::new()], |mut dists, beacon| {
                    dists[0].push(beacon.0);
                    dists[1].push(beacon.1);
                    dists[2].push(beacon.2);
                    dists
                });

        coords_per_axis[0].sort();
        coords_per_axis[1].sort();
        coords_per_axis[2].sort();
        coords_per_axis[0].dedup();
        coords_per_axis[1].dedup();
        coords_per_axis[2].dedup();

        let coord_delta_histo_per_axis = [
            coords_per_axis[0]
                .windows(2)
                .map(|elems| (elems[1] - elems[0]).abs() as u32)
                .collect::<HashSet<_>>(),
            coords_per_axis[1]
                .windows(2)
                .map(|elems| (elems[1] - elems[0]).abs() as u32)
                .collect::<HashSet<_>>(),
            coords_per_axis[2]
                .windows(2)
                .map(|elems| (elems[1] - elems[0]).abs() as u32)
                .collect::<HashSet<_>>(),
        ];
        Self {
            number,
            beacons,
            coords_per_axis,
            coord_delta_histo_per_axis,
        }
    }

    pub fn overlap_score(&self, other: &Scanner) -> (usize, (usize, usize, usize)) {
        [
            (0, 1, 2),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 0),
            (2, 0, 1),
            (2, 1, 0),
        ]
        .into_iter()
        .map(|order| {
            let score_0 = self.coord_delta_histo_per_axis[0]
                .union(&other.coord_delta_histo_per_axis[order.0])
                .count();
            let score_1 = self.coord_delta_histo_per_axis[1]
                .union(&other.coord_delta_histo_per_axis[order.1])
                .count();
            let score_2 = self.coord_delta_histo_per_axis[2]
                .union(&other.coord_delta_histo_per_axis[order.2])
                .count();
            (score_0 + score_1 + score_2, order)
        })
        .max_by_key(|(key, _)| *key)
        .unwrap_or((0, (0, 0, 0)))
    }
}

pub fn task1(scanners: &[Scanner]) -> u64 {
    for ((score, alignment), scanner1, scanner2) in scanners
        .iter()
        .tuple_combinations()
        .map(|(left, right)| {
            let score = left.overlap_score(right);
            (score, left, right)
        })
        .sorted_by_key(|t| Reverse(t.0))
    {
        println!(
            "Score: {}, ({}, {}) - align: ({:?})",
            score, scanner1.number, scanner2.number, alignment
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
