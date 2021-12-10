use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use crate::helpers::input_parsing::ParseError;

#[derive(Debug)]
pub struct BingoCell {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct BingoBoard {
    cells: HashMap<i64, BingoCell>,
    cells_x: usize,
    cells_y: usize,
}

#[derive(Debug)]
pub struct BingoGame {
    boards: Vec<BingoBoard>,
    numbers: Vec<i64>,
}

impl FromStr for BingoBoard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        let mut cells_x = 0;
        let mut cells_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, cell) in line.split_whitespace().enumerate() {
                cells.insert(cell.parse().unwrap(), BingoCell { x, y });
                cells_x = std::cmp::max(cells_x, x + 1);
                cells_y = std::cmp::max(cells_y, y + 1);
            }
        }

        Ok(BingoBoard {
            cells,
            cells_x,
            cells_y,
        })
    }
}

pub fn parse_input(input_data: &str) -> BingoGame {
    let consistent_newline_input = input_data.lines().collect::<Vec<_>>().join("\n");

    let mut parts = consistent_newline_input.split("\n\n");

    let numbers = parts
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let boards = parts.map(str::parse).map(Result::unwrap).collect();

    BingoGame { boards, numbers }
}

#[derive(Default)]
struct Score {
    x: HashMap<usize, i64>,
    y: HashMap<usize, i64>,
    won: bool,
}

pub fn task1(input_data: &BingoGame) -> i64 {
    println!("{:?}", input_data);

    let mut scores: Vec<Score> = input_data.boards.iter().map(|_| Score::default()).collect();
    let mut activated_numbers: HashSet<i64> = HashSet::new();

    for number in input_data.numbers.iter().unique() {
        activated_numbers.insert(*number);
        for (score, board) in scores.iter_mut().zip(input_data.boards.iter()) {
            if let Some(cell) = board.cells.get(number) {
                let entry_x = score.x.entry(cell.x).or_insert(0);
                let entry_y = score.y.entry(cell.y).or_insert(0);

                *entry_x += 1;
                *entry_y += 1;
                if *entry_x >= board.cells_x as i64 || *entry_y >= board.cells_y as i64 {
                    println!(
                        "Board won with number: {}, ({}, {})",
                        number, cell.x, cell.y
                    );
                    let mut sum_unmarked = 0;
                    for cell_number in board.cells.keys() {
                        if !activated_numbers.contains(cell_number) {
                            sum_unmarked += cell_number;
                        }
                    }
                    println!("Sum unmarked: {}", sum_unmarked);
                    return sum_unmarked * number;
                }
            }
        }
    }

    0
}

pub fn task2(input_data: &BingoGame) -> i64 {
    println!("{:?}", input_data);

    let mut scores: Vec<Score> = input_data.boards.iter().map(|_| Score::default()).collect();
    let mut activated_numbers: HashSet<i64> = HashSet::new();

    let mut num_won: usize = 0;
    let num_boards = scores.len();

    for number in input_data.numbers.iter().unique() {
        activated_numbers.insert(*number);
        for (score, board) in scores.iter_mut().zip(input_data.boards.iter()) {
            if let Some(cell) = board.cells.get(number) {
                let entry_x = score.x.entry(cell.x).or_insert(0);
                let entry_y = score.y.entry(cell.y).or_insert(0);

                *entry_x += 1;
                *entry_y += 1;
                if (*entry_x >= board.cells_x as i64 || *entry_y >= board.cells_y as i64)
                    && !score.won
                {
                    score.won = true;
                    num_won += 1;

                    if num_won >= num_boards {
                        println!(
                            "Board won with number: {}, ({}, {})",
                            number, cell.x, cell.y
                        );
                        let mut sum_unmarked = 0;
                        for cell_number in board.cells.keys() {
                            if !activated_numbers.contains(cell_number) {
                                sum_unmarked += cell_number;
                            }
                        }
                        println!("Sum unmarked: {}", sum_unmarked);
                        return sum_unmarked * number;
                    }
                }
            }
        }
    }

    0
}

crate::aoc_tests! {
    task1: {
        simple => 4512,
        complex => 29440,
    },
    task2: {
        simple => 1924,
        complex => 13884,
    }
}
