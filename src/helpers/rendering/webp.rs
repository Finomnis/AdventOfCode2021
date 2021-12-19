use imgref::ImgVec;
use indicatif::ProgressBar;
use ndarray::Array2;
use rgb::RGBA8;
use webp_animation::Encoder;

use super::common::map_to_image;

pub struct WebpCollector {
    scale: usize,
    frames_count: usize,
    sender: std::sync::mpsc::SyncSender<(ImgVec<RGBA8>, f64)>,
}

pub struct WebpWriter {
    sleep_time: f64,
    receiver: std::sync::mpsc::Receiver<(ImgVec<RGBA8>, f64)>,
}

impl super::Writer for WebpWriter {
    fn write(self, filename: &impl AsRef<std::path::Path>, frame_count_hint: Option<usize>) {
        let progress = ProgressBar::new(frame_count_hint.unwrap_or(0) as u64 + 1);
        let mut frame_iter = self.receiver.iter();
        if let Some((initial_frame, initial_timestamp)) = frame_iter.next() {
            let mut encoder =
                Encoder::new((initial_frame.width() as u32, initial_frame.height() as u32))
                    .unwrap();
            let bytes = initial_frame
                .pixels()
                .flat_map(|pixel| [pixel.r, pixel.g, pixel.b, pixel.a].into_iter())
                .collect::<Vec<_>>();

            let mut newest_timestamp = (initial_timestamp * 1000.0).round() as i32;
            encoder.add_frame(&bytes, newest_timestamp).unwrap();
            progress.inc(1);

            for (frame, timestamp) in frame_iter {
                let bytes = frame
                    .pixels()
                    .flat_map(|pixel| [pixel.r, pixel.g, pixel.b, pixel.a].into_iter())
                    .collect::<Vec<_>>();
                newest_timestamp = (timestamp * 1000.0).round() as i32;
                encoder.add_frame(&bytes, newest_timestamp).unwrap();
                progress.inc(1);
            }

            let webp_data = encoder
                .finalize(newest_timestamp + (self.sleep_time * 1000.0).round() as i32)
                .unwrap();
            std::fs::write(filename, webp_data).unwrap();
            progress.inc(1);
        }
        progress.finish();
    }
}

pub fn create_webp(scale: usize, sleep_time: f64) -> (impl super::Collector, impl super::Writer) {
    let (sender, receiver) = std::sync::mpsc::sync_channel(4);
    (
        WebpCollector {
            sender,
            scale,
            frames_count: 0,
        },
        WebpWriter {
            receiver,
            sleep_time,
        },
    )
}

impl super::Collector for WebpCollector {
    fn add_frame(&mut self, data: &Array2<impl super::ToColor>, timestamp: f64) {
        let img = map_to_image(data, self.scale);
        self.sender.send((img, timestamp)).unwrap();
        self.frames_count += 1;
    }
    fn get_num_frames(&self) -> usize {
        self.frames_count
    }
}
