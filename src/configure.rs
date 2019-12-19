use std::env;
use std::process::Command;

const UNKNOWN_CONFIG: &str = "<unknown config>";
const STD_EDITOR: &str = "vi";

pub fn edit_configuration() {
    let editor = get_editor();
    let config_path = get_config_path();

    Command::new(editor)
        .arg(config_path)
        .status()
        .expect("failed to open file");
}

fn get_editor() -> String {
    match env::var("EDITOR") {
        Ok(val) => val,
        Err(_) => STD_EDITOR.to_string(),
    }
}

fn get_config_path() -> String {
    let home_dir = dirs::home_dir();

    if home_dir.is_none() {
        return UNKNOWN_CONFIG.to_string();
    }

    let path = home_dir.unwrap().join(".config/starship.toml");

    match path.to_str() {
        Some(p) => String::from(p),
        None => UNKNOWN_CONFIG.to_string(),
    }
}
