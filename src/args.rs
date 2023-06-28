use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path of the video file to play.
    #[arg()]
    pub video: PathBuf,

    /// Marker file (csv or txt) exported from premiere pro.
    #[arg()]
    pub markers: PathBuf,

    /// Passes a setting value directly to mpv.
    #[arg(short, long, value_parser = parse_setting)]
    pub mpv_setting: Vec<(String, String)>,

    // == Shortcut settings ==
    /// Weather to play audio or not.
    /// Default is false.
    #[arg(short, long)]
    pub audio: bool,
}

/// Parses a mpv setting into the key and value.
/// If there is no assignment, the value is set to an empty string.
fn parse_setting(raw: &str) -> Result<(String, String), String> {
    Ok(raw
        .split_once('=')
        .map(|x| (x.0.to_owned(), x.1.to_owned()))
        .unwrap_or_else(|| (raw.to_owned(), String::new())))
}
