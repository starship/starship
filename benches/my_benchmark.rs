#[macro_use]
extern crate criterion;

use criterion::Criterion;

use clap::{App, Arg};
use starship::context::Context;
use starship::modules;
use std::fs;
use tempfile::TempDir;

fn char_segment(c: &mut Criterion) {
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);
    let context = Context::new_with_dir(args, "~");

    c.bench_function("char segment", move |b| {
        b.iter(|| modules::handle("char", &context))
    });
}

fn dir_segment(c: &mut Criterion) {
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);
    let context = Context::new_with_dir(args, "~");

    c.bench_function("dir segment", move |b| {
        b.iter(|| modules::handle("dir", &context))
    });
}

fn line_break_segment(c: &mut Criterion) {
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);
    let context = Context::new_with_dir(args, "~");

    c.bench_function("line break segment", move |b| {
        b.iter(|| modules::handle("line_break", &context))
    });
}

fn git_branch_segment(c: &mut Criterion) {
    let tmp_dir = TempDir::new().unwrap();
    let repo_dir = tmp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir).unwrap();

    git2::Repository::init(&repo_dir).unwrap();

    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);
    let context = Context::new_with_dir(args, "~");

    c.bench_function("git_branch segment", move |b| {
        b.iter(|| modules::handle("git_branch", &context))
    });
}

criterion_group!(
    benches,
    char_segment,
    dir_segment,
    line_break_segment,
    git_branch_segment
);
criterion_main!(benches);
