use ndarray::{Array2, ArrayView2};

pub fn conv2d<T, F>(image: &Array2<T>, window_size: (usize, usize), kernel: F) -> Array2<T>
where
    F: FnMut(ArrayView2<T>) -> T,
{
    let result_data = image
        .windows(window_size)
        .into_iter()
        .map(kernel)
        .collect::<Vec<_>>();

    Array2::from_shape_vec(
        (
            image.dim().0 + 1 - window_size.0,
            image.dim().1 + 1 - window_size.1,
        ),
        result_data,
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn conv2d_padded<T, F>(
    image: &Array2<T>,
    window_size: (usize, usize),
    padding: T,
    kernel: F,
) -> Array2<T>
where
    F: FnMut(ArrayView2<T>) -> T,
    T: Clone,
{
    let padding_size = (window_size.0 / 2, window_size.1 / 2);

    let image_padded = Array2::from_shape_fn(
        (
            image.dim().0 + padding_size.0 * 2,
            image.dim().1 + padding_size.1 * 2,
        ),
        |coord| {
            if coord.0 >= padding_size.0 && coord.1 >= padding_size.1 {
                if let Some(pix) =
                    image.get(((coord.0 - padding_size.0), (coord.1 - padding_size.1)))
                {
                    pix.clone()
                } else {
                    padding.clone()
                }
            } else {
                padding.clone()
            }
        },
    );

    let result_data = image_padded
        .windows(window_size)
        .into_iter()
        .map(kernel)
        .collect::<Vec<_>>();

    assert_eq!(image.dim().0 * image.dim().1, result_data.len());

    Array2::from_shape_vec(image.dim(), result_data).unwrap()
}
