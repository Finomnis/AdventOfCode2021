use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helpers::nested_iterator_chain::ChainNestedIterator;

#[derive(Debug)]
pub struct GraphNode {
    name: String,
    is_large: bool,
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
                        is_large: name.chars().any(char::is_uppercase),
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

    nodes
        .into_iter()
        .sorted_unstable_by_key(|(_, (_, pos))| *pos)
        .map(|(_, (mut el, _))| {
            el.neighbors.sort_unstable();
            el
        })
        .collect()
}

fn find_num_paths(
    graph: &[GraphNode],
    node_id: usize,
    end: usize,
    visited: &mut HashSet<usize>,
    depth: usize,
) -> usize {
    if node_id == end {
        //println!("{} -> end #PATH", "  ".repeat(depth));
        return 1;
    }

    let node = &graph[node_id];

    //println!("{} -> {}", "  ".repeat(depth), node.name);
    if !node.is_large && !visited.insert(node_id) {
        return 0;
    }

    let paths = node
        .neighbors
        .iter()
        .map(|&neighbor| find_num_paths(graph, neighbor, end, visited, depth + 1))
        .sum();

    if !node.is_large {
        visited.remove(&node_id);
    }

    paths
}

pub fn task1(graph: &[GraphNode]) -> usize {
    let start_node = graph
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "start")
        .unwrap()
        .0;
    let end_node = graph
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "end")
        .unwrap()
        .0;

    find_num_paths(graph, start_node, end_node, &mut HashSet::new(), 0)
}

fn find_num_paths_double_small(
    graph: &[GraphNode],
    node_id: usize,
    start: usize,
    end: usize,
    visited: &mut HashSet<usize>,
    mut small_twice: bool,
    depth: usize,
) -> usize {
    if node_id == end {
        //println!("{} -> end #PATH", "  ".repeat(depth));
        return 1;
    }

    let node = &graph[node_id];

    //println!("{} -> {}  {:?}", "  ".repeat(depth), node.name, visited);

    let mut inserted = false;
    if !node.is_large {
        inserted = visited.insert(node_id);
        if !inserted {
            if small_twice || node_id == start {
                return 0;
            } else {
                small_twice = true;
            }
        }
    };

    let paths = node
        .neighbors
        .iter()
        .map(|&neighbor| {
            find_num_paths_double_small(
                graph,
                neighbor,
                start,
                end,
                visited,
                small_twice,
                depth + 1,
            )
        })
        .sum();

    if inserted {
        visited.remove(&node_id);
    }

    paths
}

pub fn task2(graph: &[GraphNode]) -> usize {
    let start_node = graph
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "start")
        .unwrap()
        .0;
    let end_node = graph
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "end")
        .unwrap()
        .0;

    find_num_paths_double_small(
        graph,
        start_node,
        start_node,
        end_node,
        &mut HashSet::new(),
        false,
        0,
    )
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
