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

#[must_use = "Results of the reduce step actually need to be processed."]
pub enum ReduceResult {
    Ok,
    Changed,
    Exploded(u64, u64),
    PropagateLeft(u64),
    PropagateRight(u64),
}

pub fn split_number(num: u64) -> SnailfishNumber {
    SnailfishNumber(
        SnailfishMember::regular(num / 2),
        SnailfishMember::regular((num + 1) / 2),
    )
}

impl SnailfishMember {
    pub fn propagate_left(&mut self, propagated: u64) {
        match self {
            SnailfishMember::Regular(num) => *num += propagated,
            SnailfishMember::Nested(nested) => nested.1.propagate_left(propagated),
        }
    }
    pub fn propagate_right(&mut self, propagated: u64) {
        match self {
            SnailfishMember::Regular(num) => *num += propagated,
            SnailfishMember::Nested(nested) => nested.0.propagate_right(propagated),
        }
    }
}

impl SnailfishNumber {
    /// Reduces the number.
    ///
    /// Return 'false' if no further reduction is required
    pub fn reduce(&mut self) -> bool {
        match self.reduce_impl(0) {
            ReduceResult::Ok => false,
            ReduceResult::Changed => true,
            ReduceResult::Exploded(_, _) => panic!("Toplevel items should never explode"),
            ReduceResult::PropagateLeft(_) => true,
            ReduceResult::PropagateRight(_) => true,
        }
    }

    pub fn reduce_impl(&mut self, depth: usize) -> ReduceResult {
        if depth >= 4 {
            if let SnailfishNumber(
                SnailfishMember::Regular(left),
                SnailfishMember::Regular(right),
            ) = &self
            {
                return ReduceResult::Exploded(*left, *right);
            }
        }

        match &mut self.0 {
            SnailfishMember::Regular(num) => {
                if *num > 9 {
                    self.0 = SnailfishMember::nested(split_number(*num));
                    return ReduceResult::Changed;
                }
            }
            SnailfishMember::Nested(nested) => match nested.reduce_impl(depth + 1) {
                ReduceResult::Ok => (),
                ReduceResult::Exploded(left, right) => {
                    self.0 = SnailfishMember::regular(0);
                    self.1.propagate_right(right);
                    return ReduceResult::PropagateLeft(left);
                }
                ReduceResult::PropagateRight(propagated) => {
                    self.1.propagate_right(propagated);
                    return ReduceResult::Changed;
                }
                result => return result,
            },
        }

        match &mut self.1 {
            SnailfishMember::Regular(num) => {
                if *num > 9 {
                    self.1 = SnailfishMember::nested(split_number(*num));
                    return ReduceResult::Changed;
                }
            }
            SnailfishMember::Nested(nested) => match nested.reduce_impl(depth + 1) {
                ReduceResult::Ok => (),
                ReduceResult::Exploded(left, right) => {
                    self.1 = SnailfishMember::regular(0);
                    self.0.propagate_left(left);
                    return ReduceResult::PropagateRight(right);
                }
                ReduceResult::PropagateLeft(propagated) => {
                    self.0.propagate_left(propagated);
                    return ReduceResult::Changed;
                }
                result => return result,
            },
        }

        ReduceResult::Ok
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result =
            SnailfishNumber(SnailfishMember::nested(self), SnailfishMember::nested(rhs));
        while result.reduce() {
            println!("{}", result);
        }
        result
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

    for number in &numbers {
        println!("{}", number);
    }

    numbers.into_iter().reduce(|prev, acc| {
        println!("Computing {} + {} ...", prev, acc);
        let result = prev + acc;
        println!("  ... result: {}", result);
        result
    });

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
