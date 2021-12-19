use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
};

const SCANNER_RANGE: u32 = 1000;
const LOCAL_NEIGHBOR_RANGE: u32 = 200;

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

    fn scanner_header(input: &str) -> IResult<&str, usize> {
        delimited(
            tag("--- scanner "),
            map(u64, |id| id as usize),
            tuple((tag(" ---"), newline)),
        )(input)
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Pos(i32, i32, i32);

#[derive(Debug, Clone)]
pub struct Beacon {
    pos: Pos,
    seen_by: HashSet<usize>,
    neighbor_hash: Option<u64>,
}

#[derive(Debug)]
pub struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

fn distance_hash(p0: &Pos, p1: &Pos) -> Option<u64> {
    let d0 = (p0.0 - p1.0).abs() as u32;
    let d1 = (p0.1 - p1.1).abs() as u32;
    let d2 = (p0.2 - p1.2).abs() as u32;
    if d0 < LOCAL_NEIGHBOR_RANGE && d1 < LOCAL_NEIGHBOR_RANGE && d2 < LOCAL_NEIGHBOR_RANGE {
        let d0 = d0 as u64;
        let d1 = d1 as u64;
        let d2 = d2 as u64;
        Some(d0 * d0 + d1 * d1 + d2 * d2)
    } else {
        None
    }
}

impl Scanner {
    pub fn new((id, beacons): (usize, Vec<Pos>)) -> Self {
        let beacons = beacons
            .iter()
            .map(|pos| {
                let neighbor_hash = if pos.0.abs() as u32 + LOCAL_NEIGHBOR_RANGE < SCANNER_RANGE
                    && pos.1.abs() as u32 + LOCAL_NEIGHBOR_RANGE < SCANNER_RANGE
                    && pos.2.abs() as u32 + LOCAL_NEIGHBOR_RANGE < SCANNER_RANGE
                {
                    let mut neighbor_distances = beacons
                        .iter()
                        .filter_map(|other| distance_hash(&pos, other))
                        .filter(|&dist| dist != 0)
                        .collect::<Vec<_>>();

                    if neighbor_distances.len() < 2 {
                        None
                    } else {
                        neighbor_distances.sort();

                        let mut hasher = DefaultHasher::new();
                        for distance in &neighbor_distances {
                            hasher.write_u64(*distance);
                        }
                        let hash = hasher.finish();
                        println!("Hash: {} => {:?}", hash, neighbor_distances);
                        Some(hash)
                    }
                } else {
                    None
                };

                Beacon {
                    pos: pos.clone(),
                    seen_by: HashSet::from([id]),
                    neighbor_hash,
                }
            })
            .collect::<Vec<_>>();
        Self { id, beacons }
    }
}

pub fn task1(scanners: &[Scanner]) -> u64 {
    let mut potential_mappings = HashMap::new();

    for scanner in scanners {
        println!("SCANNER: {}", scanner.id);
        for beacon in &scanner.beacons {
            println!("  - {:?}: {:?}", beacon.pos, beacon.neighbor_hash);

            if let Some(hash) = beacon.neighbor_hash {
                potential_mappings
                    .entry(hash)
                    .or_insert_with(|| HashSet::new())
                    .insert(scanner.id);
            }
        }
        println!();
    }

    println!("{:?}", potential_mappings);

    let mut unknown_scanners = (1..scanners.len()).collect::<HashSet<_>>();

    while !unknown_scanners.is_empty() {
        return 0;
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
