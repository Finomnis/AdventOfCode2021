use std::{
    convert::Infallible,
    fmt::{Display, Write},
    str::FromStr,
};

use ndarray::Array2;

use crate::helpers::input_parsing::parse_as_2d_matrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorTile {
    MoveRight,
    MoveBottom,
    Empty,
}

impl Display for FloorTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            FloorTile::MoveRight => '>',
            FloorTile::MoveBottom => 'v',
            FloorTile::Empty => '.',
        })
    }
}

impl FromStr for FloorTile {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Self::MoveRight,
            "v" => Self::MoveBottom,
            _ => Self::Empty,
        })
    }
}

pub fn parse_input(input_data: &str) -> Array2<FloorTile> {
    parse_as_2d_matrix(input_data).unwrap()
}

pub fn move_cucumbers<F>(seafloor: &mut Array2<FloorTile>, tile: FloorTile, target_coord: F) -> bool
where
    F: Fn((usize, usize)) -> (usize, usize),
{
    let moving_cucumbers = seafloor
        .indexed_iter()
        .filter(|(_, &floor_tile)| floor_tile == tile)
        .map(|(start, _)| (start, target_coord(start)))
        .filter(|(_, target)| matches!(seafloor.get(*target), Some(FloorTile::Empty)))
        .collect::<Vec<_>>();

    let any_moving = !moving_cucumbers.is_empty();

    for (start, target) in moving_cucumbers {
        seafloor[start] = FloorTile::Empty;
        seafloor[target] = tile;
    }

    any_moving
}

pub fn move_right(seafloor: &mut Array2<FloorTile>) -> bool {
    let dim = seafloor.dim();
    move_cucumbers(seafloor, FloorTile::MoveRight, |(y, x)| {
        (y, (x + 1) % dim.1)
    })
}

pub fn move_bottom(seafloor: &mut Array2<FloorTile>) -> bool {
    let dim = seafloor.dim();
    move_cucumbers(seafloor, FloorTile::MoveBottom, |(y, x)| {
        ((y + 1) % dim.0, x)
    })
}

pub fn task1(input_data: &Array2<FloorTile>) -> u32 {
    let mut seafloor = input_data.clone();

    let mut iterations = 1;
    while {
        let moved_right = move_right(&mut seafloor);
        let moved_bottom = move_bottom(&mut seafloor);
        moved_right || moved_bottom
    } {
        iterations += 1;
    }

    iterations
}

crate::aoc_tests! {
    task1: {
        simple => 58,
        complex => 549,
    }
}
