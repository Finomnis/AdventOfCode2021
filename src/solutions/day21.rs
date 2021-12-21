use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;

pub fn parse_input(input_data: &str) -> (u32, u32) {
    input_data
        .trim()
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse().unwrap())
        .next_tuple()
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Turn {
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

pub fn dirac_dice() -> impl Iterator<Item = u32> {
    [1, 2, 3].into_iter()
}

pub fn task2(&input_data: &(u32, u32)) -> u64 {
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
                if score.0 >= 21 {
                    wins_player_0 += count;
                    false
                } else if score.1 >= 21 {
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

    wins_player_0
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
