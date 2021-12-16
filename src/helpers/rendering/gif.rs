use std::fs::File;

use ndarray::Array2;

pub struct GifskyCollector {
    collector: gifski::Collector,
    scale: usize,
    sleep_time: f64,
    frames_count: usize,
}

pub struct GifskyWriter {
    writer: gifski::Writer,
}

impl super::Writer for GifskyWriter {
    fn write(self, filename: &impl AsRef<std::path::Path>, frame_count_hint: Option<usize>) {
        let file = File::create(filename).unwrap();
        if let Some(frame_count_hint) = frame_count_hint {
            let mut progress = gifski::progress::ProgressBar::new(frame_count_hint as u64);
            self.writer.write(&file, &mut progress).unwrap();
        } else {
            let mut progress = gifski::progress::NoProgress {};
            self.writer.write(&file, &mut progress).unwrap();
        }
    }
}

pub fn create_gifski(scale: usize, sleep_time: f64) -> (impl super::Collector, impl super::Writer) {
    let (collector, writer) = gifski::new(gifski::Settings {
        quality: 100,
        fast: false,
        repeat: gifski::Repeat::Infinite,
        width: None,
        height: None,
    })
    .unwrap();

    (
        GifskyCollector {
            collector,
            scale,
            sleep_time,
            frames_count: 0,
        },
        GifskyWriter { writer },
    )
}

impl super::Collector for GifskyCollector {
    fn add_frame(&mut self, data: &Array2<impl super::ToColor>, timestamp: f64) {
        let img = super::common::map_to_image(data, self.scale);
        self.collector
            .add_frame_rgba(self.frames_count, img, timestamp + self.sleep_time)
            .unwrap();
        self.frames_count += 1;
    }
    fn get_num_frames(&self) -> usize {
        self.frames_count
    }
}
