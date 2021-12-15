use std::fs::File;

use gifski;
use ndarray::Array2;
use rgb::RGBA8;

use crate::{
    helpers::rendering::{ArrayCollector, ToColor},
    solutions::day15::get_wrapped_risk,
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

struct RenderConfig<R> {
    scale: usize,
    map_read: R,
    map_size: (usize, usize),
    start: (usize, usize),
    goal: (usize, usize),
    astar: bool,
    speedup: usize,
    speedup_end: usize,
}

fn generate_images<R>(collector: Option<gifski::Collector>, config: &RenderConfig<R>) -> usize
where
    R: Fn((usize, usize)) -> Option<u8>,
{
    let mut collector = ArrayCollector::new(collector, config.scale, 4.0);
    let time_step = 1.0 / 30.0;

    let mut image_data = Array2::from_shape_fn(config.map_size, |(x, y)| MapTile {
        risk: (config.map_read)((x, y)).unwrap(),
        considered: 0,
        solved: None,
        part_of_solution: false,
    });

    image_data[config.start].part_of_solution = true;
    image_data[config.goal].part_of_solution = true;

    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);

    // Run solving algorithm
    let mut skipped = 0;
    crate::solutions::day15::find_shortest_path(
        config.start,
        config.goal,
        &config.map_read,
        |element, solved| {
            if solved {
                image_data[element.coord].solved = element.prev;
                skipped += 1;
                if skipped >= config.speedup {
                    skipped = 0;
                    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);
                }
            } else {
                image_data[element.coord].considered += 1;
            }
        },
        config.astar,
    )
    .unwrap();

    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);

    // Animate path trace
    let mut current_path_coord = config.goal;
    skipped = 0;
    loop {
        let current_path_element = &mut image_data[current_path_coord];
        current_path_element.part_of_solution = true;

        if let Some(next) = current_path_element.solved {
            current_path_coord = next;
        } else {
            break;
        }

        skipped += 1;
        if skipped >= config.speedup_end {
            skipped = 0;
            collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);
        }
    }

    collector.add_frame(&image_data, time_step * collector.get_num_frames() as f64);

    collector.get_num_frames()
}

pub fn task1(input_data: &Array2<u8>) -> Vec<String> {
    let (collector, writer) = gifski::new(gifski::Settings {
        quality: 100,
        fast: false,
        repeat: gifski::Repeat::Infinite,
        width: None,
        height: None,
    })
    .unwrap();

    let map = input_data.clone();
    let map_read = move |coord| map.get(coord).cloned();
    let map_size = input_data.dim();

    let config = RenderConfig {
        scale: 3,
        map_read,
        map_size,
        start: (0, 0),
        goal: (map_size.0 - 1, map_size.1 - 1),
        astar: false,
        speedup: 30,
        speedup_end: 2,
    };

    let num_frames = generate_images(None, &config);

    let collector_thread = std::thread::spawn(move || {
        generate_images(Some(collector), &config);
    });

    let filename = std::env::current_dir()
        .unwrap()
        .join("aoc2021_day15_task1.gif");
    let file = File::create(&filename).unwrap();
    let mut progress = gifski::progress::ProgressBar::new(num_frames as u64);
    writer.write(&file, &mut progress).unwrap();

    collector_thread.join().unwrap();

    vec![filename.into_os_string().into_string().unwrap()]
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

    let map = input_data.clone();
    let map_read = move |coord| get_wrapped_risk(&map, coord);
    let map_size = (input_data.dim().0 * 5, input_data.dim().1 * 5);

    let config = RenderConfig {
        scale: 1,
        map_read,
        map_size,
        start: (0, 0),
        goal: (map_size.0 - 1, map_size.1 - 1),
        astar: false,
        speedup: 1000,
        speedup_end: 10,
    };

    let num_frames = generate_images(None, &config);

    let collector_thread = std::thread::spawn(move || {
        generate_images(Some(collector), &config);
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
