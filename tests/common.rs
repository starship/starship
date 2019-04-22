use clap::{App, Arg};
use starship::context::Context;
use starship::modules;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn render_segment<T>(module: &str, path: T) -> String
where
    T: Into<PathBuf>,
{
    render_segment_with_status(module, path.into(), "0")
}

pub fn render_segment_with_status<T>(module: &str, path: T, status: &str) -> String
where
    T: Into<PathBuf>,
{
    // Create an `Arg` with status_code of "0"
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", status]);
    let context = Context::new_with_dir(args, path.into());

    let segment = modules::handle(module, &context);

    segment.unwrap().output()
}
