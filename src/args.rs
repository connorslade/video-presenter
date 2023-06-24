use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path of the video file to play.
    #[arg()]
    pub video: PathBuf,

    /// Marker file (csv or txt) exported from premiere pro.
    /// Blue markers are used to mark sections.
    #[arg()]
    pub markers: PathBuf,
}
