use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

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
impl Pos {
    pub fn rotate(&self, rot: &Rotation) -> Pos {
        let p = [self.0, self.1, self.2];
        let mut p_mapped = [
            p[rot.mapping.0 as usize],
            p[rot.mapping.1 as usize],
            p[rot.mapping.2 as usize],
        ];
        if rot.flips.0 {
            p_mapped[0] *= -1;
        }
        if rot.flips.1 {
            p_mapped[1] *= -1;
        }
        if rot.flips.2 {
            p_mapped[2] *= -1;
        }
        Pos(p_mapped[0], p_mapped[1], p_mapped[2])
    }
    pub fn apply_offset(&self, offset: &Offset) -> Pos {
        Pos(self.0 + offset.0, self.1 + offset.1, self.2 + offset.2)
    }
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pos: Pos,
    neighbor_hash: Option<u64>,
}

#[derive(Debug)]
pub struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
    known_beacon_hashes: HashSet<u64>,
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
                        .filter_map(|other| distance_hash(pos, other))
                        .filter(|&dist| dist != 0)
                        .collect::<Vec<_>>();

                    if neighbor_distances.len() < 2 {
                        None
                    } else {
                        neighbor_distances.sort_unstable();

                        let mut hasher = DefaultHasher::new();
                        for distance in &neighbor_distances {
                            hasher.write_u64(*distance);
                        }
                        let hash = hasher.finish();
                        //println!("Hash: {} => {:?}", hash, neighbor_distances);
                        Some(hash)
                    }
                } else {
                    None
                };

                Beacon {
                    pos: pos.clone(),
                    neighbor_hash,
                }
            })
            .collect::<Vec<_>>();

        let known_beacon_hashes = beacons
            .iter()
            .filter_map(|beacon| beacon.neighbor_hash)
            .collect::<HashSet<_>>();
        Self {
            id,
            beacons,
            known_beacon_hashes,
        }
    }
}

const POSSIBLE_BASE_ROTATIONS: [((u8, u8, u8), bool); 6] = [
    ((0, 1, 2), false),
    ((0, 2, 1), true),
    ((1, 0, 2), true),
    ((1, 2, 0), false),
    ((2, 0, 1), false),
    ((2, 1, 0), true),
];

const POSSIBLE_ROTATION_FLIPS: [(bool, bool, bool); 4] = [
    (false, false, false),
    (true, true, false),
    (true, false, true),
    (false, true, true),
];

fn possible_rotations() -> impl Iterator<Item = Rotation> {
    POSSIBLE_BASE_ROTATIONS
        .iter()
        .flat_map(|&(base_rot, base_flip)| {
            POSSIBLE_ROTATION_FLIPS.iter().map(move |flip| Rotation {
                mapping: base_rot,
                flips: (base_flip ^ flip.0, base_flip ^ flip.1, base_flip ^ flip.2),
            })
        })
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Offset(i32, i32, i32);

impl Offset {
    pub fn from_start_to_target(start: &Pos, target: &Pos) -> Self {
        Self(target.0 - start.0, target.1 - start.1, target.2 - start.2)
    }
}

#[derive(Debug)]
pub struct Rotation {
    mapping: (u8, u8, u8),
    flips: (bool, bool, bool),
}
pub fn find_rotation_and_offset(known: &[Beacon], other: &[Beacon]) -> Option<(Offset, Rotation)> {
    let points_known = known
        .iter()
        .filter_map(|b| b.neighbor_hash.map(|hash| (hash, b.clone())))
        .collect::<HashMap<_, _>>();
    let points_other = other
        .iter()
        .filter_map(|b| b.neighbor_hash.map(|hash| (hash, b.clone())))
        .collect::<HashMap<_, _>>();
    let hashes = points_known
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&points_other.keys().collect::<HashSet<_>>())
        .cloned()
        .cloned()
        .collect::<HashSet<_>>();

    if hashes.len() < 3 {
        println!("Not enough overlap! This heurestic algorithm is fast, but needs enough overlap to function correctly.");
        return None;
    }
    let (score, rotation, offset) = possible_rotations()
        .map(|rot| {
            // Map all beacons to correct orientation
            let beacons = hashes
                .iter()
                .map(|hash| points_other[hash].clone())
                .map(|mut beacon| {
                    beacon.pos = beacon.pos.rotate(&rot);
                    beacon
                })
                .collect::<Vec<_>>();

            let scores = beacons
                .iter()
                .map(|start_beacon| {
                    let offset = Offset::from_start_to_target(
                        &start_beacon.pos,
                        &points_known[&start_beacon.neighbor_hash.unwrap()].pos,
                    );

                    let score = beacons
                        .iter()
                        .cloned()
                        .filter(|beacon| {
                            let pos = beacon.pos.apply_offset(&offset);
                            pos == points_known[&beacon.neighbor_hash.unwrap()].pos
                        })
                        .count();

                    (offset, score)
                })
                .into_group_map();

            let (offset, score) = scores
                .iter()
                .map(|(key, value)| (key.clone(), value.iter().sum::<usize>()))
                .max_by_key(|(_, key)| *key)
                .unwrap();

            (score, rot, offset)
        })
        .max_by_key(|(score, _, _)| *score)
        .unwrap();

    if score > 1 {
        Some((offset, rotation))
    } else {
        None
    }
}

pub fn get_all_beacons_and_scanners(scanners: &[Scanner]) -> (Vec<Pos>, Vec<Pos>) {
    /*
    let mut hashed_beacons = HashMap::new();

    for scanner in scanners {
        //println!("SCANNER: {}", scanner.id);
        for beacon in &scanner.beacons {
            //println!("  - {:?}: {:?}", beacon.pos, beacon.neighbor_hash);

            if let Some(hash) = beacon.neighbor_hash {
                hashed_beacons
                    .entry(hash)
                    .or_insert_with(HashSet::new)
                    .insert(scanner.id);
            }
        }
        //println!();
    }

    //println!("{:?}", hashed_beacons);
    */

    let mut unknown_scanners = (1..scanners.len()).collect::<HashSet<_>>();
    let mut known_beacon_hashes = scanners[0]
        .beacons
        .iter()
        .filter_map(|beacon| beacon.neighbor_hash)
        .collect::<HashSet<_>>();

    let mut known_beacons = scanners[0].beacons.clone();

    let mut scanner_positions = vec![Pos(0, 0, 0)];

    while !unknown_scanners.is_empty() {
        let (_count, scanner) = scanners
            .iter()
            .filter(|s| unknown_scanners.contains(&s.id))
            .map(|scanner| {
                let overlap = known_beacon_hashes.intersection(&scanner.known_beacon_hashes);
                let overlap_count = overlap.count();
                //println!("Overlap to {}: {}", scanner.id, overlap_count);
                (overlap_count, scanner)
            })
            .max_by_key(|(key, _)| *key)
            .unwrap();

        //println!("Chosen scanner {} with {} overlaps.", scanner.id, count);

        let (offset, rotation) =
            find_rotation_and_offset(&known_beacons, &scanner.beacons).unwrap();

        scanner_positions.push(Pos(offset.0, offset.1, offset.2));

        //println!("Found rotation & offset: {:?} {:?}", rotation, offset);

        // Add new points to map
        for mut beacon in scanner.beacons.iter().cloned() {
            beacon.pos = beacon.pos.rotate(&rotation).apply_offset(&offset);
            if let Some(existing_beacon) = known_beacons.iter_mut().find(|b| b.pos == beacon.pos) {
                //println!("Beacon already exists, updating hash if necessary ...");
                if existing_beacon.neighbor_hash.is_none() {
                    existing_beacon.neighbor_hash = beacon.neighbor_hash;
                }
            } else {
                known_beacons.push(beacon);
            }
        }

        known_beacon_hashes.extend(scanner.known_beacon_hashes.iter());
        unknown_scanners.remove(&scanner.id);
    }

    (
        known_beacons.iter().map(|b| b.pos.clone()).collect(),
        scanner_positions,
    )
}

pub fn task1(scanners: &[Scanner]) -> usize {
    let (beacons, _scanners) = get_all_beacons_and_scanners(scanners);
    beacons.len()
}

pub fn task2(scanners: &[Scanner]) -> u32 {
    let (_beacons, scanners) = get_all_beacons_and_scanners(scanners);

    // for scanner in &scanners {
    //     println!("{},{},{}", scanner.0, scanner.1, scanner.2);
    // }

    let result = scanners
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let dist =
                (a.0 - b.0).abs() as u32 + (a.1 - b.1).abs() as u32 + (a.2 - b.2).abs() as u32;
            //println!("Dist {}: {:?} {:?}", dist, a, b);
            (a.clone(), b.clone(), dist)
        })
        .max_by_key(|(_, _, dist)| *dist)
        .unwrap();

    //println!("{:?}", result);
    result.2
}

crate::aoc_tests! {
    task1: {
        simple => 79,
        complex => 378,
    },
    task2: {
        simple => 3621,
        complex => 13148,
    }
}
