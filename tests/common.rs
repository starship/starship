use clap::{App, Arg};
use starship::modules;
use std::path::Path;

pub fn render_segment(module: &str, path: &Path) -> String {
    // Create an `Arg` with status_code of "0"
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);

    let segment = modules::handle(module, path, &args);

    segment.unwrap().output()
}
