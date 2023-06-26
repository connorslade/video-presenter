use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use minifb::{Key, Window, WindowOptions};

use crate::app::App;

pub fn init(app: Arc<App>) -> Result<()> {
    let (width, height) = app.decoder().dimensions();

    let mut buffer: Vec<u32> = Vec::with_capacity(width as usize * height as usize);

    let mut window = Window::new(
        &format!("video-presenter \u{2013} {}", app.video_name()),
        width as usize,
        height as usize,
        WindowOptions::default(),
    )?;

    let mut now = Instant::now();

    while window.is_open() {
        window.set_title(&format!(
            "video-presenter \u{2013} {} \u{2013} {:.2} fps",
            app.video_name(),
            now.elapsed().as_secs_f64().recip()
        ));
        now = Instant::now();

        let frame = match app.decoder().next_frame()? {
            Some(frame) => frame,
            None => break,
        };

        for i in frame.chunks(3) {
            buffer.push(from_u8_rgb(i[0], i[1], i[2]));
        }

        window.update_with_buffer(&buffer, width as usize, height as usize)?;
        buffer.clear();

        // thread::sleep(Duration::from_secs_f32(
        //     spf as f32 - now.elapsed().as_secs_f32(),
        // ));
    }

    Ok(())
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
