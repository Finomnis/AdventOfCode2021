mod parser {
    use super::Scanner;
    use nom::{
        bytes::complete::tag,
        character::complete::{i64, newline, u64},
        combinator::map,
        multi::{many1, separated_list0},
        sequence::{delimited, terminated, tuple},
        IResult,
    };

    fn scanner_header(input: &str) -> IResult<&str, u64> {
        delimited(tag("--- scanner "), u64, tuple((tag(" ---"), newline)))(input)
    }

    fn scanner_beacon(input: &str) -> IResult<&str, (i64, i64, i64)> {
        tuple((terminated(i64, tag(",")), terminated(i64, tag(",")), i64))(input)
    }

    fn scanner_beacons(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
        separated_list0(newline, scanner_beacon)(input)
    }

    fn scanner(input: &str) -> IResult<&str, Scanner> {
        map(tuple((scanner_header, scanner_beacons)), Scanner::new)(input)
    }

    pub fn scanners(input: &str) -> IResult<&str, Vec<Scanner>> {
        separated_list0(many1(newline), scanner)(input)
    }
}

pub struct Scanner {}
impl Scanner {
    pub fn new((number, beacons): (u64, Vec<(i64, i64, i64)>)) -> Self {
        println!("Scanner {}: {:?}", number, beacons);
        Self {}
    }
}

pub fn parse_input(input_data: &str) -> Vec<Scanner> {
    let (_, scanners) = parser::scanners(input_data.trim()).unwrap();
    scanners
}

pub fn task1(scanners: &[Scanner]) -> u64 {
    0
}

pub fn task2(scanners: &[Scanner]) -> u64 {
    0
}

crate::aoc_tests! {
    task1: {
        simple => 0,
        complex => 0,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
