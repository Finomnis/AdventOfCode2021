use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    pub fn move_cost(&self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    pub fn home_hallway(&self) -> usize {
        match self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
        }
    }
}

#[derive(Debug)]
pub enum HallwayTile {
    Unoccupiable,
    Occupiable(Option<Amphipod>),
}

impl Display for HallwayTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HallwayTile::Unoccupiable => '.',
                HallwayTile::Occupiable(None) => '.',
                HallwayTile::Occupiable(Some(amphi)) => amphi.to_char(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Chamber {
    content: (Option<Amphipod>, Option<Amphipod>),
    pos: usize,
}

impl Chamber {
    pub fn new(content: (Option<Amphipod>, Option<Amphipod>), pos: usize) -> Self {
        Self { content, pos }
    }
}

#[derive(Debug)]
pub struct GameState {
    hallway: [HallwayTile; 11],
    chambers: [Chamber; 4],
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for hallway_tile in &self.hallway {
            write!(f, "{}", hallway_tile)?;
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        for chamber in &self.chambers {
            write!(
                f,
                "{}#",
                match &chamber.content.0 {
                    Some(a) => a.to_char(),
                    None => '.',
                }
            )?;
        }
        writeln!(f, "##")?;
        write!(f, "  #")?;
        for chamber in &self.chambers {
            write!(
                f,
                "{}#",
                match &chamber.content.1 {
                    Some(a) => a.to_char(),
                    None => '.',
                }
            )?;
        }
        writeln!(f)?;
        write!(f, "  #########")
    }
}

pub fn parse_input(input_data: &str) -> GameState {
    let (line1, line2) = input_data
        .lines()
        .skip(2)
        .take(2)
        .map(|l| {
            l.matches(char::is_alphabetic).map(|s| match s {
                "A" => Some(Amphipod::A),
                "B" => Some(Amphipod::B),
                "C" => Some(Amphipod::C),
                "D" => Some(Amphipod::D),
                _ => None,
            })
        })
        .collect_tuple()
        .unwrap();

    let mut chamber_inputs = line1.zip(line2);

    let chambers = [
        Chamber::new(chamber_inputs.next().unwrap(), 2),
        Chamber::new(chamber_inputs.next().unwrap(), 4),
        Chamber::new(chamber_inputs.next().unwrap(), 6),
        Chamber::new(chamber_inputs.next().unwrap(), 8),
    ];

    let hallway = [
        HallwayTile::Occupiable(None),
        HallwayTile::Occupiable(None),
        HallwayTile::Unoccupiable,
        HallwayTile::Occupiable(None),
        HallwayTile::Unoccupiable,
        HallwayTile::Occupiable(None),
        HallwayTile::Unoccupiable,
        HallwayTile::Occupiable(None),
        HallwayTile::Unoccupiable,
        HallwayTile::Occupiable(None),
        HallwayTile::Occupiable(None),
    ];

    GameState { chambers, hallway }
}

pub fn task1(input_data: &GameState) -> u64 {
    println!("{}", input_data);
    0
}

pub fn task2(input_data: &GameState) -> u64 {
    0
}
