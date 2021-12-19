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
        map(u64, SnailfishNumber::regular)(input)
    }

    pub fn member(input: &str) -> IResult<&str, SnailfishNumber> {
        alt((regular, snailfish_number))(input)
    }

    pub fn snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
        map(
            delimited(tag("["), separated_pair(member, tag(","), member), tag("]")),
            SnailfishNumber::pair,
        )(input)
    }
}

#[derive(Debug, Clone)]
pub enum SnailfishNumber {
    Regular(u64),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    pub fn regular(num: u64) -> Self {
        Self::Regular(num)
    }
    pub fn pair((left, right): (SnailfishNumber, SnailfishNumber)) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Regular(num) => write!(f, "{}", num),
            SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

// pub enum ReduceResult {
//     Ok,
//     NeedsExplode(u64, SnailfishNumberElement, u64)
// }

// impl SnailfishNumber {
//     pub fn reduce(mut self) -> Self {
//         match self.reduce_impl(0) {
//             ReduceResult::Ok => (),
//             ReduceResult::NeedsExplode(_, _, _) => todo!(),
//         }
//         self
//     }

//     pub fn reduce_impl(&mut self, depth: usize) -> ReduceResult {
//         if let SnailfishNumber(
//             SnailfishNumberElement::Regular(left),
//             SnailfishNumberElement::Regular(right),
//         ) = &self
//         {
//             if depth > 4 {
//                 println!("Explode {}, {}", left, right);
//             }
//         }

//         self.0.reduce(depth)

//         ReduceResult::NoActionNeeded(self)
//     }
// }

// impl SnailfishNumberElement {
//     pub fn reduce(self, depth: usize) -> (Self, ReduceResult) {}
// }

// impl std::ops::Add for SnailfishNumber {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         SnailfishNumber(
//             SnailfishNumberElement::snailfish(self),
//             SnailfishNumberElement::snailfish(rhs),
//         )
//         .reduce()
//     }
// }

// impl<'a, 'b> std::ops::Add<&'b SnailfishNumber> for &'a SnailfishNumber {
//     type Output = SnailfishNumber;

//     fn add(self, rhs: &'b SnailfishNumber) -> Self::Output {
//         SnailfishNumber(
//             SnailfishNumberElement::snailfish(self.clone()),
//             SnailfishNumberElement::snailfish(rhs.clone()),
//         )
//         .reduce()
//     }
// }

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
