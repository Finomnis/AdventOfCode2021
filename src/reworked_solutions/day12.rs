use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helpers::nested_iterator_chain::ChainNestedIterator;
use crate::helpers::temporary_hashset::HashSetExt;

#[derive(Debug)]
pub struct GraphNode {
    name: String,
    is_small: bool,
    neighbors: Vec<usize>,
}

pub fn parse_input(input_data: &str) -> Vec<GraphNode> {
    let mut nodes = input_data
        .trim()
        .lines()
        .chain_nested_iterator(|e| e.split('-'))
        .unique()
        .enumerate()
        .map(|(id, name)| {
            (
                name,
                (
                    GraphNode {
                        name: name.to_string(),
                        neighbors: vec![],
                        is_small: !name.chars().any(char::is_uppercase),
                    },
                    id,
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    input_data.trim().lines().for_each(|line| {
        let (first, second) = line.split_once('-').unwrap();

        let first_id = nodes[first].1;
        let second_id = nodes[second].1;

        nodes.get_mut(first).unwrap().0.neighbors.push(second_id);
        nodes.get_mut(second).unwrap().0.neighbors.push(first_id);
    });

    let graph = nodes
        .into_iter()
        .sorted_unstable_by_key(|(_, (_, pos))| *pos)
        .map(|(_, (mut el, _))| {
            el.neighbors.sort_unstable();
            el
        })
        .collect::<Vec<_>>();

    graph
}

fn find_num_paths(
    graph: &[GraphNode],
    node_id: usize,
    start: usize,
    end: usize,
    visited: &mut HashSet<usize>,
    mut can_double_visit: bool,
) -> usize {
    if node_id == end {
        return 1;
    }

    let node = &graph[node_id];

    let (inserted, mut visited) = visited.temporary_insert(node_id);
    if !inserted && node.is_small {
        if !can_double_visit || node_id == start {
            return 0;
        }
        can_double_visit = false;
    }

    node.neighbors
        .iter()
        .map(|&neighbor| {
            find_num_paths(graph, neighbor, start, end, &mut visited, can_double_visit)
        })
        .sum()
}

fn find_node(nodes: &[GraphNode], name: &str) -> usize {
    nodes.iter().position(|node| node.name == name).unwrap()
}

pub fn task1(graph: &[GraphNode]) -> usize {
    let start = find_node(graph, "start");
    let end = find_node(graph, "end");

    find_num_paths(graph, start, start, end, &mut HashSet::new(), false)
}

pub fn task2(graph: &[GraphNode]) -> usize {
    let start = find_node(graph, "start");
    let end = find_node(graph, "end");

    find_num_paths(graph, start, start, end, &mut HashSet::new(), true)
}

crate::aoc_tests! {
    task1: {
        simple1 => 10,
        simple2 => 19,
        simple3 => 226,
        complex => 5576,
    },
    task2: {
        simple1 => 36,
        simple2 => 103,
        simple3 => 3509,
        complex => 152837,
    }
}
