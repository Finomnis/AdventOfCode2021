use std::cmp::{max, min};

use itertools::Itertools;

pub fn parse_input(input_data: &str) -> (u32, u32) {
    input_data
        .trim()
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse().unwrap())
        .next_tuple()
        .unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Turn {
    Player0,
    Player1,
}

impl Turn {
    pub fn switch(&mut self) {
        *self = match self {
            Turn::Player0 => Turn::Player1,
            Turn::Player1 => Turn::Player0,
        }
    }
}

pub fn task1(&input_data: &(u32, u32)) -> u32 {
    let mut num_rolls = 0;
    let mut player_fields = input_data;
    let mut player_scores = (0, 0);

    let mut player_turn = Turn::Player0;

    let mut dice = (1..=100).into_iter().cycle();

    while max(player_scores.0, player_scores.1) < 1000 {
        let (field, score) = match player_turn {
            Turn::Player0 => (&mut player_fields.0, &mut player_scores.0),
            Turn::Player1 => (&mut player_fields.1, &mut player_scores.1),
        };

        let (dice_1, dice_2, dice_3) = dice.next_tuple().unwrap();
        num_rolls += 3;

        *field = ((*field + dice_1 + dice_2 + dice_3) + 9) % 10 + 1;
        *score += *field;

        // println!(
        //     "{:?}: rolled {}+{}+{}, moves to {} and has score {}",
        //     player_turn, dice_1, dice_2, dice_3, *field, *score
        // );

        player_turn.switch();
    }

    // println!("{}, {:?}", num_rolls, player_scores);
    min(player_scores.0, player_scores.1) * num_rolls
}

/*
pub fn dirac_dice() -> impl Iterator<Item = u32> {
    [1, 2, 3].into_iter()
}

pub fn task2_slow(&input_data: &(u32, u32)) -> u64 {
    let mut universe_count = HashMap::from([((input_data, (0, 0)), 1)]);

    let mut player_turn = Turn::Player0;

    let mut wins_player_0 = 0;
    let mut wins_player_1 = 0;

    while !universe_count.is_empty() {
        universe_count = universe_count
            .iter()
            .flat_map(|(&state, &count): (&((u32, u32), (u32, u32)), &u64)| {
                dirac_dice().flat_map(move |dice1| {
                    dirac_dice().flat_map(move |dice2| {
                        dirac_dice().map(move |dice3| {
                            let (mut fields, mut scores) = state.clone();
                            let (field, score) = match player_turn {
                                Turn::Player0 => (&mut fields.0, &mut scores.0),
                                Turn::Player1 => (&mut fields.1, &mut scores.1),
                            };

                            *field = ((*field + dice1 + dice2 + dice3) + 9) % 10 + 1;
                            *score += *field;

                            ((fields, scores), count)
                        })
                    })
                })
            })
            .filter(|((_, score), count)| {
                if score.0 >= SCORE_MAX {
                    wins_player_0 += count;
                    false
                } else if score.1 >= SCORE_MAX {
                    wins_player_1 += count;
                    false
                } else {
                    true
                }
            })
            .into_grouping_map()
            .sum();

        player_turn.switch();
        //println!("{:?}", universe_count);
    }

    // println!("Player0 wins: {}", wins_player_0);
    // println!("Player1 wins: {}", wins_player_1);

    wins_player_0
}
*/

const SCORE_MAX: u32 = 21;
const TOTAL_SCORES: usize = 2 * 10 * 10 * ((SCORE_MAX * SCORE_MAX) as usize);

#[derive(Eq, PartialEq)]
pub struct TurnState {
    field: (u32, u32),
    score: (u32, u32),
    turn: Turn,
}

impl TurnState {
    pub fn new(field: (u32, u32), score: (u32, u32), turn: Turn) -> Self {
        Self { field, score, turn }
    }

    pub fn index(&self) -> usize {
        ((match self.turn {
            Turn::Player0 => 0,
            Turn::Player1 => 1,
        }) + (self.field.0 - 1) * 2
            + (self.field.1 - 1) * 2 * 10
            + self.score.0 * 2 * 10 * 10
            + self.score.1 * 2 * 10 * 10 * SCORE_MAX) as usize
    }
}

pub struct UniverseCounter {
    count: [u64; TOTAL_SCORES],
}

impl UniverseCounter {
    pub fn new() -> Self {
        Self {
            count: [0u64; TOTAL_SCORES],
        }
    }

    pub fn get(&mut self, score: (u32, u32), field: (u32, u32), turn: Turn) -> &mut u64 {
        &mut self.count[TurnState::new(field, score, turn).index()]
    }
}

pub fn dirac_dice_combinations() -> impl Iterator<Item = (u32, u64)> {
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].into_iter()
}

pub fn task2(&input_data: &(u32, u32)) -> u64 {
    let mut universe_counter = UniverseCounter::new();

    *universe_counter.get((0, 0), input_data, Turn::Player0) = 1;

    let mut player0_wins: u64 = 0;
    //let mut player1_wins: u64 = 0;

    for score0 in 0..SCORE_MAX {
        for score1 in 0..SCORE_MAX {
            for field0 in 1..=10 {
                for field1 in 1..=10 {
                    {
                        let universe_count = *universe_counter.get(
                            (score0, score1),
                            (field0, field1),
                            Turn::Player0,
                        );

                        for (dice, dice_count) in dirac_dice_combinations() {
                            let field0 = (field0 + dice + 9) % 10 + 1;
                            let score0 = score0 + field0;
                            if score0 >= SCORE_MAX {
                                player0_wins += dice_count * universe_count;
                            } else {
                                *universe_counter.get(
                                    (score0, score1),
                                    (field0, field1),
                                    Turn::Player1,
                                ) += dice_count * universe_count;
                            }
                        }
                    }

                    {
                        let universe_count = *universe_counter.get(
                            (score0, score1),
                            (field0, field1),
                            Turn::Player1,
                        );

                        for (dice, dice_count) in dirac_dice_combinations() {
                            let field1 = (field1 + dice + 9) % 10 + 1;
                            let score1 = score1 + field1;
                            if score1 >= SCORE_MAX {
                                //player1_wins += dice_count * universe_count;
                            } else {
                                *universe_counter.get(
                                    (score0, score1),
                                    (field0, field1),
                                    Turn::Player0,
                                ) += dice_count * universe_count;
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("Player0 wins: {}", player0_wins);
    // println!("Player1 wins: {}", player1_wins);

    player0_wins
}

crate::aoc_tests! {
    task1: {
        simple => 739785,
        complex => 1006866,
    },
    task2: {
        simple => 444356092776315,
        complex => 273042027784929,
    }
}
