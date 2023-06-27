use std::{
    process,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use crate::{args::Args, cues::Cues};
use anyhow::{Ok, Result};
use clap::Parser;
use libmpv::{
    events::{Event, PropertyData},
    FileState, Mpv,
};
use libmpv_sys::mpv_end_file_reason;

pub struct App {
    pub args: Args,
    cues: Cues,
    mpv: Mpv,

    current_cue: AtomicUsize,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();

        let cues = Cues::from_file(&args.markers)?;
        println!("\n[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

        let mpv = Mpv::new().unwrap();
        mpv.pause().unwrap();

        for (key, val) in &args.mpv_setting {
            mpv.set_property(key, val.as_str()).unwrap();
        }

        if !args.audio {
            mpv.set_property("mute", "yes".to_owned()).unwrap();
        }

        mpv.playlist_load_files(&[(&args.video.to_string_lossy(), FileState::AppendPlay, None)])
            .unwrap();

        // osd-msg1=■
        // for (i, e) in cues.iter().enumerate() {
        //     let time = e.as_secs(60.0);
        //     mpv.set_property(&format!("chapter-list/{i}/time"), time as f64)
        //         .unwrap();
        //     // libmpv_sys::mpv_set_property(mpv.ctx.as_ptr(), format!("chapter-list/{i}/time").cs, format, data)
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
                Event::Shutdown => {
                    process::exit(0);
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
}
