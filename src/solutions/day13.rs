use std::{cmp::max, collections::HashSet, fmt::Write};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Dot {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    X(i64),
    Y(i64),
}

#[derive(Debug)]
pub struct PuzzleInput {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
}

pub fn parse_input(input_data: &str) -> PuzzleInput {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    let mut parse_instructions = false;
    for line in input_data.trim().lines() {
        if line.is_empty() {
            parse_instructions = true;
            continue;
        }

        if parse_instructions {
            if let Some(val) = line.strip_prefix("fold along x=") {
                folds.push(Fold::X(val.parse().unwrap()));
            } else if let Some(val) = line.strip_prefix("fold along y=") {
                folds.push(Fold::Y(val.parse().unwrap()));
            } else {
                panic!("Unexpected input line '{}'!", line);
            }
        } else {
            let (x, y) = line.split_once(',').unwrap();
            dots.insert(Dot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }
    }

    PuzzleInput { dots, folds }
}

pub fn fold_dots(dots: HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
    dots.into_iter()
        .filter_map(|Dot { x, y }| match fold {
            Fold::X(seam) => {
                if x > *seam {
                    Some(Dot { x: 2 * seam - x, y })
                } else if x == *seam {
                    None
                } else {
                    Some(Dot { x, y })
                }
            }
            Fold::Y(seam) => {
                if y > *seam {
                    Some(Dot { x, y: 2 * seam - y })
                } else if y == *seam {
                    None
                } else {
                    Some(Dot { x, y })
                }
            }
        })
        .collect()
}

pub fn paper_to_string(dots: &HashSet<Dot>) -> String {
    let (width, height) = dots
        .iter()
        .fold((0, 0), |(x, y), dot| (max(x, dot.x + 1), max(y, dot.y + 1)));

    let mut result = String::new();

    for y in 0..height {
        writeln!(
            &mut result,
            "{}",
            (0..width)
                .map(|x| if dots.contains(&Dot { x, y }) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>()
        )
        .unwrap();
    }

    result.trim().to_string()
}

pub fn task1(input_data: &PuzzleInput) -> usize {
    input_data
        .folds
        .iter()
        .take(1)
        .fold(input_data.dots.clone(), fold_dots)
        .len()
}

pub fn task2(input_data: &PuzzleInput) -> String {
    let folded_paper = input_data
        .folds
        .iter()
        .fold(input_data.dots.clone(), fold_dots);

    paper_to_string(&folded_paper)
}

crate::aoc_tests! {
    task1: {
        simple => 17,
        complex => 607,
    },
    task2: {
        simple => "#####\n\
                   #...#\n\
                   #...#\n\
                   #...#\n\
                   #####",
        complex => ".##..###..####.#....###..####.####.#...\n\
                    #..#.#..#....#.#....#..#.#.......#.#...\n\
                    #....#..#...#..#....#..#.###....#..#...\n\
                    #....###...#...#....###..#.....#...#...\n\
                    #..#.#....#....#....#....#....#....#...\n\
                    .##..#....####.####.#....#....####.####",
    }
}
