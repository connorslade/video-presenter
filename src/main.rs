#![feature(decl_macro)]

use std::{sync::Arc, thread};

use anyhow::Result;
use winit::{
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod app;
mod args;
mod cues;
mod misc;
use app::App;

fn main() -> Result<()> {
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("video-presenter")
        .build(&event_loop)
        .unwrap();
    let wid = u64::from(window.id());

    let app = Arc::new(App::new(wid)?);
    window.set_title(&format!("video-presenter \u{2013} {}", app.video_name()));

    let app2 = app.clone();
    thread::spawn(move || app2.event_loop());
    event_loop.run(move |event, _window, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }

            if input.key_pressed(VirtualKeyCode::P) {
                let paused = app.mpv.get_property::<bool>("pause").unwrap();
                app.mpv.set_property("pause", !paused).unwrap();
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                let paused = app.mpv.get_property::<bool>("pause").unwrap();

                if paused {
                    app.mpv.unpause().unwrap();
                } else {
                    app.seek_f().unwrap();
                }
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                app.seek_f().unwrap();
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                app.seek_r().unwrap();
            }

            if input.key_pressed(VirtualKeyCode::Period) {
                app.mpv.seek_frame().unwrap();
            }

            if input.key_pressed(VirtualKeyCode::Comma) {
                app.mpv.seek_frame_backward().unwrap();
            }
        }
    });
}
