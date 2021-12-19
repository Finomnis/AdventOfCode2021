mod parser {
    use super::SnailfishNumber;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::u64,
        combinator::map,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn regular(input: &str) -> IResult<&str, SnailfishNumber> {
        map(u64, |num| SnailfishNumber::Number(num))(input)
    }

    pub fn pair(input: &str) -> IResult<&str, SnailfishNumber> {
        map(
            delimited(
                tag("["),
                separated_pair(snailfish_number, tag(","), snailfish_number),
                tag("]"),
            ),
            |(left, right)| SnailfishNumber::Pair(Box::new(left), Box::new(right)),
        )(input)
    }

    pub fn snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
        alt((regular, pair))(input)
    }
}

#[derive(Debug)]
pub enum SnailfishNumber {
    Number(u64),
    Pair(Box<Self>, Box<Self>),
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Number(num) => write!(f, "{}", num),
            SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

pub fn parse_input(input_data: &str) -> Vec<SnailfishNumber> {
    input_data
        .trim()
        .lines()
        .map(|line| parser::snailfish_number(line.trim()).map(|e| e.1).unwrap())
        .collect()
}

pub fn task1(numbers: &[SnailfishNumber]) -> u64 {
    for number in numbers {
        println!("{}", number);
    }
    0
}

pub fn task2(_numbers: &[SnailfishNumber]) -> u64 {
    0
}

crate::aoc_tests! {
    task1: {
        simple => 3488,
        complex => 4140,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
