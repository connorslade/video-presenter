use clap::Parser;

use crate::{args::Args, cues::Cues};

pub struct App {
    args: Args,
    cues: Cues,
}

impl App {
    pub fn new() -> Self {
        let args = Args::parse();
        let cues = Cues::from_file(&args.markers).unwrap();
        println!("[*] Loaded {} cues", cues.len());
        for (i, e) in cues.iter().enumerate() {
            println!(" {}─ {:#?}", if i + 1 == cues.len() { "└" } else { "├" }, e);
        }

        Self { args, cues }
    }
}
