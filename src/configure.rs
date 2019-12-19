use std::env;
use std::ffi::OsString;
use std::process::Command;

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
    match env::var("VISUAL") {
        Ok(val) => val,
        Err(_) => match env::var("EDITOR") {
            Ok(val) => val,
            Err(_) => STD_EDITOR.to_string(),
        },
    }
}

fn get_config_path() -> OsString {
    dirs::home_dir()
        .expect("Couldn't find home directory")
        .join(".config/starship.toml")
        .as_os_str()
        .to_owned()
}
