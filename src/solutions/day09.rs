use std::collections::HashSet;
use std::ops::Mul;

use ndarray::Array2;

use crate::helpers::input_parsing::{self, parse_as_2d_matrix};

pub fn parse_input(input_data: &str) -> String {
    input_data.to_string()
}

pub fn is_larger<T: PartialOrd>(larger: Option<T>, smaller: Option<T>) -> Option<bool> {
    if let (Some(l), Some(s)) = (larger, smaller) {
        Some(l > s)
    } else {
        None
    }
}

pub fn task1(input_data: &str) -> i64 {
    let map = input_parsing::parse_as_2d_matrix_with_border::<i64>(input_data, 1).unwrap();

    map.windows((3, 3))
        .into_iter()
        .map(|window| {
            let (top, left, center, right, bottom) = (
                window[(0, 1)],
                window[(1, 0)],
                window[(1, 1)],
                window[(1, 2)],
                window[(2, 1)],
            );

            if matches!(is_larger(top, center), Some(false))
                || matches!(is_larger(left, center), Some(false))
                || matches!(is_larger(right, center), Some(false))
                || matches!(is_larger(bottom, center), Some(false))
            {
                0
            } else {
                match center {
                    Some(c) => c + 1,
                    None => 0,
                }
            }
        })
        .sum()
}

pub fn is_minimum((height, width): (usize, usize), map: &Array2<i64>) -> bool {
    if let Some(center) = map.get((height, width)) {
        !(matches!(
            is_larger(Some(center), map.get((height, width + 1))),
            Some(true)
        ) || matches!(
            is_larger(Some(center), map.get((height + 1, width))),
            Some(true)
        ) || (height > 0
            && matches!(
                is_larger(Some(center), map.get((height - 1, width))),
                Some(true)
            ))
            || (width > 0
                && matches!(
                    is_larger(Some(center), map.get((height, width - 1))),
                    Some(true)
                )))
    } else {
        false
    }
}

fn flood_fill(map: &Array2<i64>, basin: &mut HashSet<(usize, usize)>, index: (usize, usize)) {
    if let Some(&field) = map.get(index) {
        if field == 9 {
            return;
        }

        if !basin.insert(index) {
            return;
        }

        flood_fill(map, basin, (index.0 + 1, index.1));
        flood_fill(map, basin, (index.0, index.1 + 1));

        if index.0 > 0 {
            flood_fill(map, basin, (index.0 - 1, index.1));
        }
        if index.1 > 0 {
            flood_fill(map, basin, (index.0, index.1 - 1));
        }
    }
}

pub fn task2(input_data: &str) -> usize {
    let map = parse_as_2d_matrix::<i64>(input_data).unwrap();

    //println!("{:?}", map);
    let mut basins = vec![];

    for (index, _elem) in map.indexed_iter() {
        if is_minimum(index, &map) {
            // Perform flood fill
            let mut basin = HashSet::new();

            flood_fill(&map, &mut basin, index);

            //println!("{:?} => {}: {:?}", index, basin.len(), basin);

            basins.push((index, basin.len()));
        }
    }

    basins.sort_by_key(|(_, area)| std::cmp::Reverse(*area));

    basins
        .iter()
        .take(3)
        .map(|(_, area)| *area)
        .reduce(usize::mul)
        .unwrap()
}

crate::aoc_tests! {
    task1: {
        (simple, "day09_simple.txt", "15")
        (complex, "day09_complex.txt", "535")
    },
    task2: {
        (simple, "day09_simple.txt", "1134")
        (complex, "day09_complex.txt", "1122700")
    }
}
