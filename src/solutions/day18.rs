use itertools::Itertools;

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
pub enum ExplodeResult {
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

    pub fn try_split(&mut self) -> bool {
        match self {
            SnailfishMember::Regular(num) => {
                if *num > 9 {
                    *self = SnailfishMember::nested(split_number(*num));
                    true
                } else {
                    false
                }
            }
            SnailfishMember::Nested(nested) => nested.try_split(),
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            SnailfishMember::Regular(num) => *num,
            SnailfishMember::Nested(nested) => nested.magnitude(),
        }
    }
}

impl SnailfishNumber {
    /// Reduces the number.
    ///
    /// Return 'false' if no further reduction is required
    pub fn reduce(&mut self) -> bool {
        let exploded = match self.try_explode(0) {
            ExplodeResult::Ok => false,
            ExplodeResult::Changed => true,
            ExplodeResult::Exploded(_, _) => panic!("Toplevel items should never explode"),
            ExplodeResult::PropagateLeft(_) => true,
            ExplodeResult::PropagateRight(_) => true,
        };

        exploded || self.try_split()
    }

    pub fn try_split(&mut self) -> bool {
        self.0.try_split() || self.1.try_split()
    }

    pub fn try_explode(&mut self, depth: usize) -> ExplodeResult {
        if depth >= 4 {
            if let SnailfishNumber(
                SnailfishMember::Regular(left),
                SnailfishMember::Regular(right),
            ) = &self
            {
                return ExplodeResult::Exploded(*left, *right);
            }
        }

        if let SnailfishMember::Nested(nested) = &mut self.0 {
            match nested.try_explode(depth + 1) {
                ExplodeResult::Ok => (),
                ExplodeResult::Exploded(left, right) => {
                    self.0 = SnailfishMember::regular(0);
                    self.1.propagate_right(right);
                    return ExplodeResult::PropagateLeft(left);
                }
                ExplodeResult::PropagateRight(propagated) => {
                    self.1.propagate_right(propagated);
                    return ExplodeResult::Changed;
                }
                result => return result,
            }
        }

        if let SnailfishMember::Nested(nested) = &mut self.1 {
            match nested.try_explode(depth + 1) {
                ExplodeResult::Ok => (),
                ExplodeResult::Exploded(left, right) => {
                    self.1 = SnailfishMember::regular(0);
                    self.0.propagate_left(left);
                    return ExplodeResult::PropagateRight(right);
                }
                ExplodeResult::PropagateLeft(propagated) => {
                    self.0.propagate_left(propagated);
                    return ExplodeResult::Changed;
                }
                result => return result,
            }
        }

        ExplodeResult::Ok
    }

    pub fn magnitude(&self) -> u64 {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result =
            SnailfishNumber(SnailfishMember::nested(self), SnailfishMember::nested(rhs));
        while result.reduce() {}
        result
    }
}

impl std::iter::Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|prev, acc| prev + acc).unwrap()
    }
}

pub fn parse_input(input_data: &str) -> Vec<SnailfishNumber> {
    input_data
        .trim()
        .lines()
        .map(|line| {
            parser::snailfish_number(line.trim())
                .map(|(_, num)| num)
                .unwrap()
        })
        .collect()
}

pub fn task1(numbers: &[SnailfishNumber]) -> u64 {
    numbers
        .to_vec()
        .into_iter()
        .sum::<SnailfishNumber>()
        .magnitude()
}

pub fn task2(numbers: &[SnailfishNumber]) -> u64 {
    numbers
        .iter()
        .combinations(2)
        .map(|items| {
            std::cmp::max(
                items
                    .iter()
                    .cloned()
                    .cloned()
                    .sum::<SnailfishNumber>()
                    .magnitude(),
                items
                    .into_iter()
                    .rev()
                    .cloned()
                    .sum::<SnailfishNumber>()
                    .magnitude(),
            )
        })
        .max()
        .unwrap()
}

crate::aoc_tests! {
    task1: {
        simple1 => 3488,
        simple2 => 4140,
        complex => 4124,
    },
    task2: {
        simple2 => 3993,
        complex => 4673,
    }
}
