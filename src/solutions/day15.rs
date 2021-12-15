use std::collections::{BinaryHeap, HashSet};

use ndarray::Array2;

use crate::helpers::input_parsing::parse_as_2d_matrix;

pub fn parse_input(input_data: &str) -> Array2<u8> {
    parse_as_2d_matrix(input_data).unwrap()
}

#[derive(Eq, PartialEq, Debug)]
pub struct NextPathElement {
    pub cost: u64,
    pub coord: (usize, usize),
    pub prev: Option<(usize, usize)>,
    remaining_lower_bound: u64,
}

impl Ord for NextPathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.remaining_lower_bound).cmp(&(self.cost + self.remaining_lower_bound))
    }
}

impl PartialOrd for NextPathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn direct_neighbors(coord: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (coord.0.checked_sub(1), Some(coord.1)),
        (coord.0.checked_add(1), Some(coord.1)),
        (Some(coord.0), coord.1.checked_sub(1)),
        (Some(coord.0), coord.1.checked_add(1)),
    ]
    .into_iter()
    .filter_map(|coord| {
        if let (Some(x), Some(y)) = coord {
            Some((x, y))
        } else {
            None
        }
    })
}

fn manhattan_dist(c1: (usize, usize), c2: (usize, usize)) -> usize {
    let diff0 = if c1.0 > c2.0 {
        c1.0 - c2.0
    } else {
        c2.0 - c1.0
    };
    let diff1 = if c1.1 > c2.1 {
        c1.1 - c2.1
    } else {
        c2.1 - c1.1
    };
    diff0 + diff1
}

pub fn find_shortest_path<FV, FQ>(
    start: (usize, usize),
    goal: (usize, usize),
    query_map: FQ,
    mut on_step: FV,
    astar: bool,
) -> Option<u64>
where
    FQ: Fn(&(usize, usize)) -> Option<u8>,
    FV: FnMut(&NextPathElement, bool),
{
    let mut visited = HashSet::new();
    let mut next = BinaryHeap::from([NextPathElement {
        cost: 0,
        coord: start,
        prev: None,
        remaining_lower_bound: if astar {
            manhattan_dist(start, goal) as u64
        } else {
            0
        },
    }]);

    while let Some(current) = next.pop() {
        if !visited.insert(current.coord) {
            continue;
        }
        on_step(&current, true);

        if current.coord == goal {
            return Some(current.cost);
        }

        next.extend(
            direct_neighbors(&current.coord)
                .filter_map(|neighbor| {
                    query_map(&neighbor).map(|value| NextPathElement {
                        coord: neighbor,
                        cost: current.cost + value as u64,
                        prev: Some(current.coord),
                        remaining_lower_bound: if astar {
                            manhattan_dist(neighbor, goal) as u64
                        } else {
                            0
                        },
                    })
                })
                .map(|neighbor| {
                    on_step(&neighbor, false);
                    neighbor
                }),
        );
    }

    None
}

pub fn task1(map: &Array2<u8>) -> u64 {
    let start = (0, 0);
    let size = map.dim();
    let goal = (size.0 - 1, size.1 - 1);

    find_shortest_path(
        start,
        goal,
        |&coord| map.get(coord).cloned(),
        //|el| println!("{:?}", el),
        |_, _| (),
        false,
    )
    .unwrap()
}

pub fn get_wrapped_risk(map: &Array2<u8>, coord: (usize, usize)) -> Option<u8> {
    let size = map.dim();
    let wrapped_coord = (coord.0 % size.0, coord.1 % size.1);
    let tile = (coord.0 / size.0, coord.1 / size.1);
    if tile.0 >= 5 || tile.1 >= 5 {
        None
    } else {
        map.get(wrapped_coord)
            .map(|&risk| ((risk as usize + tile.0 + tile.1 + 8) % 9 + 1) as u8)
    }
}

pub fn task2(map: &Array2<u8>) -> u64 {
    let start = (0, 0);
    let size = map.dim();
    let goal = (size.0 * 5 - 1, size.1 * 5 - 1);

    find_shortest_path(
        start,
        goal,
        |&coord| get_wrapped_risk(map, coord),
        //|el| println!("{:?}", el),
        |_, _| (),
        false,
    )
    .unwrap()
}

crate::aoc_tests! {
    task1: {
        simple => 40,
        complex => 745,
    },
    task2: {
        simple => 315,
        complex => 3002,
    }
}
