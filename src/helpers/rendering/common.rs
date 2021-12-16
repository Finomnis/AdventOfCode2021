use imgref::ImgVec;
use ndarray::{Array2, Axis};
use rgb::RGBA8;

pub fn map_to_image(map: &Array2<impl super::ToColor>, scale: usize) -> ImgVec<RGBA8> {
    let mut pixels = vec![];

    for row in map.axis_iter(Axis(0)) {
        let mut row_pixels = vec![];
        for value in &row {
            let pixel = value.to_color();
            for _ in 0..scale {
                row_pixels.push(pixel);
            }
        }
        for _ in 0..scale {
            pixels.extend(row_pixels.iter());
        }
    }

    ImgVec::<RGBA8>::new(pixels, map.dim().1 * scale, map.dim().0 * scale)
}
