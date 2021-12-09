use std::{error::Error, fmt};
use std::{fmt::Debug, str::FromStr};

use ndarray::{s, Array, Array2, Axis};

#[derive(Debug)]
pub struct ParseError(pub String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

#[allow(dead_code)]
pub fn parse_as_2d_matrix_with_border<T: Clone + FromStr + Debug>(
    input_data: &str,
    border_size: usize,
) -> Result<Array2<Option<T>>, ParseError> {
    let input_data = input_data.trim();

    let (width, height) = input_data
        .lines()
        .fold((usize::MAX, 0usize), |(width, height), row| {
            (std::cmp::min(width, row.trim().len()), height + 1)
        });

    let mut matrix: Array2<Option<T>> =
        Array::from_elem((height + border_size * 2, width + border_size * 2), None);

    let mut borderless = matrix.slice_mut(s![
        border_size..height + border_size,
        border_size..=width + border_size
    ]);

    input_data
        .lines()
        .zip(borderless.axis_iter_mut(Axis(0)))
        .try_for_each(|(in_line, mut out_line)| {
            in_line
                .trim()
                .chars()
                .map(|c| c.to_string())
                .zip(out_line.iter_mut())
                .try_for_each(|(in_el, out_el)| {
                    *out_el = Some(
                        in_el
                            .parse::<T>()
                            .map_err(|_| ParseError(format!("Unable to parse '{}'!", in_el)))?,
                    );
                    Ok(())
                })
        })?;

    Ok(matrix)
}
