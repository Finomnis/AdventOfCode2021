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

mod parsers {
    pub type Bits<'a> = (&'a [u8], usize);
    use super::{Instruction, Packet, Payload};
    use nom::{
        bits::complete::{tag, take},
        branch::alt,
        combinator::map_opt,
        multi::{length_count, many_till},
        sequence::preceded,
        IResult,
    };

    fn length(input: &Bits) -> usize {
        input.0.len() * 8 - input.1
    }

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

    pub fn subpackets_count(input: Bits) -> IResult<Bits, Vec<Packet>> {
        let packet_count = take::<_, usize, _, _>(11usize);
        preceded(tag(1, 1usize), length_count(packet_count, packet))(input)
    }

    pub fn subpackets_length(input: Bits) -> IResult<Bits, Vec<Packet>> {
        let (mut input, data_length): (Bits, usize) =
            preceded(tag(0, 1usize), take(15usize))(input)?;
        let expected_length = length(&input) - data_length;

        // Iterate manually because length_value does not work on bit parsers
        let mut packets = vec![];
        while length(&input) > expected_length {
            let result = packet(input)?;
            input = result.0;
            packets.push(result.1);
        }

        Ok((input, packets))
    }

    pub fn operator(input: Bits) -> IResult<Bits, Payload> {
        let (input, instruction) = instruction(input)?;
        let (input, subpackets) = alt((subpackets_count, subpackets_length))(input)?;

        Ok((input, Payload::Operator(instruction, subpackets)))
    }

    pub fn payload(input: Bits) -> IResult<Bits, Payload> {
        alt((literal, operator))(input)
    }

    pub fn packet(input: Bits) -> IResult<Bits, Packet> {
        let (input, version) = take(3usize)(input)?;
        let (input, payload) = payload(input)?;
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
