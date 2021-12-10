use crate::helpers::input_parsing::ParseError;
use std::str::FromStr;

pub enum Direction {
    Forward,
    Up,
    Down,
}

pub struct DriveCommand {
    direction: Direction,
    distance: i64,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(ParseError("Invalid direction".to_string())),
        }
    }
}

impl FromStr for DriveCommand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = s.split_whitespace();
        let direction = args
            .next()
            .ok_or_else(|| ParseError("Not enough arguments in line".to_string()))?
            .parse()?;
        let distance = args
            .next()
            .ok_or_else(|| ParseError("Not enough arguments in line".to_string()))?
            .parse()
            .map_err(|_| ParseError("Unable to parse distance".to_string()))?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

pub fn parse_input(input_data: &str) -> Vec<DriveCommand> {
    input_data
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

struct Position {
    horizontal: i64,
    depth: i64,
}

impl Position {
    fn new(horizontal: i64, depth: i64) -> Self {
        Self { horizontal, depth }
    }
}

struct AimPosition {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl AimPosition {
    fn new(horizontal: i64, depth: i64, aim: i64) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }
}

pub fn task1(input_data: &[DriveCommand]) -> i64 {
    let position = input_data
        .iter()
        .fold(Position::new(0, 0), |pos, elem| match elem.direction {
            Direction::Forward => Position::new(pos.horizontal + elem.distance, pos.depth),
            Direction::Up => Position::new(pos.horizontal, pos.depth - elem.distance),
            Direction::Down => Position::new(pos.horizontal, pos.depth + elem.distance),
        });
    position.depth * position.horizontal
}

pub fn task2(input_data: &[DriveCommand]) -> i64 {
    let position = input_data
        .iter()
        .fold(AimPosition::new(0, 0, 0), |pos, elem| {
            match elem.direction {
                Direction::Forward => AimPosition::new(
                    pos.horizontal + elem.distance,
                    pos.depth + pos.aim * elem.distance,
                    pos.aim,
                ),
                Direction::Up => {
                    AimPosition::new(pos.horizontal, pos.depth, pos.aim - elem.distance)
                }
                Direction::Down => {
                    AimPosition::new(pos.horizontal, pos.depth, pos.aim + elem.distance)
                }
            }
        });
    position.depth * position.horizontal
}

crate::aoc_tests! {
    task1: {
        (simple, "150")
        (complex, "1938402")
    },
    task2: {
        (simple, "900")
        (complex, "1947878632")
    }
}
