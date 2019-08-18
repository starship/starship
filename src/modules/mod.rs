mod battery;
mod character;
mod cmd_duration;
mod directory;
mod git_branch;
mod git_status;
mod golang;
mod jobs;
mod line_break;
mod nodejs;
mod package;
mod python;
mod ruby;
mod rust;
mod nim;
mod username;

use crate::context::Context;
use crate::module::Module;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match module {
        "directory" => directory::module(context),
        "character" => character::module(context),
        "nodejs" => nodejs::module(context),
        "rust" => rust::module(context),
        "python" => python::module(context),
        "ruby" => ruby::module(context),
        "golang" => golang::module(context),
        "nim" => nim::module(context),
        "line_break" => line_break::module(context),
        "package" => package::module(context),
        "git_branch" => git_branch::module(context),
        "git_status" => git_status::module(context),
        "username" => username::module(context),
        "battery" => battery::module(context),
        "cmd_duration" => cmd_duration::module(context),
        "jobs" => jobs::module(context),

        _ => panic!("Unknown module: {}", module),
    }
}
