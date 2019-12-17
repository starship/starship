use crate::context::Context;
use clap::ArgMatches;

use std::process::Command;

pub fn edit_configuration(args: ArgMatches) {
    let context = Context::new(args);
    let editor = get_editor(context);

    println!("using {} as editor", editor);

    Command::new(editor)
        .arg("~/.config/starship.toml")
        .status()
        .expect("failed to open file");
}

fn get_editor(context: Context) -> String {
    let config = context.config.get_root_config();
    String::from(config.editor)
}