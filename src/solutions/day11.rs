use std::collections::VecDeque;

use itertools::Itertools;
use ndarray::{Array2, Axis};

use crate::helpers::input_parsing::parse_as_2d_matrix;

pub fn parse_input(input_data: &str) -> Array2<u8> {
    parse_as_2d_matrix::<u8>(input_data).unwrap()
}

#[allow(dead_code)]
fn format_map(map: &Array2<u8>) -> String {
    map.axis_iter(Axis(0))
        .map(|row| row.iter().map(u8::to_string).collect::<String>())
        .join("\n")
}

pub fn update_map(map: &mut Array2<u8>) -> usize {
    let mut num_flashes = 0;

    // Part 1: increase everything by 1
    map.iter_mut().for_each(|el| {
        *el += 1;
    });

    // Part 2: Flash
    let mut need_flash = map
        .indexed_iter()
        .filter_map(|(index, &value)| if value > 9 { Some(index) } else { None })
        .collect::<VecDeque<_>>();

    while let Some((y, x)) = need_flash.pop_front() {
        num_flashes += 1;

        let mut flash = |y: Option<usize>, x: Option<usize>| {
            if let (Some(y), Some(x)) = (y, x) {
                if let Some(cell) = map.get_mut((y, x)) {
                    *cell += 1;
                    if *cell == 10 {
                        need_flash.push_back((y, x));
                    }
                }
            }
        };

        flash(y.checked_sub(1), x.checked_sub(1));
        flash(y.checked_sub(1), Some(x));
        flash(y.checked_sub(1), x.checked_add(1));
        flash(Some(y), x.checked_sub(1));
        flash(Some(y), x.checked_add(1));
        flash(y.checked_add(1), x.checked_sub(1));
        flash(y.checked_add(1), Some(x));
        flash(y.checked_add(1), x.checked_add(1));
    }

    // Part 3: reduce all flashed to 0
    map.iter_mut().for_each(|el| {
        if *el > 9 {
            *el = 0;
        }
    });

    num_flashes
}

pub fn task1(input_data: &Array2<u8>) -> usize {
    let mut map = input_data.clone();

    let mut num_flashes = 0;

    // println!("Initial conditions:\n{}\n", format_map(&map));

    for _step in 1..=100 {
        num_flashes += update_map(&mut map);
        // println!("After step {}:\n{}\n", step, format_map(&map));
    }

    num_flashes
}

pub fn task2(input_data: &Array2<u8>) -> usize {
    let mut map = input_data.clone();

    let mut num_cycles = 1;

    // At the point of synchronization, all octopuses flash at once
    while update_map(&mut map) != map.len() {
        num_cycles += 1;
    }

    num_cycles
}

crate::aoc_tests! {
    task1: {
        simple => 1656,
        complex => 1588,
    },
    task2: {
        simple => 195,
        complex => 517,
    }
}
