use ndarray::Array2;
use rgb::RGBA8;

use crate::{
    helpers::rendering::{Collector, FramesCounter, Renderers, ToColor, Writer},
    solutions::day25::{move_bottom, move_right, FloorTile},
};

struct MapTile {
    risk: u8,
    considered: u8,
    solved: Option<(usize, usize)>,
    part_of_solution: bool,
}

impl ToColor for MapTile {
    fn to_color(&self) -> RGBA8 {
        let base_gray = 200 - self.risk * 12;
        let mut r = base_gray;
        let mut g = base_gray;
        let mut b = base_gray;

        if self.part_of_solution {
            r = 255;
            g -= 60;
            b -= 60;
            g /= 2;
            b /= 2;
        } else if self.solved.is_some() {
            r -= 60;
            g -= 60;
            b += 55;
        } else if self.considered > 0 {
            r -= 80;
            g += 55;
            b -= 80;
        }

        RGBA8::new(r, g, b, 255)
    }
}

impl ToColor for FloorTile {
    fn to_color(&self) -> RGBA8 {
        match self {
            FloorTile::MoveRight => RGBA8::new(255, 0, 0, 255),
            FloorTile::MoveBottom => RGBA8::new(0, 0, 255, 255),
            FloorTile::Empty => RGBA8::new(255, 255, 255, 255),
        }
    }
}

fn generate_images(mut collector: impl Collector, input_data: &Array2<FloorTile>) -> usize {
    let time_step = 1.0 / 60.0;

    let mut seafloor = input_data.clone();

    collector.add_frame(&seafloor, time_step * collector.get_num_frames() as f64);

    while {
        let moved_right = move_right(&mut seafloor);
        collector.add_frame(&seafloor, time_step * collector.get_num_frames() as f64);
        let moved_bottom = move_bottom(&mut seafloor);
        collector.add_frame(&seafloor, time_step * collector.get_num_frames() as f64);
        moved_right || moved_bottom
    } {}

    collector.get_num_frames()
}

pub fn task1(input_data: &Array2<FloorTile>) -> Vec<String> {
    let num_frames = generate_images(FramesCounter::new(), input_data);

    let (collector, writer) = Renderers::create_webp_renderer(3, 4.0);

    let input_data = input_data.clone();
    let collector_thread = std::thread::spawn(move || {
        generate_images(collector, &input_data);
    });

    let filename = std::env::current_dir().unwrap().join("aoc2021_day25.webp");
    writer.write(&filename, Some(num_frames));

    collector_thread.join().unwrap();

    vec![filename.into_os_string().into_string().unwrap()]
}
