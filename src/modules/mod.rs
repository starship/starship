mod battery;
mod character;
mod directory;
mod git_branch;
mod git_status;
mod golang;
mod line_break;
mod nodejs;
mod package;
mod python;
mod rust;
mod timer;
mod username;

use crate::context::Context;
use crate::module::Module;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match module {
        "dir" | "directory" => directory::module(context),
        "char" | "character" => character::module(context),
        "node" | "nodejs" => nodejs::module(context),
        "rust" | "rustlang" => rust::module(context),
        "python" => python::module(context),
        "go" | "golang" => golang::module(context),
        "line_break" => line_break::module(context),
        "package" => package::module(context),
        "git_branch" => git_branch::module(context),
        "git_status" => git_status::module(context),
        "username" => username::module(context),
        "battery" => battery::module(context),
        "timer" => timer::module(context),

        _ => panic!("Unknown module: {}", module),
    }
}
