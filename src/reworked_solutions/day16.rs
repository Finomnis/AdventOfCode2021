use nom::{bits, error::Error};

#[derive(Debug)]
pub struct Packet {
    version: u8,
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
            Payload::Operator(instruction, children) => {
                let children = children.iter().map(Packet::evaluate).collect::<Vec<_>>();
                match instruction {
                    Instruction::Sum => children.iter().sum(),
                    Instruction::Product => children.iter().product(),
                    Instruction::Minimum => children.into_iter().min().unwrap(),
                    Instruction::Maximum => children.into_iter().max().unwrap(),
                    Instruction::Greater => (children[0] > children[1]) as u64,
                    Instruction::Less => (children[0] < children[1]) as u64,
                    Instruction::Equal => (children[0] == children[1]) as u64,
                }
            }
        }
    }
}

// mod parse {
//     use super::*;

//     fn int<const N: usize>(stream: &mut impl Iterator<Item = bool>) -> Option<Int<N>> {
//         let mut result = [false; N];
//         for val in &mut result {
//             *val = stream.next()?;
//         }
//         Some(Int(result))
//     }

//     #[allow(clippy::blocks_in_if_conditions)]
//     fn literal(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
//         let mut val = 0;
//         while {
//             let needs_more = stream.next()?;
//             let part: Int<4> = parse::int(stream)?;
//             val = val * 16 + part.to_num::<u64>();
//             needs_more
//         } {}
//         Some(Payload::Literal(val))
//     }

//     fn operator(stream: &mut impl Iterator<Item = bool>, payload_type: u8) -> Option<Payload> {
//         let length_as_count = stream.next()?;
//         let subpackets = if length_as_count {
//             let count: Int<11> = parse::int(stream)?;
//             (0..count.to_num::<usize>())
//                 .map(|_| parse::packet(stream))
//                 .collect::<Option<Vec<_>>>()?
//         } else {
//             let length: Int<15> = parse::int(stream)?;
//             let mut data = (0..length.to_num::<usize>())
//                 .map(|_| stream.next())
//                 .collect::<Option<Vec<_>>>()?
//                 .into_iter()
//                 .peekable();
//             let mut subpackets = vec![];
//             while data.peek().is_some() {
//                 subpackets.push(parse::packet(&mut data)?);
//             }
//             subpackets
//         };

//         let instruction = match payload_type {
//             0 => Instruction::Sum,
//             1 => Instruction::Product,
//             2 => Instruction::Minimum,
//             3 => Instruction::Maximum,
//             5 => Instruction::Greater,
//             6 => Instruction::Less,
//             7 => Instruction::Equal,
//             _ => panic!("Unknown payload type: {}", payload_type),
//         };

//         Some(Payload::Operator(instruction, subpackets))
//     }

//     fn payload(stream: &mut impl Iterator<Item = bool>) -> Option<Payload> {
//         let payload_type: Int<3> = parse::int(stream)?;
//         match payload_type {
//             Int([true, false, false]) => parse::literal(stream),
//             _ => parse::operator(stream, payload_type.to_num()),
//         }
//     }

//     pub fn packet(stream: &mut impl Iterator<Item = bool>) -> Option<Packet> {
//         let version = parse::int(stream)?;
//         let payload = parse::payload(stream)?;
//         Some(Packet { version, payload })
//     }
// }

mod parsers {
    pub type Bits<'a> = (&'a [u8], usize);
    use super::{Instruction, Packet, Payload};
    use nom::{
        bits::complete::{tag, take},
        branch::alt,
        combinator::map_opt,
        multi::many_till,
        sequence::{preceded, tuple},
        IResult,
    };

    pub fn literal(input: Bits) -> IResult<Bits, Payload> {
        let part_continue = preceded(tag(1, 1usize), take(4usize));
        let part_finished = preceded(tag(0, 1usize), take(4usize));
        let number_parts = many_till(part_continue, part_finished);
        let (input, (parts, last)) = preceded(tag(4, 3usize), number_parts)(input)?;

        let number = parts
            .into_iter()
            .chain(std::iter::once(last))
            .fold(0, |acc, part: u64| acc * 16 + part);

        Ok((input, Payload::Literal(number)))
    }

    pub fn instruction(input: Bits) -> IResult<Bits, Instruction> {
        map_opt(
            take(3usize),
            |instruction_type: u8| match instruction_type {
                0 => Some(Instruction::Sum),
                1 => Some(Instruction::Product),
                2 => Some(Instruction::Minimum),
                3 => Some(Instruction::Maximum),
                5 => Some(Instruction::Greater),
                6 => Some(Instruction::Less),
                7 => Some(Instruction::Equal),
                _ => None,
            },
        )(input)
    }

    pub fn operator(input: Bits) -> IResult<Bits, Payload> {
        let (input, instruction) = instruction(input)?;

        println!("INST: {:?}", instruction);

        Ok((input, Payload::Literal(13)))
    }

    pub fn payload(input: Bits) -> IResult<Bits, Payload> {
        alt((literal, operator))(input)
    }

    pub fn packet(input: Bits) -> IResult<Bits, Packet> {
        let (input, (version, payload)) = tuple((take(3usize), payload))(input)?;
        Ok((input, Packet { version, payload }))
    }
}

pub fn hex_to_binary(s: &str) -> Vec<u8> {
    // TODO write .array_chunks iterators
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn parse_input(input_data: &str) -> Packet {
    let data = hex_to_binary(input_data.trim());

    let (_, packet) =
        bits::<_, _, Error<parsers::Bits>, Error<&[u8]>, _>(parsers::packet)(&data).unwrap();

    packet
}

pub fn task1(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.payload {
            Payload::Literal(_) => 0,
            Payload::Operator(_, children) => children.iter().map(task1).sum(),
        }
}

pub fn task2(packet: &Packet) -> u64 {
    println!("{:?}", packet);
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
        complex => 9485076995911,
    }
}
