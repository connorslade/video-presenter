use std::{
    borrow::Cow,
    cell::UnsafeCell,
    fs, result,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use clap::Parser;
use libmpv::{
    events::{Event, PropertyData},
    FileState, Mpv,
};

use crate::{args::Args, cues::Cues};

/// The main application state.
pub struct App {
    pub args: Args,
    pub cues: Cues,
    pub mpv: Mpv,

    pub fps: UnsafeCell<Option<f64>>,
    pub current_cue: AtomicUsize,
}

// idk girl
unsafe impl Send for App {}
unsafe impl Sync for App {}

impl App {
    pub fn new(wid: u64) -> Result<Self> {
        // Parse command line arguments with clap
        let args = Args::parse();

        // Loads cues from specified file
        let contents = fs::read_to_string(&args.markers)?;
        let cues = Cues::from_str(&contents)?;
        println!("\n[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

        // Instantiate libmpv
        let mpv = Mpv::new().unwrap();
        mpv.pause().unwrap();

        // Set default mpv settings
        // These can be overridden by the user
        mpv.set_property("wid", wid as i64).unwrap();
        mpv.set_property("keep-open", true).unwrap();
        mpv.set_property("keep-open", true).unwrap();
        mpv.set_property("osd-bar", false).unwrap();
        mpv.set_property("osc", "").unwrap();
        if !args.audio {
            mpv.set_property("mute", "yes".to_owned()).unwrap();
        }

        // Allow users to pass custom settings to mpv
        for (key, val) in &args.mpv_setting {
            mpv.set_property(key, val.as_str()).unwrap();
        }

        // Load the intended video
        // Not sure if this is the most concise way to do this, but its working
        mpv.playlist_load_files(&[(&args.video.to_string_lossy(), FileState::AppendPlay, None)])
            .unwrap();

        Ok(Self {
            args,
            cues,
            mpv,

            current_cue: AtomicUsize::default(),
            fps: UnsafeCell::new(None),
        })
    }

    pub fn event_loop(&self) -> ! {
        // Listen for the playback-time event
        // Using this we can pause playback at each cue point

        let mut events = self.mpv.create_event_context();
        events
            .observe_property("playback-time", libmpv::Format::Double, 0)
            .unwrap();

        loop {
            // Not sure why the api is like this
            // The timeout is completely arbitrary, but I saw 1000 being used in the examples, so
            let event = match events.wait_event(1000.0) {
                Some(e) => e.unwrap(),
                None => continue,
            };

            match event {
                // If the file has loaded, get its FPS and print some info
                Event::FileLoaded => {
                    let fps = self.mpv.get_property::<f64>("container-fps").unwrap();
                    unsafe { *self.fps.get() = Some(fps) };

                    #[rustfmt::skip]
                    const INFO: &[(&str, &str)] = &[
                        ("Container FPS", "container-fps"),
                        ("Duration",      "duration"),
                        ("Video Format",  "video-format"),
                        ("MPV Version",   "mpv-version"),
                    ];

                    println!("\n[*] Loaded video `{}`", self.video_name());
                    for (i, (name, val)) in INFO.iter().enumerate() {
                        let val = self.mpv.get_property::<String>(val).unwrap();
                        println!(
                            " {}─ {}: {}",
                            if i + 1 == INFO.len() { "└" } else { "├" },
                            name,
                            val
                        );
                    }
                }
                // If the playback-time has changed, check if we need to pause
                Event::PropertyChange {
                    name: "playback-time",
                    change: PropertyData::Double(val),
                    ..
                } => {
                    let current = self.cues.current(val, self.fps());
                    let old = self.current_cue.load(Ordering::Relaxed);

                    if current > old {
                        self.mpv.pause().unwrap();
                        self.current_cue.store(current, Ordering::Relaxed);
                    }
                }
                _ => {}
            }
        }
    }

    // == MPV Interaction ==

    /// Seek forward to the next cue point
    pub fn seek_f(&self) -> result::Result<(), libmpv::Error> {
        let cue = self.current_cue.load(Ordering::Relaxed) + 1;
        let time = self.cues.get(cue);

        // If we are at the end of the video, do nothing
        if cue > self.cues.len() + 1 {
            return Ok(());
        }

        // breon i used an else
        // are you proud of me??
        if time.is_end() {
            self.current_cue
                .store(self.cues.len() + 1, Ordering::Relaxed);
            self.mpv.seek_percent_absolute(100)
        } else {
            self.current_cue.store(cue, Ordering::Relaxed);
            self.mpv.seek_absolute(time.as_secs(self.fps()))
        }
    }

    /// Seek backward to the previous cue point
    pub fn seek_r(&self) -> result::Result<(), libmpv::Error> {
        let cue = self.current_cue.load(Ordering::Relaxed).saturating_sub(1);
        let time = self.cues.get(cue);
        self.info(format!("#{cue}"));

        self.current_cue.store(cue, Ordering::Relaxed);
        self.mpv.seek_absolute(time.as_secs(self.fps()))
    }

    /// Automatically update the current cue point based on the playback time
    /// This is not always accurate, because mpv doesn't supply a frame number but a value in seconds.
    /// So rounding errors and such can cause one cue point to be interpreted as another.
    pub fn auto_cue(&self) {
        let time = self.mpv.get_property::<f64>("playback-time").unwrap();
        self.current_cue
            .store(self.cues.current(time, self.fps()), Ordering::Relaxed);
    }

    /// Display a message on the screen using mpv's OSD.
    /// By default it will last for one second.
    pub fn info(&self, msg: impl AsRef<str>) {
        self.mpv
            .command(
                "show-text",
                &[&format!(r#""{}""#, msg.as_ref().replace('\"', "\\\""))],
            )
            .unwrap();
    }

    // == Info getters ==

    /// Get the name of the video, from the file name
    /// This is used to display the video name in the window title
    pub fn video_name(&self) -> Cow<'_, str> {
        self.args.video.file_name().unwrap().to_string_lossy()
    }

    /// Get the FPS of the video
    pub fn fps(&self) -> f64 {
        unsafe { *self.fps.get() }.unwrap_or(60.0)
    }
}
