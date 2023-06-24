use std::sync::Arc;

use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::app::App;

pub fn init(app: Arc<App>) {
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("video-presenter - {}", app.video_name()))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
        }

        if let Event::RedrawRequested(_) = event {
            // do render stuff idk
        }
    });
}
