#![feature(decl_macro)]

use std::sync::Arc;

use anyhow::Result;

mod app;
mod args;
mod cues;
mod misc;
mod mp4_converter;
mod window;
use app::App;

fn main() -> Result<()> {
    let app = Arc::new(App::new()?);
    window::init(app);

    Ok(())
}
