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

    for _ in 0..3000 {
        app.decoder().next_frame().unwrap().unwrap();
    }

    let (width, height) = app.decoder().dimensions();
    image::RgbImage::from_raw(
        width as u32,
        height as u32,
        app.decoder().next_frame().unwrap().unwrap(),
    )
    .unwrap()
    .save("test.jpg")
    .unwrap();

    // window::init(app);

    Ok(())
}
