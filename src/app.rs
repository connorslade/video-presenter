use std::borrow::Cow;
use std::cell::{RefCell, RefMut};
use std::path::Path;
use std::{fs::File, io::BufReader};

use anyhow::{Ok, Result};
use clap::Parser;
use openh264::decoder::Decoder as H264Decoder;

use crate::misc::time::Time;
use crate::{args::Args, cues::Cues, mp4_converter::Mp4BitstreamConverter};

pub struct App {
    pub args: Args,
    cues: Cues,
    decoder: RefCell<Decoder>,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let decoder = Decoder::new(&args.video)?;

        let cues = Cues::from_file(&args.markers)?;
        println!("\n[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

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

    pub fn decoder(&self) -> RefMut<'_, Decoder> {
        self.decoder.borrow_mut()
    }
}

pub struct Decoder {
    id: u32,
    pub fps: f64,
    sample: u32,

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

        let duration = Time::from_duration(header.duration(), fps as f32);
        println!("[*] Loaded Video");
        println!(" ├─ Brand: {}", header.major_brand());
        println!(" ├─ Duration: {}", duration);
        println!(" └─ Samples: {}", samples);

        let stream = Mp4BitstreamConverter::for_mp4_track(track).unwrap();
        let decoder = H264Decoder::new()?;
        let buffer = Vec::new();

        Ok(Self {
            id,
            fps,
            sample: 0,

            buffer,
            header,
            stream,
            decoder,
        })
    }

    pub fn set_sample(&mut self, sample: u32) -> Result<()> {
        if sample > self.header.tracks().get(&self.id).unwrap().sample_count() {
            return Err(anyhow::anyhow!("Frame out of bounds"));
        }

        self.sample = sample;
        Ok(())
    }

    pub fn next_frame(&mut self) -> Result<Option<Vec<u8>>> {
        let track = self.header.tracks().get(&self.id).unwrap();
        let mut rgb = vec![0; track.width() as usize * track.height() as usize * 3];
        self.sample += 1;

        if self.sample > track.sample_count() {
            return Ok(None);
        }

        self.buffer.clear();
        let sample = self.header.read_sample(self.id, self.sample)?.unwrap();
        self.stream.convert_packet(&sample.bytes, &mut self.buffer);

        let image = self.decoder.decode(&self.buffer)?.unwrap();
        image.write_rgb8(&mut rgb);

        Ok(Some(rgb))
    }

    pub fn dimensions(&self) -> (u16, u16) {
        let track = self.header.tracks().get(&self.id).unwrap();
        (track.width(), track.height())
    }
}
