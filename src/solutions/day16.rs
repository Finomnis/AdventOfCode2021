use itertools::Itertools;
use num::Unsigned;

use crate::helpers::nested_iterator_chain::ChainNestedIterator;

pub fn parse_input(input_data: &str) -> &str {
    input_data
}

pub struct Int<const N: usize>([bool; N]);
impl<const N: usize> Int<N> {
    pub fn to_num<T>(&self) -> T
    where
        T: Unsigned,
    {
        self.0.iter().fold(T::zero(), |acc, &val| {
            acc * (T::one() + T::one()) + if val { T::one() } else { T::zero() }
        })
    }
}

impl<const N: usize> std::fmt::Debug for Int<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_num::<u128>())
    }
}

#[derive(Debug)]
pub struct Packet {
    version: Int<3>,
    payload: Payload,
}

#[derive(Debug)]
pub enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

mod parse {
    use super::*;

    fn int3(stream: &mut impl Iterator<Item = bool>) -> Option<Int<3>> {
        stream.next_tuple().map(|(d2, d1, d0)| Int([d2, d1, d0]))
    }
    fn int4(stream: &mut impl Iterator<Item = bool>) -> Option<Int<4>> {
        stream
            .next_tuple()
            .map(|(d3, d2, d1, d0)| Int([d3, d2, d1, d0]))
    }

    fn literal(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
        Some(Payload::Literal(69))
    }

    fn operand(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
        Some(Payload::Literal(69))
    }

    fn payload(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
        let payload_type = parse::int3(stream)?;
        match payload_type {
            Int([true, false, false]) => parse::literal(stream),
            _ => parse::operand(stream),
        }
    }

    pub fn packet(stream: &mut impl Iterator<Item = bool>) -> Option<Packet> {
        let version = parse::int3(stream)?;
        let payload = parse::payload(stream)?;
        Some(Packet {
            version,
            payload: payload,
        })
    }
}

fn hex_to_binary_stream(input_data: &str) -> impl Iterator<Item = bool> + '_ {
    input_data.trim().chars().chain_nested_iterator(|ch| {
        [
            matches!(ch, '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F'),
            matches!(ch, '4' | '5' | '6' | '7' | 'C' | 'D' | 'E' | 'F'),
            matches!(ch, '2' | '3' | '6' | '7' | 'A' | 'B' | 'E' | 'F'),
            matches!(ch, '1' | '3' | '5' | '7' | '9' | 'B' | 'D' | 'F'),
        ]
        .into_iter()
    })
}

pub fn task1(input_data: &str) -> u64 {
    let stream = &mut hex_to_binary_stream(input_data);

    let packet = parse::packet(stream);

    println!("{:?}", packet);

    0
}

pub fn task2(_input_data: &str) -> u64 {
    0
}

crate::aoc_tests! {
    task1: {
        simple1 => 16,
        simple2 => 12,
        simple3 => 23,
        simple4 => 31,
        complex => 0,
    },
    task2: {
        complex => 0,
    }
}
