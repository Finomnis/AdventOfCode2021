use std::fs::File;

use gifski;
use imgref::ImgVec;
use ndarray::{Array2, Axis};
use rgb::RGBA8;

use crate::solutions::day11::update_map;

fn map_to_image(map: &Array2<u8>, scale: usize) -> ImgVec<RGBA8> {
    let mut pixels = vec![];

    for row in map.axis_iter(Axis(0)) {
        let mut row_pixels = vec![];
        for &value in row {
            let pixel = if value == 0 {
                RGBA8::new(255, 255, 128, 255)
            } else {
                RGBA8::new((value - 1) * 16, (value - 1) * 16, (value - 1) * 16, 255)
            };

            for _ in 0..scale {
                row_pixels.push(pixel.clone());
            }
        }
        for _ in 0..scale {
            pixels.extend(row_pixels.iter());
        }
    }

    ImgVec::<RGBA8>::new(pixels, map.dim().1 * scale, map.dim().0 * scale)
}

pub fn task2(input_data: &Array2<u8>) -> Vec<String> {
    let mut map = input_data.clone();
    let scale = 20;
    let time_step = 1.0 / 10.0;

    let (mut collector, writer) = gifski::new(gifski::Settings {
        quality: 100,
        fast: false,
        repeat: gifski::Repeat::Infinite,
        width: None,
        height: None,
    })
    .unwrap();

    let collector_thread = std::thread::spawn(move || {
        let mut index = 0;

        collector
            .add_frame_rgba(index, map_to_image(&map, scale), index as f64 * time_step)
            .unwrap();
        index += 1;

        while update_map(&mut map) != map.len() {
            collector
                .add_frame_rgba(index, map_to_image(&map, scale), index as f64 * time_step)
                .unwrap();
            index += 1;
            println!("Frame {} ...", index);
        }

        for _ in 0..100 {
            update_map(&mut map);
            collector
                .add_frame_rgba(index, map_to_image(&map, scale), index as f64 * time_step)
                .unwrap();
            index += 1;
        }
    });

    let filename = std::env::current_dir().unwrap().join("aoc2021_day11.gif");
    let file = File::create(&filename).unwrap();
    let mut progress = gifski::progress::NoProgress {};
    writer.write(&file, &mut progress).unwrap();

    collector_thread.join().unwrap();

    vec![filename.into_os_string().into_string().unwrap()]
}
