#![feature(decl_macro)]

use std::{sync::Arc, thread};

use anyhow::Result;

mod app;
mod args;
mod cues;
mod misc;
use app::App;
use libmpv::{FileState, Mpv};

fn main() -> Result<()> {
    let app = Arc::new(App::new()?);

    let mpv = Mpv::new().unwrap();

    mpv.playlist_load_files(&[(
        &app.args.video.to_string_lossy(),
        FileState::AppendPlay,
        None,
    )])
    .unwrap();

    thread::park();
    Ok(())
}
