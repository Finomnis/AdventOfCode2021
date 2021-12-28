use std::{error::Error, fmt};
use std::{fmt::Debug, str::FromStr};

use ndarray::Array2;

#[derive(Debug)]
pub struct ParseError(pub String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

fn get_2d_matrix_width_height(input_data: &str) -> (usize, usize) {
    input_data
        .trim()
        .lines()
        .fold((usize::MAX, 0usize), |(width, height), row| {
            (std::cmp::min(width, row.trim().len()), height + 1)
        })
}

fn parse_as_2d_matrix_fn<T, F, G, E>(
    input_data: &str,
    border_size: usize,
    element: F,
    border: G,
) -> Result<Array2<T>, E>
where
    F: Fn(char) -> Result<T, E>,
    G: Fn() -> T,
{
    let input_data = input_data.trim();

    let (width, height) = get_2d_matrix_width_height(input_data);

    let mut parsed_data = input_data
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| element(ch).map(|el| Some(el)))
                .collect::<Result<Vec<_>, E>>()
        })
        .collect::<Result<Vec<_>, E>>()?;

    let matrix = Array2::from_shape_fn(
        (height + border_size * 2, width + border_size * 2),
        |(y, x)| {
            if y < border_size || x < border_size {
                border()
            } else if let Some(elem) = parsed_data
                .get_mut(y - border_size)
                .and_then(|row| row.get_mut(x - border_size))
            {
                elem.take().unwrap_or_else(|| {
                    unreachable!("Every element of parsed_data should only be taken once!")
                })
            } else {
                border()
            }
        },
    );

    Ok(matrix)
}

#[allow(dead_code)]
pub fn parse_as_2d_matrix_with_border<T: FromStr>(
    input_data: &str,
    border_size: usize,
) -> Result<Array2<Option<T>>, T::Err> {
    parse_as_2d_matrix_fn(
        input_data,
        border_size,
        |c| format!("{}", c).parse::<T>().map(|e| Some(e)),
        || None,
    )
}

#[allow(dead_code)]
pub fn parse_as_2d_matrix_with_filled_border<T: Clone + FromStr>(
    input_data: &str,
    border_size: usize,
    border_value: T,
) -> Result<Array2<T>, T::Err> {
    parse_as_2d_matrix_fn(
        input_data,
        border_size,
        |c| format!("{}", c).parse::<T>(),
        || border_value.clone(),
    )
}

#[allow(dead_code)]
pub fn parse_as_2d_matrix<T: FromStr>(input_data: &str) -> Result<Array2<T>, T::Err> {
    parse_as_2d_matrix_fn(
        input_data,
        0,
        |c| format!("{}", c).parse::<T>(),
        || unreachable!(),
    )
}
