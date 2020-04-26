use std::env;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process;
use std::process::Command;

use starship::config::StarshipConfig;
use std::fs::File;
use std::io::Write;
use toml::map::Map;
use toml::Value;

const STD_EDITOR: &str = "vi";

pub fn update_configuration(name: &str, value: &str) {
    let config_path = get_config_path();

    let keys: Vec<&str> = name.split('.').collect();
    if keys.len() != 2 {
        log::error!("Please pass in a config key with a '.'");
        process::exit(1);
    }

    let starship_config = StarshipConfig::initialize();
    let mut config = starship_config
        .config
        .expect("Failed to load starship config");

    if let Some(table) = config.as_table_mut() {
        if !table.contains_key(keys[0]) {
            table.insert(keys[0].to_string(), Value::Table(Map::new()));
        }

        if let Some(values) = table.get(keys[0]).unwrap().as_table() {
            let mut updated_values = values.clone();

            if value.parse::<bool>().is_ok() {
                updated_values.insert(
                    keys[1].to_string(),
                    Value::Boolean(value.parse::<bool>().unwrap()),
                );
            } else if value.parse::<i64>().is_ok() {
                updated_values.insert(
                    keys[1].to_string(),
                    Value::Integer(value.parse::<i64>().unwrap()),
                );
            } else {
                updated_values.insert(keys[1].to_string(), Value::String(value.to_string()));
            }

            table.insert(keys[0].to_string(), Value::Table(updated_values));
        }

        let config_str =
            toml::to_string_pretty(&table).expect("Failed to serialize the config to string");
        File::create(&config_path)
            .and_then(|mut file| file.write_all(config_str.as_ref()))
            .expect("Error writing starship config");
    }
}

pub fn edit_configuration() {
    let config_path = get_config_path();
    let editor_cmd = get_editor();

    let mut cmd_iter = editor_cmd
        .to_str()
        .expect("environment variable contains invalid unicode")
        .split_whitespace();

    let editor = cmd_iter.next().unwrap_or(STD_EDITOR);
    let args: Vec<_> = cmd_iter.collect();

    let command = Command::new(editor).args(args).arg(config_path).status();

    match command {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!(
                    "Error: editor {:?} was not found. Did you set your $EDITOR or $VISUAL \
                    environment variables correctly?",
                    editor
                );
                eprintln!("Full error: {:?}", error);
                std::process::exit(1)
            }
            other_error => panic!("failed to open file: {:?}", other_error),
        },
    };
}

fn get_editor() -> OsString {
    get_editor_internal(env::var_os("VISUAL"), env::var_os("EDITOR"))
}

fn get_editor_internal(visual: Option<OsString>, editor: Option<OsString>) -> OsString {
    let mut editor_name = visual.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    editor_name = editor.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    STD_EDITOR.into()
}

fn get_config_path() -> OsString {
    let config_path = env::var_os("STARSHIP_CONFIG").unwrap_or_else(|| "".into());
    if config_path.is_empty() {
        dirs::home_dir()
            .expect("couldn't find home directory")
            .join(".config/starship.toml")
            .as_os_str()
            .to_owned()
    } else {
        config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is every possible permutation, 3² = 9.

    #[test]
    fn visual_set_editor_set() {
        let actual = get_editor_internal(Some("foo".into()), Some("bar".into()));
        assert_eq!("foo", actual);
    }
    #[test]
    fn visual_set_editor_empty() {
        let actual = get_editor_internal(Some("foo".into()), None);
        assert_eq!("foo", actual);
    }
    #[test]
    fn visual_set_editor_not_set() {
        let actual = get_editor_internal(Some("foo".into()), None);
        assert_eq!("foo", actual);
    }

    #[test]
    fn visual_empty_editor_set() {
        let actual = get_editor_internal(Some("".into()), Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_empty_editor_empty() {
        let actual = get_editor_internal(Some("".into()), Some("".into()));
        assert_eq!("vi", actual);
    }
    #[test]
    fn visual_empty_editor_not_set() {
        let actual = get_editor_internal(Some("".into()), None);
        assert_eq!("vi", actual);
    }

    #[test]
    fn visual_not_set_editor_set() {
        let actual = get_editor_internal(None, Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_not_set_editor_empty() {
        let actual = get_editor_internal(None, Some("".into()));
        assert_eq!("vi", actual);
    }
    #[test]
    fn visual_not_set_editor_not_set() {
        let actual = get_editor_internal(None, None);
        assert_eq!("vi", actual);
    }
}
