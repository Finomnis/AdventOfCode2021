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
    pub fn move_cost(self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    pub fn home_chamber(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }

    pub fn to_char(self) -> char {
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
    content: Vec<Option<Amphipod>>,
}

impl Chamber {
    pub fn new(content: (Option<Amphipod>, Option<Amphipod>)) -> Self {
        Self {
            content: vec![content.0, content.1],
        }
    }

    pub fn get_first_amphipod(&self) -> Option<(usize, Amphipod)> {
        self.content
            .iter()
            .enumerate()
            .find_map(|(pos, amphi)| amphi.map(|a| (pos, a)))
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

    pub fn chamber_is_misoccupied(&self, chamber_id: usize) -> bool {
        self.chambers[chamber_id]
            .content
            .iter()
            .any(|chamber_cell| match chamber_cell {
                None => false,
                Some(amphi) => amphi.home_chamber() != chamber_id,
            })
    }

    pub fn is_solved(&self) -> bool {
        self.chambers
            .iter()
            .enumerate()
            .all(|(chamber_id, chamber)| {
                chamber.content.iter().all(|cell| match cell {
                    Some(amphi) => amphi.home_chamber() == chamber_id,
                    None => false,
                })
            })
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
                match &chamber.content[0] {
                    Some(a) => a.to_char(),
                    None => '.',
                }
            )?;
        }
        writeln!(f, "##")?;
        let cell_length = self.chambers.iter().map(|c| c.content.len()).max().unwrap();
        for i in 1..cell_length {
            write!(f, "  #")?;
            for chamber in &self.chambers {
                write!(
                    f,
                    "{}#",
                    match &chamber.content[i] {
                        Some(a) => a.to_char(),
                        None => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
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
        other.cost.cmp(&self.cost)
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
        if !game_state.chamber_is_misoccupied(chamber_id) {
            continue;
        }

        let (pos_in_chamber, amphi) = match chamber.get_first_amphipod() {
            Some(a) => a,
            None => continue,
        };

        let base_movements = pos_in_chamber as u32 + 1;

        //println!("Movable: {}, {}", base_movements, amphi.to_char());

        let mut state = game_state.clone();
        state.chambers[chamber_id].content[pos_in_chamber] = None;

        let chamber_pos = GameState::chamber_id_to_position(chamber_id);

        // Move out and to the left
        for i in (0..chamber_pos).rev() {
            if let HallwayTile::Occupiable(occupant) = &state.hallway[i] {
                if occupant.is_some() {
                    break;
                }
                let mut state = state.clone();
                state.hallway[i] = HallwayTile::Occupiable(Some(amphi));

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
                state.hallway[i] = HallwayTile::Occupiable(Some(amphi));

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
                (hallway_pos + 1)..target_chamber_pos
            } else {
                target_chamber_pos..hallway_pos
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

                let chamber = &mut state.chambers[target_chamber];
                let chamber_free_space = chamber
                    .get_first_amphipod()
                    .map(|(pos, _)| pos)
                    .unwrap_or(chamber.content.len());
                if chamber_free_space == 0 {
                    panic!("Trying to put an amphipot into a full chamber. Should never happen.");
                }

                let vertical_distance = chamber_free_space;

                chamber.content[chamber_free_space - 1] = Some(*amphi);
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

pub fn find_cheapest_solution(
    input_state: &GameState,
) -> Option<(GamePathElement, HashMap<GameState, Option<GameState>>)> {
    let mut solved_game_states: HashMap<GameState, Option<GameState>> = HashMap::new();
    let mut cheapest_positions: BinaryHeap<GamePathElement> = BinaryHeap::new();

    cheapest_positions.push(GamePathElement::new(input_state.clone(), None, 0));

    while let Some(current_path) = cheapest_positions.pop() {
        // println!("");
        // println!("Total cost: {}", current_path.cost);
        // println!("{}", current_path.state);
        // println!("");

        if current_path.state.is_solved() {
            return Some((current_path, solved_game_states));
        }

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

    None
}

pub fn task1(input_state: &GameState) -> u32 {
    // let state = input_state.clone();
    // let (_, state) = get_follow_up_states(&state).nth(13).unwrap();
    // let (_, state) = get_follow_up_states(&state).nth(15).unwrap();

    // println!("Start: {}", state);
    // for (iter, (cost, state)) in get_follow_up_states(&state).enumerate() {
    //     println!();
    //     println!("{}: {}", iter, cost);
    //     println!("{}", state);
    //     println!();
    // }

    let (solution, _solution_map) = find_cheapest_solution(input_state).unwrap();
    solution.cost
}

pub fn task2(input_state: &GameState) -> u32 {
    let mut state = input_state.clone();
    state.chambers[0].content.insert(1, Some(Amphipod::D));
    state.chambers[0].content.insert(2, Some(Amphipod::D));
    state.chambers[1].content.insert(1, Some(Amphipod::C));
    state.chambers[1].content.insert(2, Some(Amphipod::B));
    state.chambers[2].content.insert(1, Some(Amphipod::B));
    state.chambers[2].content.insert(2, Some(Amphipod::A));
    state.chambers[3].content.insert(1, Some(Amphipod::A));
    state.chambers[3].content.insert(2, Some(Amphipod::C));
    println!("{}", state);

    let (solution, _solution_map) = find_cheapest_solution(&state).unwrap();
    solution.cost
}

crate::aoc_tests! {
    task1: {
        simple => 12521,
        complex => 16300,
    },
    task2: {
        simple => 44169,
        complex => 48676,
    }
}
