use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

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

#[derive(Clone, Copy)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

pub struct InpQuery {
    alu: Alu,
    target: Register,
}

impl InpQuery {
    pub fn input(&self, num: i64) -> Alu {
        let mut alu = self.alu.clone();
        *alu.register(self.target) = num;
        alu
    }
}

pub enum AluResult {
    Done(Alu),
    NeedsInp(InpQuery),
}

impl Alu {
    pub fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn perform_arithm<F>(mut self, a: Register, b: ArgB, f: F) -> AluResult
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let val_a = *self.register(a);
        let val_b = match b {
            ArgB::Register(r) => *self.register(r),
            ArgB::Immediate(n) => n,
        };

        *self.register(a) = f(val_a, val_b);

        AluResult::Done(self)
    }

    pub fn execute(&self, instr: &Instruction) -> AluResult {
        let alu = self.clone();
        let instr = instr.clone();
        match instr {
            Instruction::Inp(target) => AluResult::NeedsInp(InpQuery { alu, target }),
            Instruction::Add(a, b) => alu.perform_arithm(a, b, |a, b| a + b),
            Instruction::Mul(a, b) => alu.perform_arithm(a, b, |a, b| a * b),
            Instruction::Div(a, b) => alu.perform_arithm(a, b, |a, b| a / b),
            Instruction::Mod(a, b) => alu.perform_arithm(a, b, |a, b| a % b),
            Instruction::Eql(a, b) => alu.perform_arithm(a, b, |a, b| (a == b) as i64),
        }
    }

    pub fn register(&mut self, reg: Register) -> &mut i64 {
        match reg {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }

    #[allow(dead_code)]
    fn state(&self) -> (i64, i64, i64, i64) {
        (self.w, self.x, self.y, self.z)
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

pub fn task1(monad: &[Instruction]) -> i64 {
    let mut possible_alu_states: HashMap<Alu, i64> = HashMap::from([(Alu::new(), 0)]);

    for (step, instruction) in monad.iter().enumerate() {
        possible_alu_states = possible_alu_states
            .into_iter()
            .flat_map(|(alu, input_number)| {
                match alu.execute(instruction) {
                    AluResult::Done(alu) => vec![(alu, input_number)],
                    AluResult::NeedsInp(query) => (1..=9)
                        .into_iter()
                        .map(|num| (query.input(num), input_number * 10 + num))
                        .collect::<Vec<_>>(),
                }
                .into_iter()
            })
            .into_grouping_map()
            .max();
        println!(
            "Step {}: {} => {} possibilities",
            step + 1,
            instruction,
            possible_alu_states.len()
        );
        // for alu in &possible_alu_states {
        //     println!("   {:?}", alu.state());
        // }
    }

    println!(
        "{}",
        possible_alu_states
            .iter()
            .filter(|(alu, _)| alu.z == 0)
            .count()
    );

    let (alu, input) = possible_alu_states
        .iter()
        .filter(|(alu, _)| alu.z == 0)
        .max_by_key(|(_, input)| *input)
        .unwrap();

    println!("{:?}, {}", alu, input);

    *input
}

pub fn task2(_monad: &[Instruction]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alu_interactive(alu: Alu, instr: &mut impl Iterator<Item = Instruction>, num: i64) -> Alu {
        match alu.execute(instr.next().unwrap()) {
            AluResult::Done(_) => panic!("Should be inp instruction!"),
            AluResult::NeedsInp(query) => query.input(num),
        }
    }
    fn alu_noninteractive(alu: Alu, instr: &mut impl Iterator<Item = Instruction>) -> Alu {
        match alu.execute(instr.next().unwrap()) {
            AluResult::Done(alu) => alu,
            AluResult::NeedsInp(_) => panic!("Should not be an inp instruction!"),
        }
    }
    fn alu_run_to_completion(alu: Alu, instr: &mut impl Iterator<Item = Instruction>) -> Alu {
        instr.fold(alu, |alu, instr| {
            alu_noninteractive(alu, &mut [instr].into_iter())
        })
    }

    #[test]
    fn simple1() {
        let instructions = "
            inp x
            mul x -1
        ";
        let instructions = &mut parse_input(instructions).into_iter();

        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 10);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.state(), (0, -10, 0, 0));
    }

    #[test]
    fn simple2() {
        let instructions_str = "
            inp z
            inp x
            mul z 3
            eql z x
        ";
        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 3);
        let alu = alu_interactive(alu, instructions, 8);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.z, 0);

        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 3);
        let alu = alu_interactive(alu, instructions, 9);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.z, 1);
    }

    #[test]
    fn simple3() {
        let instructions_str = "
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        ";
        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 13);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.state(), (1, 1, 0, 1));
    }

    #[test]
    fn div() {
        let instructions_str = "
            inp x
            inp y
            div x y
        ";
        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 14);
        let alu = alu_interactive(alu, instructions, 3);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.x, 4);

        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, -14);
        let alu = alu_interactive(alu, instructions, 3);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.x, -4);

        let instructions = &mut parse_input(instructions_str).into_iter();
        let alu = Alu::new();
        let alu = alu_interactive(alu, instructions, 14);
        let alu = alu_interactive(alu, instructions, -3);
        let alu = alu_run_to_completion(alu, instructions);
        assert_eq!(alu.x, -4);
    }
}

crate::aoc_tests! {
    task1: {
        complex => 0,
    },
    task2: {
        complex => 0,
    }
}
