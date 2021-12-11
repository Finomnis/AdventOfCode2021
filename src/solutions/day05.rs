use std::cmp::{max, min};
use std::{fmt, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::helpers::input_parsing::ParseError;

#[derive(Debug, Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct VentLine {
    start: Coord,
    end: Coord,
}

impl FromStr for VentLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }
        let captures = RE.captures(s).unwrap();

        Ok(Self {
            start: Coord {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            },
            end: Coord {
                x: captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap(),
            },
        })
    }
}

pub fn parse_input(input_data: &str) -> Vec<VentLine> {
    input_data
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub struct VentMap {
    data: Vec<i64>,
    pub width: usize,
    pub height: usize,
}

impl VentMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }
    pub fn new_auto_bounds(lines: &[VentLine]) -> Self {
        let (width, height) = lines.iter().fold((0, 0), |(x, y), line| {
            (
                max(max(x, line.start.x + 1), line.end.x + 1),
                max(max(y, line.start.y + 1), line.end.y + 1),
            )
        });
        Self::new(width, height)
    }
}

impl fmt::Display for VentMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (index, &data) in self.data.iter().enumerate() {
            if data == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "{}", data)?;
            }
            if index % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl VentMap {
    fn get_mut(&mut self, x: usize, y: usize) -> &mut i64 {
        &mut self.data[self.width * y + x]
    }

    pub fn get(&self, x: usize, y: usize) -> i64 {
        self.data[self.width * y + x]
    }

    pub fn render_straight_line(&mut self, line: &VentLine) {
        if line.start.x == line.end.x {
            let start = min(line.start.y, line.end.y);
            let end = max(line.start.y, line.end.y);

            let x = line.start.x;
            for y in start..=end {
                *self.get_mut(x, y) += 1;
            }
        } else if line.start.y == line.end.y {
            let start = min(line.start.x, line.end.x);
            let end = max(line.start.x, line.end.x);

            let y = line.start.y;
            for x in start..=end {
                *self.get_mut(x, y) += 1;
            }
        }
    }

    pub fn render_line(&mut self, line: &VentLine) {
        if line.start.x == line.end.x || line.start.y == line.end.y {
            self.render_straight_line(line);
        } else {
            self.render_diagonal_line(line);
        }
    }

    fn render_diagonal_line(&mut self, line: &VentLine) {
        if line.start.x > line.end.x {
            let flipped_line = VentLine {
                start: line.end.clone(),
                end: line.start.clone(),
            };
            self.render_diagonal_line(&flipped_line);
            return;
        }
        let start = line.start.x;
        let end = line.end.x;
        let direction = line.end.y >= line.start.y;

        for x in start..=end {
            let y = if direction {
                line.start.y + x - start
            } else {
                line.start.y - (x - start)
            };
            *self.get_mut(x, y) += 1;
        }
    }
}

pub fn task1(input_data: &[VentLine]) -> usize {
    let mut vent_map = VentMap::new_auto_bounds(input_data);

    for line in input_data {
        vent_map.render_straight_line(line);
    }

    //println!("VentMap: \n{}", vent_map);

    vent_map.data.iter().filter(|&&val| val > 1).count()
}

pub fn task2(input_data: &[VentLine]) -> usize {
    let mut vent_map = VentMap::new_auto_bounds(input_data);

    for line in input_data {
        vent_map.render_line(line);
    }

    //println!("VentMap: \n{}", vent_map);

    vent_map.data.iter().filter(|&&val| val > 1).count()
}

crate::aoc_tests! {
    task1: {
        simple => 5,
        complex => 5585,
    },
    task2: {
        simple => 12,
        complex => 17193,
    }
}
