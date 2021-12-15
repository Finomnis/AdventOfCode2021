use std::fs::File;

use gifski;
use ndarray::Array2;
use rgb::RGBA8;

use crate::{
    helpers::rendering::{ArrayCollector, ToColor},
    solutions::day15::{get_wrapped_risk, NextPathElement},
};

struct MapTile {
    risk: u8,
    considered: u8,
    solved: Option<(usize, usize)>,
}

impl ToColor for MapTile {
    fn to_color(&self) -> RGBA8 {
        let base_gray = 200 - self.risk * 15;
        let mut r = base_gray;
        let mut g = base_gray;
        let mut b = base_gray;

        if self.solved.is_some() {
            r -= 30;
            g -= 30;
            b += 55;
        } else if self.considered > 0 {
            r -= 30;
            g += 55;
            b -= 30;
        }

        RGBA8::new(r, g, b, 255)
    }
}

pub fn run_algorithm<FV>(map: &Array2<u8>, step_callback: FV)
where
    FV: FnMut(&NextPathElement, bool),
{
    let start = (0, 0);
    let size = map.dim();
    let goal = (size.0 * 5 - 1, size.1 * 5 - 1);

    crate::solutions::day15::find_shortest_path(
        start,
        goal,
        |&coord| {
            let wrapped_coord = (coord.0 % size.0, coord.1 % size.1);
            let tile = (coord.0 / size.0, coord.1 / size.1);
            if tile.0 >= 5 || tile.1 >= 5 {
                None
            } else {
                map.get(wrapped_coord)
                    .map(|&risk| ((risk as usize + tile.0 + tile.1 + 8) % 9 + 1) as u8)
            }
        },
        step_callback,
        false,
    )
    .unwrap();
}

pub fn generate_images(
    collector: Option<gifski::Collector>,
    map: &Array2<u8>,
    speedup: usize,
) -> usize {
    let mut collector = ArrayCollector::new(collector, 1);
    let time_step = 1.0 / 30.0;

    let original_map_size = map.dim();
    let map_size = (original_map_size.0 * 5, original_map_size.1 * 5);

    let mut image_data = Array2::from_shape_fn(map_size, |(x, y)| MapTile {
        risk: get_wrapped_risk(map, (x, y)).unwrap(),
        considered: 0,
        solved: None,
    });

    println!("Original size: {:?}", original_map_size);
    println!("Map size: {:?}", map_size);
    println!("Size: {:?}", image_data.dim());

    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);

    let mut skipped = 0;
    run_algorithm(map, |element, solved| {
        if solved {
            image_data[element.coord].solved = element.prev;
            skipped += 1;
            if skipped >= speedup {
                skipped = 0;
                collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);
            }
        } else {
            image_data[element.coord].considered += 1;
        }
    });

    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);

    collector.get_num_frames()
}

pub fn task2(input_data: &Array2<u8>) -> Vec<String> {
    let (collector, writer) = gifski::new(gifski::Settings {
        quality: 100,
        fast: false,
        repeat: gifski::Repeat::Infinite,
        width: None,
        height: None,
    })
    .unwrap();

    let speedup = 500;

    let num_frames = generate_images(None, input_data, speedup);

    let map = input_data.clone();
    let collector_thread = std::thread::spawn(move || {
        generate_images(Some(collector), &map, speedup);
    });

    let filename = std::env::current_dir()
        .unwrap()
        .join("aoc2021_day15_task2.gif");
    let file = File::create(&filename).unwrap();
    let mut progress = gifski::progress::ProgressBar::new(num_frames as u64);
    writer.write(&file, &mut progress).unwrap();

    collector_thread.join().unwrap();

    vec![filename.into_os_string().into_string().unwrap()]
}
