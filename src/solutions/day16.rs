use num::Unsigned;
use std::fmt::Write;

use crate::helpers::nested_iterator_chain::ChainNestedIterator;

pub fn parse_input(input_data: &str) -> &str {
    input_data
}

#[derive(Debug)]
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

impl<const N: usize> std::fmt::Display for Int<N> {
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
    Operator(Instruction, Vec<Packet>),
}

#[derive(Debug)]
pub enum Instruction {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

impl Packet {
    pub fn evaluate(&self) -> u64 {
        self.payload.evaluate()
    }
}

impl Payload {
    pub fn evaluate(&self) -> u64 {
        match self {
            Payload::Literal(val) => *val,
            Payload::Operator(instruction, children) => match instruction {
                Instruction::Sum => children.iter().map(Packet::evaluate).sum(),
                Instruction::Product => children.iter().map(Packet::evaluate).product(),
                Instruction::Minimum => children.iter().map(Packet::evaluate).min().unwrap(),
                Instruction::Maximum => children.iter().map(Packet::evaluate).max().unwrap(),
                Instruction::Greater => (children[0].evaluate() > children[1].evaluate()) as u64,
                Instruction::Less => (children[0].evaluate() < children[1].evaluate()) as u64,
                Instruction::Equal => (children[0].evaluate() == children[1].evaluate()) as u64,
            },
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (v{})", self.payload, self.version)
    }
}
impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Payload::Literal(num) => write!(f, "{}", num),
            Payload::Operator(instruction, _) => write!(f, "{}", instruction),
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::Sum => "sum",
                Instruction::Product => "product",
                Instruction::Minimum => "min",
                Instruction::Maximum => "max",
                Instruction::Greater => "greater",
                Instruction::Less => "less",
                Instruction::Equal => "equal",
            }
        )
    }
}

mod parse {
    use super::*;

    fn int<const N: usize>(stream: &mut impl Iterator<Item = bool>) -> Option<Int<N>> {
        let mut result = [false; N];
        for val in &mut result {
            *val = stream.next()?;
        }
        Some(Int(result))
    }

    #[allow(clippy::blocks_in_if_conditions)]
    fn literal(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
        let mut val = 0;
        while {
            let needs_more = stream.next()?;
            let part: Int<4> = parse::int(stream)?;
            val = val * 16 + part.to_num::<u64>();
            needs_more
        } {}
        Some(Payload::Literal(val))
    }

    fn operator(stream: &mut impl Iterator<Item = bool>, payload_type: u8) -> Option<Payload> {
        let length_as_count = stream.next()?;
        let subpackets = if length_as_count {
            let count: Int<11> = parse::int(stream)?;
            (0..count.to_num::<usize>())
                .map(|_| parse::packet(stream))
                .collect::<Option<Vec<_>>>()?
        } else {
            let length: Int<15> = parse::int(stream)?;
            let mut data = (0..length.to_num::<usize>())
                .map(|_| stream.next())
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .peekable();
            let mut subpackets = vec![];
            while data.peek().is_some() {
                subpackets.push(parse::packet(&mut data)?);
            }
            subpackets
        };

        let instruction = match payload_type {
            0 => Instruction::Sum,
            1 => Instruction::Product,
            2 => Instruction::Minimum,
            3 => Instruction::Maximum,
            5 => Instruction::Greater,
            6 => Instruction::Less,
            7 => Instruction::Equal,
            _ => panic!("Unknown payload type: {}", payload_type),
        };

        Some(Payload::Operator(instruction, subpackets))
    }

    fn payload(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
        let payload_type: Int<3> = parse::int(stream)?;
        match payload_type {
            Int([true, false, false]) => parse::literal(stream),
            _ => parse::operator(stream, payload_type.to_num()),
        }
    }

    pub fn packet(stream: &mut impl Iterator<Item = bool>) -> Option<Packet> {
        let version = parse::int(stream)?;
        let payload = parse::payload(stream)?;
        Some(Packet { version, payload })
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

#[allow(dead_code)]
fn packet_tree_to_string(packet: &Packet) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    writeln!(result, "{}", packet)?;
    if let Payload::Operator(_, children) = &packet.payload {
        for child in children
            .iter()
            .map(|child| packet_tree_to_string(child))
            .collect::<Result<Vec<_>, _>>()?
        {
            for line in child.lines() {
                writeln!(result, "  {}", line)?;
            }
        }
    }
    Ok(result)
}

fn get_accumulated_version_numbers(packet: &Packet) -> usize {
    packet.version.to_num::<usize>()
        + match &packet.payload {
            Payload::Literal(_) => 0,
            Payload::Operator(_, children) => {
                children.iter().map(get_accumulated_version_numbers).sum()
            }
        }
}

pub fn task1(input_data: &str) -> usize {
    let stream = &mut hex_to_binary_stream(input_data);
    let packet = parse::packet(stream).unwrap();

    get_accumulated_version_numbers(&packet)
}

pub fn task2(input_data: &str) -> u64 {
    let stream = &mut hex_to_binary_stream(input_data);
    let packet = parse::packet(stream).unwrap();

    //println!("{}", packet_tree_to_string(&packet).unwrap());

    packet.evaluate()
}

crate::aoc_tests! {
    task1: {
        simple1 => 16,
        simple2 => 12,
        simple3 => 23,
        simple4 => 31,
        literal => 6,
        complex => 897,
    },
    task2: {
        sum => 3,
        product => 54,
        min => 7,
        max => 9,
        less => 1,
        greater => 0,
        equal => 0,
        simple5 => 1,
        complex => 0,
    }
}
