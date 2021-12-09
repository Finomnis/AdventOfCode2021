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
        if matches!(
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
                ))
        {
            false
        } else {
            true
        }
    } else {
        false
    }
}

pub fn task2(input_data: &str) -> i64 {
    let map = parse_as_2d_matrix::<i64>(input_data).unwrap();

    for (index, _elem) in map.indexed_iter() {
        if is_minimum(index, &map) {
            println!("min: {:?}", index);
        }
    }

    0
}

crate::aoc_tests! {
    task1: {
        (simple, "day09_simple.txt", "15")
        (complex, "day09_complex.txt", "535")
    },
    task2: {
        (simple, "day09_simple.txt", "")
        (complex, "day09_complex.txt", "")
    }
}
