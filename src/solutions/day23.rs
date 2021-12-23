use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Chamber {
    content: (Option<Amphipod>, Option<Amphipod>),
}

impl Chamber {
    pub fn new(content: (Option<Amphipod>, Option<Amphipod>)) -> Self {
        Self { content }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        Chamber::new(chamber_inputs.next().unwrap()),
        Chamber::new(chamber_inputs.next().unwrap()),
        Chamber::new(chamber_inputs.next().unwrap()),
        Chamber::new(chamber_inputs.next().unwrap()),
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

#[derive(Eq, PartialEq)]
pub struct GamePathElement {
    cost: u32,
    state: GameState,
    parent: Option<GameState>,
}

impl Ord for GamePathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for GamePathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl GamePathElement {
    pub fn new(state: GameState, parent: Option<GameState>, cost: u32) -> Self {
        Self {
            state,
            parent,
            cost,
        }
    }
}

pub fn get_follow_up_states(game_state: &GameState) -> impl Iterator<Item = (u32, GameState)> {
    vec![].into_iter()
}

pub fn task1(input_state: &GameState) -> u64 {
    let mut solved_game_states: HashMap<GameState, Option<GameState>> = HashMap::new();
    let mut cheapest_positions: BinaryHeap<GamePathElement> = BinaryHeap::new();

    cheapest_positions.push(GamePathElement::new(input_state.clone(), None, 0));

    while let Some(current_path) = cheapest_positions.pop() {
        match solved_game_states.entry(current_path.state.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => continue,
            std::collections::hash_map::Entry::Vacant(e) => e.insert(current_path.parent),
        };

        for (cost, follow_up_state) in get_follow_up_states(&current_path.state) {
            let total_cost = cost + current_path.cost;
            cheapest_positions.push(GamePathElement::new(
                follow_up_state,
                Some(current_path.state.clone()),
                total_cost,
            ));
        }
    }

    println!("{}", input_state);
    0
}

pub fn task2(_input_data: &GameState) -> u64 {
    0
}
