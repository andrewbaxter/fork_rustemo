use std::{path::PathBuf, process::exit};

use rustemo::settings::Settings;

fn main() {
    let root_dir: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "src"].iter().collect();
    if let Err(e) = rustemo::generate_parsers(
        root_dir,
        &Settings::default().with_force_all(true),
    ) {
        eprintln!("{}", e);
        exit(1);
    }
}