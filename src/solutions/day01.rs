pub fn parse_input(input_data: &str) -> Vec<u32> {
    input_data
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn task1(input_data: &[u32]) -> u32 {
    input_data
        .iter()
        .fold((0, u32::MAX), |(sum, prev), &elem| {
            (sum + if elem > prev { 1 } else { 0 }, elem)
        })
        .0
}

fn optional_add(a: Option<u32>, b: u32) -> Option<u32> {
    a.map(|prev| prev + b)
}

fn check_increased(prev: Option<u32>, curr: Option<u32>, elem: u32) -> u32 {
    let check = || {
        if curr? + elem > prev? {
            Some(())
        } else {
            None
        }
    };

    if check().is_some() {
        1
    } else {
        0
    }
}

pub fn task2(input_data: &[u32]) -> u32 {
    input_data
        .iter()
        .fold(
            (0u32, None, None, None),
            |(sum, group1, group2, group3), &elem| {
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
}

crate::aoc_tests! {
    task1: {
        (simple, "7")
        (complex, "1655")
    },
    task2: {
        (simple, "5")
        (complex, "1683")
    }
}
