mod battery;
mod character;
mod directory;
mod git_branch;
mod git_status;
mod go;
mod line_break;
mod nodejs;
mod package;
mod python;
mod rust;
mod username;

use crate::context::Context;
use crate::module::Module;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match module {
        "dir" | "directory" => directory::segment(context),
        "char" | "character" => character::segment(context),
        "node" | "nodejs" => nodejs::segment(context),
        "rust" | "rustlang" => rust::segment(context),
        "python" => python::segment(context),
        "go" | "golang" => go::segment(context),
        "line_break" => line_break::segment(context),
        "package" => package::segment(context),
        "git_branch" => git_branch::segment(context),
        "git_status" => git_status::segment(context),
        "username" => username::segment(context),
        "battery" => battery::segment(context),

        _ => panic!("Unknown module: {}", module),
    }
}
