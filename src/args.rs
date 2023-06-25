use std::path::PathBuf;

use clap::Parser;
use color_name::Color;

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

    /// Weather to play audio or not.
    /// Default is false.
    #[arg(short, long)]
    pub audio: bool,

    /// Background color.
    /// This will be shown on the borders of the video.
    #[arg(short, long, value_parser = parse_color)]
    pub background: Option<[u8; 3]>,
}

fn parse_color(raw: &str) -> Result<[u8; 3], String> {
    if raw.starts_with("#") && raw.len() == 7 {
        fn parse(raw: &str) -> Result<u8, String> {
            u8::from_str_radix(raw, 16).map_err(|e| e.to_string())
        }
        return Ok([parse(&raw[1..3])?, parse(&raw[3..5])?, parse(&raw[5..7])?]);
    }

    match Color.by_string(raw.to_owned()) {
        Ok(color) => Ok(color),
        Err(_) => Err(format!("Unknown color: {}", raw)),
    }
}
