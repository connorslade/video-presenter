use std::{
    borrow::Cow,
    result,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{args::Args, cues::Cues};
use anyhow::Result;
use clap::Parser;
use libmpv::{
    events::{Event, PropertyData},
    FileState, Mpv,
};

pub struct App {
    pub args: Args,
    pub cues: Cues,
    pub mpv: Mpv,

    current_cue: AtomicUsize,
}

impl App {
    pub fn new(wid: u64) -> Result<Self> {
        let args = Args::parse();

        let cues = Cues::from_file(&args.markers)?;
        println!("\n[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

        let mpv = Mpv::new().unwrap();
        mpv.pause().unwrap();

        // Set default settings
        mpv.set_property("wid", wid as i64).unwrap();
        mpv.set_property("keep-open", true).unwrap();
        if !args.audio {
            mpv.set_property("mute", "yes".to_owned()).unwrap();
        }

        // Allow users to pass custom settings to libmpv
        for (key, val) in &args.mpv_setting {
            mpv.set_property(key, val.as_str()).unwrap();
        }

        // Load the intended video
        mpv.playlist_load_files(&[(&args.video.to_string_lossy(), FileState::AppendPlay, None)])
            .unwrap();

        // osd-msg1=■
        // for (i, e) in cues.iter().enumerate() {
        //     let time = e.as_secs(60.0);
        //     mpv.set_property(&format!("chapter-list/{i}/time"), time as f64)
        //         .unwrap();
        // }

        Ok(Self {
            args,
            cues,
            mpv,

            current_cue: AtomicUsize::default(),
        })
    }

    pub fn event_loop(&self) -> ! {
        let mut events = self.mpv.create_event_context();
        events
            .observe_property("playback-time", libmpv::Format::Double, 0)
            .unwrap();

        loop {
            let event = match events.wait_event(600.0) {
                Some(e) => e.unwrap(),
                None => continue,
            };

            match event {
                Event::Seek => {
                    let time = self.mpv.get_property::<f64>("playback-time").unwrap();
                    self.current_cue
                        .store(self.cues.current(time), Ordering::Relaxed);
                }
                Event::PropertyChange {
                    name: "playback-time",
                    change: PropertyData::Double(val),
                    ..
                } => {
                    let current = self.cues.current(val);
                    let old = self.current_cue.load(Ordering::Relaxed);

                    if current > old {
                        self.mpv.pause().unwrap();
                    }
                    self.current_cue.store(current, Ordering::Relaxed);
                }
                _ => {}
            }
        }
    }

    pub fn seek_f(&self) -> result::Result<(), libmpv::Error> {
        let cue = self.current_cue.load(Ordering::Relaxed) + 1;
        let time = self.cues.get(cue);

        if time.is_end() {
            self.mpv.seek_percent(100)
        } else {
            self.mpv.seek_absolute(time.as_secs(60.) as f64)
        }
    }

    pub fn seek_r(&self) -> result::Result<(), libmpv::Error> {
        let cue = self.current_cue.load(Ordering::Relaxed).saturating_sub(1);
        let time = self.cues.get(cue);
        self.mpv.seek_absolute(time.as_secs(60.) as f64)
    }

    pub fn video_name(&self) -> Cow<'_, str> {
        self.args.video.file_name().unwrap().to_string_lossy()
    }
}
