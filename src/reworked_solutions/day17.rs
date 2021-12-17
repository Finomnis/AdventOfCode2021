use regex::Regex;
use std::{cmp::Ordering, ops::RangeInclusive};

#[derive(Debug)]
pub struct Rect {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

pub fn parse_input(input_data: &str) -> Rect {
    let re = Regex::new(r"^target area: x=(\d+)\.\.(\d+), y=(\-?\d+)\.\.(\-?\d+)$").unwrap();
    let captures = re.captures(input_data.trim()).unwrap();

    let x_min = captures[1].parse().unwrap();
    let x_max = captures[2].parse().unwrap();
    let y_min = captures[3].parse().unwrap();
    let y_max = captures[4].parse().unwrap();

    Rect {
        x: x_min..=x_max,
        y: y_min..=y_max,
    }
}

pub fn task1(target: &Rect) -> i32 {
    // This is a guess, which worked in my case.
    // It's not guaranteed that there is a matching x velocity,
    // though. There happened to be one in my case.
    (target.y.start() * (target.y.start() + 1)) / 2
}

pub fn drag(vx: i32) -> i32 {
    match vx.cmp(&0) {
        Ordering::Greater => vx - 1,
        Ordering::Equal => 0,
        Ordering::Less => vx + 1,
    }
}

pub fn reaches((x, y): (i32, i32), (vx, vy): (i32, i32), target: &Rect) -> bool {
    if y < *target.y.start() && vy < 0 {
        false
    } else if target.y.contains(&y) && target.x.contains(&x) {
        true
    } else {
        reaches((x + vx, y + vy), (drag(vx), vy - 1), target)
    }
}

pub fn task2(target: &Rect) -> usize {
    let vx_min = 0;
    let vx_max = *target.x.end();
    let vy_max = -target.y.start();
    let vy_min = *target.y.start();

    (vy_min..=vy_max)
        .map(|vy| {
            (vx_min..=vx_max)
                .filter(|&vx| reaches((0, 0), (vx, vy), target))
                .count()
        })
        .sum()
}

crate::aoc_tests! {
    task1: {
        simple => 45,
        complex => 2278,
    },
    task2: {
        simple => 112,
        complex => 996,
    }
}
