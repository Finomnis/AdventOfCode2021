pub fn task1(input_data: &str) -> String {
    input_data
        .lines()
        .map(|el| el.parse().unwrap())
        .fold((0, u32::MAX), |(sum, prev), elem| {
            (sum + if elem > prev { 1 } else { 0 }, elem)
        })
        .0
        .to_string()
}

fn optional_add(a: Option<u32>, b: u32) -> Option<u32> {
    if let Some(prev) = a {
        Some(prev + b)
    } else {
        None
    }
}

fn check_increased(prev: Option<u32>, curr: Option<u32>, elem: u32) -> u32 {
    if let Some(prev_uwrapped) = prev {
        if let Some(curr_unwrapped) = curr {
            if curr_unwrapped + elem > prev_uwrapped {
                1
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    }
}

pub fn task2(input_data: &str) -> String {
    input_data
        .lines()
        .map(|el| el.parse::<u32>().unwrap())
        .fold(
            (0u32, None, None, None),
            |(sum, group1, group2, group3), elem| {
                println!("{:?} {:?}", optional_add(group2, elem), group3);
                (
                    sum + check_increased(group3, group2, elem),
                    Some(elem),
                    optional_add(group1, elem),
                    optional_add(group2, elem),
                )
            },
        )
        .0
        .to_string()
}

crate::aoc_tests! {
    task1: {
        (simple, "day01_simple.txt", "7")
        (complex, "day01_complex.txt", "1655")
    },
    task2: {
        (simple, "day01_simple.txt", "5")
        (complex, "day01_complex.txt", "1683")
    }
}
