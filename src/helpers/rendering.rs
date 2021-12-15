use imgref::ImgVec;
use ndarray::{Array2, Axis};
use rgb::RGBA8;

pub trait ToColor {
    fn to_color(&self) -> RGBA8;
}

pub struct ArrayCollector {
    collector: Option<gifski::Collector>,
    scale: usize,
    num_frames: usize,
}

fn map_to_image<T>(map: &Array2<T>, scale: usize) -> ImgVec<RGBA8>
where
    T: ToColor,
{
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

impl ArrayCollector {
    pub fn new(collector: Option<gifski::Collector>, scale: usize) -> Self {
        Self {
            collector,
            scale,
            num_frames: 0,
        }
    }

    pub fn add_frame<T>(&mut self, data: &Array2<T>, timestamp: f64)
    where
        T: ToColor,
    {
        if let Some(collector) = self.collector.as_mut() {
            let img = map_to_image(data, self.scale);
            collector
                .add_frame_rgba(self.num_frames, img, timestamp)
                .unwrap()
        }
        self.num_frames += 1;
    }

    pub fn get_num_frames(&self) -> usize {
        self.num_frames
    }
}
