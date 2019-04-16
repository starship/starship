#[macro_use]
extern crate criterion;

use criterion::Criterion;

use clap::{App, Arg};
use starship::modules;
use std::path::Path;

fn char_segment(c: &mut Criterion) {
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);

    let path = Path::new("~");

    c.bench_function("char segment", move |b| b.iter(|| modules::handle("char", &path, &args)));
}

fn dir_segment(c: &mut Criterion) {
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);

    let path = Path::new("~");

    c.bench_function("dir segment", move |b| b.iter(|| modules::handle("dir", &path, &args)));
}

criterion_group!(benches, dir_segment, char_segment);
criterion_main!(benches);
