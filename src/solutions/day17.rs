use std::{cmp::Ordering, ops::RangeInclusive};

mod parser {
    use super::Rect;
    use nom::{
        bytes::complete::tag,
        character::complete::i32,
        sequence::{preceded, separated_pair},
        IResult,
    };

    fn i32_range(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(i32, tag(".."), i32)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Rect> {
        let (input, (x_min, x_max)) = preceded(tag("target area: x="), i32_range)(input)?;
        let (input, (y_min, y_max)) = preceded(tag(", y="), i32_range)(input)?;

        Ok((
            input,
            Rect {
                x: x_min..=x_max,
                y: y_min..=y_max,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Rect {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

pub fn parse_input(input_data: &str) -> Rect {
    let (_, rect) = parser::parse(input_data.trim()).unwrap();
    rect
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
        complex => 0,
    }
}
