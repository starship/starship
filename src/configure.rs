use std::env;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process;
use std::process::Command;

use crate::config::RootModuleConfig;
use crate::config::StarshipConfig;
use std::fs::File;
use std::io::Write;
use toml::map::Map;
use toml::value::Table;
use toml::Value;

#[cfg(not(windows))]
const STD_EDITOR: &str = "vi";
#[cfg(windows)]
const STD_EDITOR: &str = "notepad.exe";

pub fn update_configuration(name: &str, value: &str) {
    let keys: Vec<&str> = name.split('.').collect();
    if keys.len() != 2 {
        log::error!("Please pass in a config key with a '.'");
        process::exit(1);
    }

    if let Some(table) = get_configuration().as_table_mut() {
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

        write_configuration(table);
    }
}

pub fn print_configuration(use_default: bool) {
    let config = if use_default {
        // Get default config
        let default_config = crate::configs::FullConfig::default();
        // Convert back to Value because toml can't serialize FullConfig directly
        toml::value::Value::try_from(default_config).unwrap()
    } else {
        // Get config as toml::Value
        let user_config = get_configuration();
        // Convert into FullConfig and fill in default values
        let user_config = crate::configs::FullConfig::try_load(Some(&user_config));
        // Convert back to Value because toml can't serialize FullConfig directly
        toml::value::Value::try_from(user_config).unwrap()
    };

    let string_config = toml::to_string_pretty(&config).unwrap();

    println!("# Warning: This config does not include keys that have an unset value");
    println!("{}", string_config);
}

pub fn toggle_configuration(name: &str, key: &str) {
    if let Some(table) = get_configuration().as_table_mut() {
        match table.get(name) {
            Some(v) => {
                if let Some(values) = v.as_table() {
                    let mut updated_values = values.clone();

                    let current: bool = match updated_values.get(key) {
                        Some(v) => match v.as_bool() {
                            Some(b) => b,
                            _ => {
                                log::error!(
                                    "Given config key '{}' must be in 'boolean' format",
                                    key
                                );
                                process::exit(1);
                            }
                        },
                        _ => {
                            log::error!("Given config key '{}' must be exist in config file", key);
                            process::exit(1);
                        }
                    };

                    updated_values.insert(key.to_string(), Value::Boolean(!current));

                    table.insert(name.to_string(), Value::Table(updated_values));

                    write_configuration(table);
                }
            }
            _ => {
                log::error!("Given module '{}' not found in config file", name);
                process::exit(1);
            }
        };
    }
}

pub fn get_configuration() -> Value {
    let starship_config = StarshipConfig::initialize();

    starship_config
        .config
        .expect("Failed to load starship config")
}

pub fn write_configuration(table: &mut Table) {
    let config_path = get_config_path();

    let config_str =
        toml::to_string_pretty(&table).expect("Failed to serialize the config to string");

    File::create(&config_path)
        .and_then(|mut file| file.write_all(config_str.as_ref()))
        .expect("Error writing starship config");
}

pub fn edit_configuration() {
    let config_path = get_config_path();
    let editor_cmd = shell_words::split(&get_editor()).expect("Unmatched quotes found in $EDITOR.");
    let editor_path = which::which(&editor_cmd[0]).expect("Unable to locate editor in $PATH.");

    let command = Command::new(editor_path)
        .args(&editor_cmd[1..])
        .arg(config_path)
        .status();

    match command {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!(
                    "Error: editor {:?} was not found. Did you set your $EDITOR or $VISUAL \
                    environment variables correctly?",
                    editor_cmd
                );
                std::process::exit(1)
            }
            other_error => panic!("failed to open file: {:?}", other_error),
        },
    };
}

fn get_editor() -> String {
    get_editor_internal(env::var("VISUAL").ok(), env::var("EDITOR").ok())
}

fn get_editor_internal(visual: Option<String>, editor: Option<String>) -> String {
    let editor_name = visual.unwrap_or_default();
    if !editor_name.is_empty() {
        return editor_name;
    }
    let editor_name = editor.unwrap_or_default();
    if !editor_name.is_empty() {
        return editor_name;
    }
    STD_EDITOR.into()
}

fn get_config_path() -> OsString {
    if let Some(config_path) = env::var_os("STARSHIP_CONFIG") {
        return config_path;
    }
    dirs_next::home_dir()
        .expect("couldn't find home directory")
        .join(".config")
        .join("starship.toml")
        .into()
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
        assert_eq!(STD_EDITOR, actual);
    }
    #[test]
    fn visual_empty_editor_not_set() {
        let actual = get_editor_internal(Some("".into()), None);
        assert_eq!(STD_EDITOR, actual);
    }

    #[test]
    fn visual_not_set_editor_set() {
        let actual = get_editor_internal(None, Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_not_set_editor_empty() {
        let actual = get_editor_internal(None, Some("".into()));
        assert_eq!(STD_EDITOR, actual);
    }
    #[test]
    fn visual_not_set_editor_not_set() {
        let actual = get_editor_internal(None, None);
        assert_eq!(STD_EDITOR, actual);
    }
}
