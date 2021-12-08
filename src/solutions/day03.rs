pub fn parse_input(input_data: &str) -> Vec<Vec<bool>> {
    input_data
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Unexpected character '{}'", ch),
                })
                .collect()
        })
        .collect()
}

pub fn create_histogram(input_data: &[Vec<bool>]) -> Vec<usize> {
    input_data.iter().fold(Vec::new(), |mut hist, elem| {
        hist.resize(elem.len(), 0);

        for (hist_entry, &digit) in hist.iter_mut().zip(elem.iter()) {
            if digit {
                *hist_entry += 1;
            }
        }

        hist
    })
}

pub fn task1(input_data: &[Vec<bool>]) -> i64 {
    let hist = create_histogram(input_data);

    let mut gamma = 0;
    let mut epsilon = 0;

    for elem in hist {
        gamma *= 2;
        epsilon *= 2;
        if elem * 2 > input_data.len() {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn filter_numbers(numbers: Vec<Vec<bool>>, digit: usize, use_more_common: bool) -> Vec<Vec<bool>> {
    if numbers.len() == 1 {
        return numbers;
    }

    let hist = create_histogram(&numbers);

    let wanted_digit = if 2 * hist[digit] >= numbers.len() {
        use_more_common
    } else {
        !use_more_common
    };

    numbers
        .into_iter()
        .filter(|elem| elem[digit] == wanted_digit)
        .collect()
}

fn binary_to_number(binary: &[bool]) -> i64 {
    binary
        .iter()
        .fold(0, |accu, &val| accu * 2 + if val { 1 } else { 0 })
}

pub fn task2(input_data: &[Vec<bool>]) -> i64 {
    let mut result_oxy = input_data.to_vec();
    let mut result_co2 = input_data.to_vec();
    for digit in 0..input_data[0].len() {
        result_oxy = filter_numbers(result_oxy, digit, true);
        result_co2 = filter_numbers(result_co2, digit, false);
    }

    let value_oxy = binary_to_number(&result_oxy[0]);
    let value_co2 = binary_to_number(&result_co2[0]);

    println!("Oxy: {}", value_oxy);
    println!("CO2: {}", value_co2);

    value_co2 * value_oxy
}

crate::aoc_tests! {
    task1: {
        (simple, "day03_simple.txt", "198")
        (complex, "day03_complex.txt", "749376")
    },
    task2: {
        (simple, "day03_simple.txt", "230")
        (complex, "day03_complex.txt", "2372923")
    }
}
