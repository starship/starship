mod character;
mod directory;
mod git_branch;
mod line_break;
mod nodejs;
mod package;
mod python;
mod rust;

use crate::context::Context;
use crate::module::Module;

pub fn handle(module: &str, context: &Context) -> Option<Module> {
    match module {
        "dir" | "directory" => directory::segment(context),
        "char" | "character" => character::segment(context),
        "node" | "nodejs" => nodejs::segment(context),
        "rust" | "rustlang" => rust::segment(context),
        "python" => python::segment(context),
        "line_break" => line_break::segment(context),
        "package" => package::segment(context),
        "git_branch" => git_branch::segment(context),

        _ => panic!("Unknown module: {}", module),
    }
}
