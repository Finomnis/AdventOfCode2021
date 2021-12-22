use ndarray::Array3;
use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum Command {
    ON,
    OFF,
}

#[derive(Debug)]
pub struct CuboidAction {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    command: Command,
}

pub fn parse_input(input_data: &str) -> Vec<CuboidAction> {
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

            let command = match &captures[1] {
                "on" => Command::ON,
                "off" => Command::OFF,
                _ => panic!("Unknown command!"),
            };

            CuboidAction { x, y, z, command }
        })
        .collect()
}

pub fn task1(input_data: &[CuboidAction]) -> u64 {
    let mut reactor = Array3::from_shape_simple_fn((101, 101, 101), || 0);

    for command in input_data {
        let x_start = (command.x.start() + 50).clamp(0, 101);
        let x_end = (command.x.end() + 51).clamp(0, 101);
        let y_start = (command.y.start() + 50).clamp(0, 101);
        let y_end = (command.y.end() + 51).clamp(0, 101);
        let z_start = (command.z.start() + 50).clamp(0, 101);
        let z_end = (command.z.end() + 51).clamp(0, 101);

        let mut slice =
            reactor.slice_mut(ndarray::s![x_start..x_end, y_start..y_end, z_start..z_end]);

        slice.fill(match command.command {
            Command::ON => 1,
            Command::OFF => 0,
        });
    }

    reactor.sum()
}

pub fn task2(_input_data: &[CuboidAction]) -> u64 {
    0
}

crate::aoc_tests! {
    task1: {
        simple => 590784,
        complex => 543306,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
