use std::path::Path;

use ndarray::Array2;
use rgb::RGBA8;

mod common;
mod gif;
mod webp;

pub trait ToColor {
    fn to_color(&self) -> RGBA8;
}

pub trait Writer {
    fn write(self, filename: &impl AsRef<Path>, frame_count_hint: Option<usize>);
}

pub trait Collector {
    fn add_frame(&mut self, data: &Array2<impl ToColor>, timestamp: f64);
    fn get_num_frames(&self) -> usize;
}

pub struct FramesCounter {
    count: usize,
}

impl Collector for FramesCounter {
    fn add_frame(&mut self, _data: &Array2<impl ToColor>, _timestamp: f64) {
        self.count += 1;
    }
    fn get_num_frames(&self) -> usize {
        self.count
    }
}

impl FramesCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

pub struct Renderers {
    _inconstructible: (),
}

#[allow(dead_code)]
impl Renderers {
    pub fn create_gif_renderer(scale: usize, sleep_time: f64) -> (impl Collector, impl Writer) {
        self::gif::create_gifski(scale, sleep_time)
    }
    pub fn create_webp_renderer(scale: usize, sleep_time: f64) -> (impl Collector, impl Writer) {
        self::webp::create_webp(scale, sleep_time)
    }
}
