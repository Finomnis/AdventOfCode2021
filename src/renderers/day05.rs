use image::ImageBuffer;

use crate::solutions::day05::{VentLine, VentMap};

fn write_to_image(vent_map: &VentMap, name: &str) -> String {
    let image = ImageBuffer::from_fn(vent_map.width as u32, vent_map.height as u32, |x, y| {
        let value = vent_map.get(x as usize, y as usize);
        let luma = 255 - 50 * value;
        if luma > 255 {
            image::Luma([255u8])
        } else if luma < 0 {
            image::Luma([0u8])
        } else {
            image::Luma([luma as u8])
        }
    });

    let output_path = std::env::current_dir().unwrap().join(name);

    image.save(&output_path).unwrap();

    output_path.into_os_string().into_string().unwrap()
}

pub fn task1(input_data: &[VentLine]) -> Vec<String> {
    let mut vent_map = VentMap::new_auto_bounds(input_data);

    for line in input_data {
        vent_map.render_straight_line(line);
    }

    vec![write_to_image(&vent_map, "day05_task1.png")]
}
pub fn task2(input_data: &[VentLine]) -> Vec<String> {
    let mut vent_map = VentMap::new_auto_bounds(input_data);

    for line in input_data {
        vent_map.render_line(line);
    }

    vec![write_to_image(&vent_map, "day05_task2.png")]
}
