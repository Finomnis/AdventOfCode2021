use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

    pub fn home_chamber(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }

    pub fn from_home_chamber(hallway_id: usize) -> Option<Self> {
        match hallway_id {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            3 => Some(Self::D),
            _ => None,
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

impl GameState {
    pub fn chamber_id_to_position(id: usize) -> usize {
        2 * id as usize + 2
    }

    pub fn chamber_position_to_id(offset: usize) -> Option<usize> {
        match offset {
            2 => Some(0),
            4 => Some(1),
            6 => Some(2),
            8 => Some(3),
            _ => None,
        }
    }

    pub fn chamber_is_misoccupied(&self, chamber_id: usize) -> bool {
        let wanted_amphipod = Amphipod::from_home_chamber(chamber_id).unwrap();

        match self.chambers[chamber_id].content {
            (None, None) => false,
            (None, Some(inh)) => wanted_amphipod != inh,
            (Some(_), None) => true,
            (Some(inh1), Some(inh2)) => inh1 != inh2 || inh1 != wanted_amphipod,
        }
    }
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
    let mut follow_ups = vec![];

    // Try to move amphipods out of chamber
    for (chamber_id, chamber) in game_state.chambers.iter().enumerate() {
        let (base_movements, amphi) = match &chamber.content {
            (Some(amphi), _) => (1, amphi),
            (None, Some(amphi)) => (2, amphi),
            (None, None) => continue,
        };

        println!("Movable: {}, {}", base_movements, amphi.to_char());

        let mut state = game_state.clone();
        if base_movements == 1 {
            state.chambers[chamber_id].content.0 = None;
        } else {
            state.chambers[chamber_id].content.1 = None;
        }

        let chamber_pos = GameState::chamber_id_to_position(chamber_id);

        // Move out and to the left
        for i in (0..chamber_pos).rev() {
            if let HallwayTile::Occupiable(occupant) = &state.hallway[i] {
                if occupant.is_some() {
                    break;
                }
                let mut state = state.clone();
                state.hallway[i] = HallwayTile::Occupiable(Some(*amphi));

                follow_ups.push((
                    ((chamber_pos - i) as u32 + base_movements) * amphi.move_cost(),
                    state,
                ));
            }
        }

        // Move out and to the right
        for i in (chamber_pos + 1)..state.hallway.len() {
            if let HallwayTile::Occupiable(occupant) = &state.hallway[i] {
                if occupant.is_some() {
                    break;
                }
                let mut state = state.clone();
                state.hallway[i] = HallwayTile::Occupiable(Some(*amphi));

                follow_ups.push((
                    ((i - chamber_pos) as u32 + base_movements) * amphi.move_cost(),
                    state,
                ));
            }
        }
    }

    // Try to move amphipods from hallway to target chambers
    for (hallway_pos, hallway_tile) in game_state.hallway.iter().enumerate() {
        if let HallwayTile::Occupiable(Some(amphi)) = hallway_tile {
            let target_chamber = amphi.home_chamber();
            if game_state.chamber_is_misoccupied(target_chamber) {
                continue;
            }
            let target_chamber_pos = GameState::chamber_id_to_position(target_chamber);

            let fields_in_between = if target_chamber_pos >= hallway_pos {
                ((hallway_pos + 1)..target_chamber_pos).into_iter()
            } else {
                (target_chamber_pos..hallway_pos).into_iter()
            };

            let path_possible = fields_in_between.into_iter().all(|pos| {
                matches!(
                    game_state.hallway[pos],
                    HallwayTile::Unoccupiable | HallwayTile::Occupiable(None)
                )
            });

            if path_possible {
                let horizontal_distance = if target_chamber_pos >= hallway_pos {
                    target_chamber_pos - hallway_pos
                } else {
                    hallway_pos - target_chamber_pos
                };

                let mut state = game_state.clone();

                let chamber_content = &mut state.chambers[target_chamber].content;
                let vertical_distance = if chamber_content.1.is_some() {
                    chamber_content.0 = Some(*amphi);
                    1
                } else {
                    chamber_content.1 = Some(*amphi);
                    2
                };

                state.hallway[hallway_pos] = HallwayTile::Occupiable(None);

                follow_ups.push((
                    (horizontal_distance + vertical_distance) as u32 * amphi.move_cost(),
                    state,
                ));
            }
        }
    }

    follow_ups.into_iter()
}

pub fn task1(input_state: &GameState) -> u64 {
    // let (_, state) = get_follow_up_states(input_state).nth(15).unwrap();
    // let (_, state) = get_follow_up_states(&state).nth(9).unwrap();

    // for (cost, follow_up_state) in get_follow_up_states(&state) {
    //     println!();
    //     println!("Cost: {}", cost);
    //     println!("{}", follow_up_state);
    // }
    // println!();

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
