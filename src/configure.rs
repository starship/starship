use std::env;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process;
use std::process::Stdio;
use std::str::FromStr;

use crate::config::RootModuleConfig;
use crate::config::StarshipConfig;
use crate::configs::PROMPT_ORDER;
use crate::utils;
use std::fs::File;
use std::io::Write;
use toml::Value;
use toml_edit::Document;

#[cfg(not(windows))]
const STD_EDITOR: &str = "vi";
#[cfg(windows)]
const STD_EDITOR: &str = "notepad.exe";

pub fn update_configuration(name: &str, value: &str) {
    let mut doc = get_configuration_edit();

    match handle_update_configuration(&mut doc, name, value) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        _ => write_configuration(&doc),
    }
}

fn handle_update_configuration(doc: &mut Document, name: &str, value: &str) -> Result<(), String> {
    let mut current_item = &mut doc.root;

    for key in name.split('.') {
        if !current_item.is_table_like() {
            return Err("This command can only index into TOML tables".to_owned());
        }

        if key.is_empty() {
            return Err("Empty table keys are not supported".to_owned());
        }

        let table = current_item.as_table_like_mut().unwrap();

        if !table.contains_key(key) {
            table.insert(key, toml_edit::table());
        }

        current_item = table.get_mut(key).unwrap();
    }

    let mut new_value = toml_edit::Value::from_str(value)
        .map(toml_edit::Item::Value)
        .unwrap_or_else(|_| toml_edit::value(value));

    if let Some(value) = current_item.as_value() {
        *new_value.as_value_mut().unwrap().decor_mut() = value.decor().clone();
    }

    *current_item = new_value;

    Ok(())
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

    println!("# Warning: This config does not include keys that have an unset value\n");
    println!(
        "# $all is shorthand for {}",
        PROMPT_ORDER
            .iter()
            .map(|module_name| format!("${}", module_name))
            .collect::<String>()
    );

    // Unwrapping is fine because config is based on FullConfig
    let custom_modules = config.get("custom").unwrap().as_table().unwrap();
    if !use_default && !custom_modules.is_empty() {
        println!(
            "# $custom (excluding any modules already listed in `format`) is shorthand for {}",
            custom_modules
                .keys()
                .map(|module_name| format!("${{custom.{}}}", module_name))
                .collect::<String>()
        );
    }
    println!("{}", string_config);
}

pub fn toggle_configuration(name: &str, key: &str) {
    let mut doc = get_configuration_edit();

    match handle_toggle_configuration(&mut doc, name, key) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        _ => write_configuration(&doc),
    }
}

fn handle_toggle_configuration(doc: &mut Document, name: &str, key: &str) -> Result<(), String> {
    if name.is_empty() || key.is_empty() {
        return Err("Empty table keys are not supported".to_owned());
    }

    let table = doc.as_table_mut();

    let values = table
        .get_mut(name)
        .ok_or_else(|| format!("Given module '{}' not found in config file", name))?
        .as_table_like_mut()
        .ok_or_else(|| format!("Given config entry '{}' is not a module", key))?;

    let old_value = values
        .get(key)
        .ok_or_else(|| format!("Given config key '{}' must exist in config file", key))?;

    let old = old_value
        .as_bool()
        .ok_or_else(|| format!("Given config key '{}' must be in 'boolean' format", key))?;

    let mut new_value = toml_edit::value(!old);
    // Above code already checks if it is a value (bool)
    *new_value.as_value_mut().unwrap().decor_mut() = old_value.as_value().unwrap().decor().clone();

    values.insert(key, new_value);
    Ok(())
}

pub fn get_configuration() -> Value {
    let starship_config = StarshipConfig::initialize();

    starship_config
        .config
        .expect("Failed to load starship config")
}

pub fn get_configuration_edit() -> Document {
    let file_path = get_config_path();
    let toml_content = match utils::read_file(&file_path) {
        Ok(content) => {
            log::trace!("Config file content: \"\n{}\"", &content);
            Some(content)
        }
        Err(e) => {
            let level = if e.kind() == ErrorKind::NotFound {
                log::Level::Debug
            } else {
                log::Level::Error
            };

            log::log!(level, "Unable to read config file content: {}", &e);
            None
        }
    };

    toml_content
        .unwrap_or_default()
        .parse::<Document>()
        .expect("Failed to load starship config")
}

pub fn write_configuration(doc: &Document) {
    let config_path = get_config_path();

    let config_str = doc.to_string();

    File::create(&config_path)
        .and_then(|mut file| file.write_all(config_str.as_ref()))
        .expect("Error writing starship config");
}

pub fn edit_configuration() {
    let config_path = get_config_path();
    let editor_cmd = shell_words::split(&get_editor()).expect("Unmatched quotes found in $EDITOR.");

    let command = utils::create_command(&editor_cmd[0])
        .expect("Unable to locate editor in $PATH.")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
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
    utils::home_dir()
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

    fn create_doc() -> Document {
        let config = concat!(
            " # comment\n",
            "  [status] # comment\n",
            "disabled =    false # comment\n",
            "# comment\n",
            "\n"
        );

        config.parse::<Document>().unwrap()
    }

    #[test]
    fn test_toggle_simple() {
        let mut doc = create_doc();

        assert!(!doc["status"]["disabled"].as_bool().unwrap());

        handle_toggle_configuration(&mut doc, "status", "disabled").unwrap();

        assert!(doc["status"]["disabled"].as_bool().unwrap());

        let new_config = concat!(
            " # comment\n",
            "  [status] # comment\n",
            "disabled =    true # comment\n",
            "# comment\n",
            "\n"
        );

        assert_eq!(doc.to_string(), new_config)
    }

    #[test]
    fn test_toggle_missing_module() {
        let mut doc = create_doc();
        assert!(handle_toggle_configuration(&mut doc, "missing_module", "disabled").is_err());
    }

    #[test]
    fn test_toggle_missing_key() {
        let mut doc = create_doc();
        assert!(handle_toggle_configuration(&mut doc, "status", "missing").is_err());
    }

    #[test]
    fn test_toggle_wrong_type() {
        let mut doc = create_doc();
        doc["status"]["disabled"] = toml_edit::value("a");

        assert!(handle_toggle_configuration(&mut doc, "status", "disabled").is_err());

        doc["format"] = toml_edit::value("$all");

        assert!(handle_toggle_configuration(&mut doc, "format", "disabled").is_err());
    }

    #[test]
    fn test_toggle_empty() {
        let mut doc = create_doc();

        doc["status"][""] = toml_edit::value(true);
        doc[""]["disabled"] = toml_edit::value(true);

        assert!(handle_toggle_configuration(&mut doc, "status", "").is_err());
        assert!(handle_toggle_configuration(&mut doc, "", "disabled").is_err());
    }

    #[test]
    fn test_update_config_wrong_type() {
        let mut doc = create_doc();

        assert!(
            handle_update_configuration(&mut doc, "status.disabled.not_a_table", "true").is_err()
        );
    }

    #[test]
    fn test_update_config_simple() {
        let mut doc = create_doc();

        assert!(!doc["status"]["disabled"].as_bool().unwrap());

        handle_update_configuration(&mut doc, "status.disabled", "true").unwrap();

        assert!(doc["status"]["disabled"].as_bool().unwrap());

        let new_config = concat!(
            " # comment\n",
            "  [status] # comment\n",
            "disabled =    true # comment\n",
            "# comment\n",
            "\n"
        );

        assert_eq!(doc.to_string(), new_config)
    }

    #[test]
    fn test_update_config_parse() {
        let mut doc = create_doc();

        handle_update_configuration(&mut doc, "test", "true").unwrap();

        assert!(doc["test"].as_bool().unwrap());

        handle_update_configuration(&mut doc, "test", "0").unwrap();

        assert_eq!(doc["test"].as_integer().unwrap(), 0);

        handle_update_configuration(&mut doc, "test", "0.0").unwrap();

        assert!(doc["test"].is_float());

        handle_update_configuration(&mut doc, "test", "a string").unwrap();

        assert_eq!(doc["test"].as_str().unwrap(), "a string");

        handle_update_configuration(&mut doc, "test", "\"true\"").unwrap();

        assert_eq!(doc["test"].as_str().unwrap(), "true");
    }

    #[test]
    fn test_update_config_empty() {
        let mut doc = create_doc();

        assert!(handle_update_configuration(&mut doc, "", "true").is_err());
        assert!(handle_update_configuration(&mut doc, ".....", "true").is_err());
        assert!(handle_update_configuration(&mut doc, "a.a.a..a.a", "true").is_err());
        assert!(handle_update_configuration(&mut doc, "a.a.a.a.a.", "true").is_err());
    }

    #[test]
    fn test_update_config_deep() {
        let mut doc = create_doc();

        handle_update_configuration(&mut doc, "a.b.c.d.e.f.g.h", "true").unwrap();

        assert!(doc["a"]["b"]["c"]["d"]["e"]["f"]["g"]["h"]
            .as_bool()
            .unwrap())
    }
}
