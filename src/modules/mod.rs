// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod character;
mod cmd_duration;
mod directory;
mod git_branch;
mod git_state;
mod git_status;
mod golang;
mod hostname;
mod jobs;
mod line_break;
mod nix_shell;
mod nodejs;
mod package;
mod python;
mod ruby;
mod rust;
mod time;
mod username;

#[cfg(feature = "battery")]
mod battery;

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
        "line_break" => line_break::module(context),
        "package" => package::module(context),
        "git_branch" => git_branch::module(context),
        "git_state" => git_state::module(context),
        "git_status" => git_status::module(context),
        "username" => username::module(context),
        #[cfg(feature = "battery")]
        "battery" => battery::module(context),
        "cmd_duration" => cmd_duration::module(context),
        "jobs" => jobs::module(context),
        "nix_shell" => nix_shell::module(context),
        "hostname" => hostname::module(context),
        "time" => time::module(context),

        _ => {
            eprintln!("Error: Unknown module {}. Use starship module --list to list out all supported modules.", module);
            None
        }
    }
}
