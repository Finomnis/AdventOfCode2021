pub fn parse_input(input_data: &str) -> Vec<i64> {
    input_data
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn task1(input_data: &[i64]) -> usize {
    let mut population = input_data.to_vec();

    for _day in 1..=80 {
        let num_born = population.iter_mut().fold(0, |mut num_born, fish| {
            if *fish == 0 {
                *fish = 6;
                num_born += 1;
            } else {
                *fish -= 1;
            }
            num_born
        });
        population.extend(std::iter::repeat(8).take(num_born));
        //println!("After {} days: {:?}", day, population);
    }

    population.len()
}

pub fn task2(input_data: &[i64]) -> u64 {
    let mut histogram = [0u64; 9];

    for &fish in input_data {
        histogram[fish as usize] += 1;
    }

    for day in 1..=256 {
        histogram.rotate_left(1);
        histogram[6] += histogram[8];
        println!("After {} days: {:?}", day, histogram);
    }

    histogram.iter().sum()
}

crate::aoc_tests! {
    task1: {
        simple => 5934,
        complex => 393019,
    },
    task2: {
        simple => 26984457539,
        complex => 1757714216975,
    }
}
