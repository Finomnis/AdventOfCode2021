use std::collections::{BinaryHeap, HashSet};

use ndarray::Array2;

use crate::helpers::input_parsing::parse_as_2d_matrix;

pub fn parse_input(input_data: &str) -> Array2<u8> {
    parse_as_2d_matrix(input_data).unwrap()
}

#[derive(Eq, Debug)]
pub struct NextPathElement {
    cost: u64,
    coord: (usize, usize),
    prev: Option<(usize, usize)>,
}

impl Ord for NextPathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NextPathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NextPathElement {
    fn eq(&self, other: &Self) -> bool {
        other.cost.eq(&self.cost)
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

pub fn search_shortest_path<G, F>(
    start: (usize, usize),
    goal: (usize, usize),
    query_map: G,
    mut on_visited: F,
) -> u64
where
    G: Fn(&(usize, usize)) -> Option<u8>,
    F: FnMut(&NextPathElement),
{
    let mut visited = HashSet::new();
    let mut next = BinaryHeap::from([NextPathElement {
        cost: 0,
        coord: start,
        prev: None,
    }]);

    while let Some(current) = next.pop() {
        if !visited.insert(current.coord) {
            continue;
        }
        on_visited(&current);

        if current.coord == goal {
            return current.cost;
        }

        next.extend(direct_neighbors(&current.coord).filter_map(|neighbor| {
            query_map(&neighbor).map(|value| NextPathElement {
                coord: neighbor,
                cost: current.cost + value as u64,
                prev: Some(current.coord),
            })
        }));
    }

    0
}

pub fn task1(map: &Array2<u8>) -> u64 {
    let start = (0, 0);
    let size = map.dim();
    let goal = (size.0 - 1, size.1 - 1);

    search_shortest_path(
        start,
        goal,
        |&coord| map.get(coord).cloned(),
        |el| println!("{:?}", el),
    )
}

pub fn task2(_input_data: &Array2<u8>) -> usize {
    0
}

crate::aoc_tests! {
    task1: {
        simple => 40,
        complex => 745,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
