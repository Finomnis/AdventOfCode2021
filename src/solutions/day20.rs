use itertools::Itertools;
use ndarray::Array2;

use crate::helpers::{
    image_manipulation::conv2d, input_parsing::parse_as_2d_matrix_with_filled_border,
};

#[derive(Debug)]
pub struct PuzzleInput {
    enhancement_lookup: Vec<char>,
    image: Array2<char>,
}

pub fn parse_input(input_data: &str) -> PuzzleInput {
    let mut lines = input_data.trim().lines();

    let enhancement_lookup = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next().unwrap();

    let image = parse_as_2d_matrix_with_filled_border(&lines.join("\n"), 100, '.').unwrap();

    PuzzleInput {
        enhancement_lookup,
        image,
    }
}

pub fn enhance(image: &Array2<char>, enhancement_lookup: &[char]) -> Array2<char> {
    conv2d(image, (3, 3), |window| {
        let lookup_id = window
            .iter()
            .fold(0, |num, &elem| num * 2 + (elem == '#') as usize);
        enhancement_lookup[lookup_id]
    })
}

pub fn task1(input_data: &PuzzleInput) -> usize {
    let image = input_data.image.clone();
    let enhancement_lookup = &input_data.enhancement_lookup;
    println!("{:?}", image);
    let image = enhance(&image, enhancement_lookup);
    println!("{:?}", image);
    let image = enhance(&image, enhancement_lookup);
    println!("{:?}", image);

    image.iter().filter(|&el| *el == '#').count()
}

pub fn task2(input_data: &PuzzleInput) -> usize {
    let mut image = input_data.image.clone();
    let enhancement_lookup = &input_data.enhancement_lookup;

    for _ in 0..50 {
        image = enhance(&image, enhancement_lookup);
    }

    image.iter().filter(|&el| *el == '#').count()
}

crate::aoc_tests! {
    task1: {
        simple => 35,
        complex => 5225,
    },
    task2: {
        simple => 3351,
        complex => 18131,
    }
}
