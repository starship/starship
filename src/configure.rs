use crate::context::Context;
use clap::ArgMatches;

use std::env;
use std::process::Command;

const UNKNOWN_CONFIG: &str = "<unknown config>";
const STD_EDITOR: &str = "vi";

pub fn edit_configuration(args: ArgMatches) {
    let context = Context::new(args);
    let editor = get_editor(context);
    let config_path = get_config_path();

    println!("using {} as editor", editor);
    println!("path: {}", config_path);

    Command::new(editor)
        .arg(config_path)
        .status()
        .expect("failed to open file");
}

fn get_editor(context: Context) -> String {
    let config = context.config.get_root_config();
    
    let mut editor = match env::var("EDITOR") {
        Ok(e) => e,
        Err(_) => STD_EDITOR.to_string(),
    };

    if config.editor != "" {
        editor = config.editor.to_string();
    }
    
    editor
}

fn get_config_path() -> String {
    let home_dir = dirs::home_dir();

    if home_dir.is_none() {
        return UNKNOWN_CONFIG.to_string();
    }

    let path = home_dir.unwrap().join(".config/starship.toml");

    return match path.to_str() {
        Some(p) => String::from(p),
        None => UNKNOWN_CONFIG.to_string(),
    }
}