use std::fmt::Display;

mod parser {
    use super::{ArgB, Instruction, Register};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, i64, space1},
        combinator::{map, map_opt, value},
        sequence::separated_pair,
        IResult,
    };

    pub fn arg_a(input: &str) -> IResult<&str, Register> {
        alt((
            value(Register::W, tag("w")),
            value(Register::X, tag("x")),
            value(Register::Y, tag("y")),
            value(Register::Z, tag("z")),
        ))(input)
    }

    pub fn arg_b(input: &str) -> IResult<&str, ArgB> {
        let reg = map(arg_a, |r| ArgB::Register(r));
        let imm = map(i64, |n| ArgB::Immediate(n));
        alt((reg, imm))(input)
    }

    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        let instr_inp = map(separated_pair(tag("inp"), space1, arg_a), |(_, reg)| {
            Instruction::Inp(reg)
        });
        let instr = map_opt(
            separated_pair(alpha1, space1, separated_pair(arg_a, space1, arg_b)),
            |(instr_tag, (a, b))| match instr_tag {
                "add" => Some(Instruction::Add(a, b)),
                "mul" => Some(Instruction::Mul(a, b)),
                "div" => Some(Instruction::Div(a, b)),
                "mod" => Some(Instruction::Mod(a, b)),
                "eql" => Some(Instruction::Eql(a, b)),
                _ => None,
            },
        );
        alt((instr_inp, instr))(input)
    }
}

#[derive(Clone)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub enum ArgB {
    Register(Register),
    Immediate(i64),
}

#[derive(Clone)]
pub enum Instruction {
    Inp(Register),
    Add(Register, ArgB),
    Mul(Register, ArgB),
    Div(Register, ArgB),
    Mod(Register, ArgB),
    Eql(Register, ArgB),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(a) => write!(f, "inp {}", a),
            Instruction::Add(a, b) => write!(f, "add {} {}", a, b),
            Instruction::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Instruction::Div(a, b) => write!(f, "div {} {}", a, b),
            Instruction::Mod(a, b) => write!(f, "mod {} {}", a, b),
            Instruction::Eql(a, b) => write!(f, "eql {} {}", a, b),
        }
    }
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::W => 'w',
                Register::X => 'x',
                Register::Y => 'y',
                Register::Z => 'z',
            }
        )
    }
}
impl Display for ArgB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgB::Register(reg) => write!(f, "{}", reg),
            ArgB::Immediate(num) => write!(f, "{}", num),
        }
    }
}

pub fn parse_input(input_data: &str) -> Vec<Instruction> {
    input_data
        .trim()
        .lines()
        .map(|line| {
            let (_, instruction) = parser::instruction(line.trim()).unwrap();
            instruction
        })
        .collect()
}

pub fn task1(monad: &[Instruction]) -> u32 {
    for instruction in monad {
        println!("{}", instruction);
    }
    0
}

pub fn task2(_monad: &[Instruction]) -> u32 {
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
