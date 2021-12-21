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

pub fn task1(&input_data: &(u32, u32)) -> u32 {
    let mut num_rolls = 0;
    let mut player_fields = input_data;
    let mut player_scores = (0, 0);

    let mut player_0_turn = true;

    let mut dice = (1..=100).into_iter().cycle();

    while max(player_scores.0, player_scores.1) < 1000 {
        let (field, score) = if player_0_turn {
            (&mut player_fields.0, &mut player_scores.0)
        } else {
            (&mut player_fields.1, &mut player_scores.1)
        };
        player_0_turn = !player_0_turn;

        let (dice_1, dice_2, dice_3) = dice.next_tuple().unwrap();
        num_rolls += 3;

        *field = ((*field + dice_1 + dice_2 + dice_3) + 9) % 10 + 1;
        *score += *field;

        // println!(
        //     "Player {}: rolled {}+{}+{}, moves to {} and has score {}",
        //     player_0_turn as u32,
        //     dice_1,
        //     dice_2,
        //     dice_3,
        //     *field,
        //     *score
        // );
    }

    // println!("{}, {:?}", num_rolls, player_scores);
    min(player_scores.0, player_scores.1) * num_rolls
}

pub fn dirac_dice() -> impl Iterator<Item = u32> {
    [1, 2, 3].into_iter()
}

pub fn task2(&input_data: &(u32, u32)) -> usize {
    let mut universe_count = HashMap::from([((input_data, (0, 0)), 1)]);

    let mut player_0_turn = true;

    universe_count = universe_count
        .iter()
        .flat_map(|(&state, &count)| {
            dirac_dice().flat_map(move |dice1| {
                dirac_dice().flat_map(move |dice2| {
                    dirac_dice().map(move |dice3| {
                        let (mut fields, mut scores) = state.clone();
                        let (field, score) = if player_0_turn {
                            (&mut fields.0, &mut scores.0)
                        } else {
                            (&mut fields.1, &mut scores.1)
                        };

                        *field = ((*field + dice1 + dice2 + dice3) + 9) % 10 + 1;
                        *score += *field;

                        ((fields, scores), count)
                    })
                })
            })
        })
        .into_grouping_map()
        .sum();

    println!("{:?}", universe_count);

    0
}

crate::aoc_tests! {
    task1: {
        simple => 739785,
        complex => 1006866,
    },
    task2: {
        simple => 0,
        complex => 0,
    }
}
