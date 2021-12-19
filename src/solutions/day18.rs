mod parser {
    use super::{SnailfishMember, SnailfishNumber};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::u64,
        combinator::map,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn member(input: &str) -> IResult<&str, SnailfishMember> {
        alt((
            map(u64, SnailfishMember::regular),
            map(snailfish_number, SnailfishMember::nested),
        ))(input)
    }

    pub fn snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
        map(
            delimited(tag("["), separated_pair(member, tag(","), member), tag("]")),
            |(first, second)| SnailfishNumber(first, second),
        )(input)
    }
}

#[derive(Debug, Clone)]
pub enum SnailfishMember {
    Regular(u64),
    Nested(Box<SnailfishNumber>),
}
impl SnailfishMember {
    pub fn regular(num: u64) -> Self {
        Self::Regular(num)
    }
    pub fn nested(num: SnailfishNumber) -> Self {
        Self::Nested(Box::new(num))
    }
}

#[derive(Debug, Clone)]
pub struct SnailfishNumber(SnailfishMember, SnailfishMember);

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{},{}]",
            match &self.0 {
                SnailfishMember::Regular(num) => format!("{}", num),
                SnailfishMember::Nested(num) => format!("{}", num),
            },
            match &self.1 {
                SnailfishMember::Regular(num) => format!("{}", num),
                SnailfishMember::Nested(num) => format!("{}", num),
            }
        )
    }
}

pub enum ReduceResult {
    Ok(SnailfishNumber),
}

//  impl SnailfishNumber {
//     pub fn reduce(&mut self) -> Self {
//         match self.reduce_impl(0) {
//             ReduceResult::Ok(red)
//         }
//         self
//     }

//     pub fn reduce_impl(&mut self, depth: usize) -> ReduceResult {
//         if self.depth >= 4 {
//             if is_regular_pair {

//             }
//         }

//         self.0.reduce(depth)

//         ReduceResult::NoActionNeeded(self)
//     }
// }

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SnailfishNumber(SnailfishMember::nested(self), SnailfishMember::nested(rhs))
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
    let numbers = numbers.to_vec();

    for number in numbers {
        println!("{}", number);
    }
    // numbers.into_iter().reduce(|prev, acc| {
    //     print!("{} + {} = ", prev, acc);
    //     let result = prev + acc;
    //     println!("{}", result);
    //     result
    // });

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
