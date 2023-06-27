#![feature(decl_macro)]

use std::sync::Arc;

use anyhow::Result;

mod app;
mod args;
mod cues;
mod misc;
use app::App;

fn main() -> Result<()> {
    let app = Arc::new(App::new()?);
    app.event_loop();
}
