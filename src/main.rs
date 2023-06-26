#![feature(decl_macro)]

use std::{sync::Arc, thread, time::Duration};

use anyhow::Result;

mod app;
mod args;
mod cues;
mod misc;
mod mp4_converter;
mod window;
use app::App;
use libmpv::{FileState, Mpv};

fn main() -> Result<()> {
    let app = Arc::new(App::new()?);

    // window::init(app);

    let mpv = Mpv::new().unwrap();

    mpv.playlist_load_files(&[(
        "https://www.youtube.com/watch?v=DLzxrzFCyOs",
        FileState::AppendPlay,
        None,
    )])
    .unwrap();

    thread::sleep(Duration::from_secs(3));

    mpv.set_property("volume", 25).unwrap();

    thread::sleep(Duration::from_secs(5));

    // Trigger `Event::EndFile`.
    mpv.playlist_next_force().unwrap();

    Ok(())
}
