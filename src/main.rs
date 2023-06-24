#![feature(decl_macro)]

mod app;
mod args;
mod cues;
mod misc;
mod window;
use std::sync::Arc;

use app::App;

fn main() {
    let app = Arc::new(App::new());
    window::init(app);
}
