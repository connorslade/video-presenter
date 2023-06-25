use std::borrow::Cow;
use std::cell::RefCell;
use std::path::Path;
use std::{fs::File, io::BufReader};

use anyhow::{Ok, Result};
use clap::Parser;
use openh264::decoder::Decoder as H264Decoder;

use crate::{args::Args, cues::Cues, mp4_converter::Mp4BitstreamConverter};

pub struct App {
    pub args: Args,
    cues: Cues,
    decoder: RefCell<Decoder>,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let cues = Cues::from_file(&args.markers)?;
        println!("[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

        let decoder = Decoder::new(&args.video)?;

        Ok(Self {
            args,
            cues,
            decoder: RefCell::new(decoder),
        })
    }

    pub fn video_name(&self) -> Cow<'_, str> {
        self.args
            .video
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    }

    pub fn next_frame(&self) -> Result<Option<Vec<u8>>> {
        self.decoder.borrow_mut().next()
    }
}

struct Decoder {
    id: u32,
    fps: f64,
    frame: u32,

    buffer: Vec<u8>,
    header: mp4::Mp4Reader<BufReader<File>>,
    stream: Mp4BitstreamConverter,
    decoder: H264Decoder,
}

impl Decoder {
    fn new(file: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(file)?;
        let size = file.metadata()?.len();
        let reader = BufReader::new(file);

        let header = mp4::Mp4Reader::read_header(reader, size)?;
        let track = header
            .tracks()
            .values()
            .find(|x| x.media_type().unwrap() == mp4::MediaType::H264)
            .unwrap();
        let samples = track.sample_count();
        let id = track.track_id();
        let fps = track.frame_rate();

        println!("[*] Loaded Video");
        println!(" ├─ Brand: {}", header.major_brand());
        println!(" ├─ Duration: {:?}", header.duration());
        println!(" └─ Samples: {}", samples);

        let stream = Mp4BitstreamConverter::for_mp4_track(track).unwrap();
        let decoder = H264Decoder::new()?;
        let buffer = Vec::new();

        Ok(Self {
            id,
            fps,
            frame: 0,

            buffer,
            header,
            stream,
            decoder,
        })
    }

    fn next(&mut self) -> Result<Option<Vec<u8>>> {
        let track = self.header.tracks().get(&self.id).unwrap();
        let mut rgb = vec![0; track.width() as usize * track.height() as usize * 3];
        self.frame += 1;

        if self.frame > track.sample_count() {
            return Ok(None);
        }

        let sample = self.header.read_sample(self.id, self.frame)?.unwrap();
        self.stream.convert_packet(&sample.bytes, &mut self.buffer);

        if let Some(image) = self.decoder.decode(&self.buffer).unwrap() {
            image.write_rgb8(&mut rgb);
        }

        Ok(Some(rgb))
    }
}
