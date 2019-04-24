mod character;
mod directory;
mod line_break;
mod nodejs;
mod rust;
mod python;

use crate::context::Context;
use crate::segment::Segment;

pub fn handle(module: &str, context: &Context) -> Option<Segment> {
    match module {
        "dir" | "directory" => directory::segment(context),
        "char" | "character" => character::segment(context),
        "node" | "nodejs" => nodejs::segment(context),
        "rust" | "rustlang" => rust::segment(context),
        "python" => python::segment(context),
        "line_break" => line_break::segment(context),

        _ => panic!("Unknown module: {}", module),
    }
}
