use ndarray::Array3;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::RangeInclusive,
};

#[derive(Debug, Clone, Copy)]
pub enum ReactorState {
    ON,
    OFF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cuboid {
    pub fn volume(&self) -> u64 {
        (self.x.end() - self.x.start() + 1) as u64
            * (self.y.end() - self.y.start() + 1) as u64
            * (self.z.end() - self.z.start() + 1) as u64
    }

    pub fn overlap(&self, action: &Cuboid) -> Option<Cuboid> {
        let x_start = max(self.x.start(), action.x.start());
        let x_end = min(self.x.end(), action.x.end());
        let y_start = max(self.y.start(), action.y.start());
        let y_end = min(self.y.end(), action.y.end());
        let z_start = max(self.z.start(), action.z.start());
        let z_end = min(self.z.end(), action.z.end());

        if x_start > x_end || y_start > y_end || z_start > z_end {
            None
        } else {
            Some(Cuboid {
                x: *x_start..=*x_end,
                y: *y_start..=*y_end,
                z: *z_start..=*z_end,
            })
        }
    }
}

pub fn parse_input(input_data: &str) -> Vec<(Cuboid, ReactorState)> {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();

    input_data
        .trim()
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();

            let x = captures[2].parse().unwrap()..=captures[3].parse().unwrap();
            let y = captures[4].parse().unwrap()..=captures[5].parse().unwrap();
            let z = captures[6].parse().unwrap()..=captures[7].parse().unwrap();

            let state = match &captures[1] {
                "on" => ReactorState::ON,
                "off" => ReactorState::OFF,
                _ => panic!("Unknown command!"),
            };

            (Cuboid { x, y, z }, state)
        })
        .collect()
}

pub fn task1(input_data: &[(Cuboid, ReactorState)]) -> u64 {
    let mut reactor = Array3::from_shape_simple_fn((101, 101, 101), || 0);

    for (_step, (command, state)) in input_data.iter().enumerate() {
        let x_start = (command.x.start() + 50).clamp(0, 101);
        let x_end = (command.x.end() + 51).clamp(0, 101);
        let y_start = (command.y.start() + 50).clamp(0, 101);
        let y_end = (command.y.end() + 51).clamp(0, 101);
        let z_start = (command.z.start() + 50).clamp(0, 101);
        let z_end = (command.z.end() + 51).clamp(0, 101);

        let mut slice =
            reactor.slice_mut(ndarray::s![x_start..x_end, y_start..y_end, z_start..z_end]);

        slice.fill(match state {
            ReactorState::ON => 1,
            ReactorState::OFF => 0,
        });

        //println!("{}: {}", _step, reactor.sum());
    }

    reactor.sum()
}

struct Reactor {
    parts: HashMap<Cuboid, i32>,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
        }
    }

    pub fn count_cells(&self) -> i64 {
        self.parts.iter().fold(0, |sum, (cub, &count)| {
            sum + cub.volume() as i64 * count as i64
        })
    }

    pub fn remove_cuboid(&mut self, cuboid: &Cuboid) {
        let overlaps = self
            .parts
            .iter()
            .filter_map(|(part, count)| part.overlap(cuboid).map(|o| (o, *count)))
            .collect::<Vec<_>>();

        for (cub, count) in overlaps {
            let part_count = self.parts.entry(cub.clone()).or_insert(0);
            *part_count -= count;
            if *part_count == 0 {
                self.parts.remove(&cub);
            }
        }

        //println!("Removing overlaps: {:?}", overlaps);
    }

    pub fn perform_action(&mut self, cuboid: &Cuboid, action: &ReactorState) {
        self.remove_cuboid(cuboid);

        if let ReactorState::ON = action {
            *self.parts.entry(cuboid.clone()).or_insert(0) += 1;
        }
    }
}

pub fn task2(input_data: &[(Cuboid, ReactorState)]) -> i64 {
    let mut reactor = Reactor::new();

    for (_step, (cuboid, action)) in input_data.iter().enumerate() {
        reactor.perform_action(cuboid, action);
        //println!("{}: {}", _step, reactor.count_cells());
    }

    reactor.count_cells()
}

crate::aoc_tests! {
    task1: {
        simple1 => 590784,
        simple2 => 474140,
        complex => 543306,
    },
    task2: {
        simple2 => 2758514936282235,
        complex => 1285501151402480,
    }
}
