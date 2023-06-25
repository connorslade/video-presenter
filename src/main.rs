#![feature(decl_macro)]

use std::{fs::File, io::BufReader, sync::Arc, time::Instant};

mod app;
mod args;
mod cues;
mod misc;
mod mp4_bitstream_converter;
mod window;

use app::App;
use openh264::decoder::Decoder;

use crate::mp4_bitstream_converter::Mp4BitstreamConverter;

fn main() {
    let app = Arc::new(App::new());
    // window::init(app);

    let file = File::open(r"Main_1.mp4").unwrap();
    let size = file.metadata().unwrap().len();
    let reader = BufReader::new(file);

    let mut header = mp4::Mp4Reader::read_header(reader, size).unwrap();

    println!("Brand: {}", header.major_brand());
    println!("Duration: {:?}", header.duration());
    println!("Timescale: {:?}", header.timescale());

    let track = header
        .tracks()
        .values()
        .find(|x| x.media_type().unwrap() == mp4::MediaType::H264)
        .unwrap();
    let samples = track.sample_count();
    let id = track.track_id();
    let fps = track.frame_rate();
    println!("Media Type: {}", track.media_type().unwrap());
    println!("Samples: {}", samples);

    let mut stream = Mp4BitstreamConverter::for_mp4_track(track).unwrap();
    let mut decoder = Decoder::new().unwrap();

    let mut buffer = Vec::new();
    let mut rgb = vec![0; track.width() as usize * track.height() as usize * 3];

    let start = Instant::now();
    let mut tmp = Instant::now();
    let mut frames = 0;
    let mut total_frames = 0;

    for i in 1..track.sample_count() + 1 {
        let Some(sample) = header.read_sample(id, i).unwrap() else {
            continue;
        };

        stream.convert_packet(&sample.bytes, &mut buffer);

        if let Some(image) = decoder.decode(&buffer).unwrap() {
            image.write_rgb8(&mut rgb);

            frames += 1;
            total_frames += 1;
            if tmp.elapsed().as_secs() >= 1 {
                println!("FPS: {}", frames);
                frames = 0;
                tmp = Instant::now();
            }
        }
    }

    let total_fps = total_frames as f64 / start.elapsed().as_secs_f64();
    println!(
        "Total FPS: {} ({:.2}% realtime)",
        total_fps,
        total_fps / fps * 100.
    );
}
