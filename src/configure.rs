use std::fmt::Write as _;
use std::process;
use std::process::Stdio;
use std::str::FromStr;

use crate::config::ModuleConfig;
use crate::config::StarshipConfig;
use crate::configs::PROMPT_ORDER;
use crate::context::Context;
use crate::utils;
use std::fs::File;
use std::io::Write;
use toml_edit::DocumentMut;

#[cfg(not(windows))]
const STD_EDITOR: &str = "vi";
#[cfg(windows)]
const STD_EDITOR: &str = "notepad.exe";

pub fn update_configuration(context: &Context, name: &str, value: &str) {
    let mut doc = get_configuration_edit(context);

    match handle_update_configuration(&mut doc, name, value) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        _ => write_configuration(context, &doc),
    }
}

fn handle_update_configuration(
    doc: &mut DocumentMut,
    name: &str,
    value: &str,
) -> Result<(), String> {
    let mut keys = name.split('.');

    let first_key = keys.next().unwrap_or_default();
    if first_key.is_empty() {
        return Err("Empty table keys are not supported".to_owned());
    }

    let table = doc.as_table_mut();
    let mut current_item = table.entry(first_key).or_insert_with(toml_edit::table);

    for key in keys {
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
        .map_or_else(|_| toml_edit::value(value), toml_edit::Item::Value);

    if let Some(value) = current_item.as_value() {
        *new_value.as_value_mut().unwrap().decor_mut() = value.decor().clone();
    }

    *current_item = new_value;

    Ok(())
}

pub fn print_configuration(context: &Context, use_default: bool, paths: &[String]) -> String {
    let config = if use_default {
        // Get default config
        let default_config = crate::configs::FullConfig::default();
        // Convert back to Value because toml can't serialize FullConfig directly
        toml::value::Value::try_from(default_config).unwrap()
    } else {
        // Get config as toml::Value
        let user_config = get_configuration(context);
        // Convert into FullConfig and fill in default values
        let user_config = crate::configs::FullConfig::load(&user_config);
        // Convert back to Value because toml can't serialize FullConfig directly
        toml::value::Value::try_from(user_config).unwrap()
    };

    println!("# Warning: This config does not include keys that have an unset value\n");

    // These are only used for format specifiers so don't print them if we aren't showing formats.
    if paths.is_empty()
        || paths
            .iter()
            .any(|path| path == "format" || path == "right_format")
    {
        println!(
            "# $all is shorthand for {}",
            PROMPT_ORDER
                .iter()
                .fold(String::new(), |mut output, module_name| {
                    let _ = write!(output, "${module_name}");
                    output
                })
        );

        // Unwrapping is fine because config is based on FullConfig
        let custom_modules = config.get("custom").unwrap().as_table().unwrap();
        if !use_default && !custom_modules.is_empty() {
            println!(
                "# $custom (excluding any modules already listed in `format`) is shorthand for {}",
                custom_modules.keys().fold(String::new(), |mut output, b| {
                    let _ = write!(output, "${{custom.{b}}}");
                    output
                })
            );
        }
    }

    let print_config = if paths.is_empty() {
        config
    } else {
        extract_toml_paths(config, paths)
    };

    let string_config = toml::to_string_pretty(&print_config).unwrap();

    println!("{string_config}");
    string_config
}

fn extract_toml_paths(mut config: toml::Value, paths: &[String]) -> toml::Value {
    // Extract all the requested sections into a new configuration.
    let mut subset = toml::value::Table::new();
    let Some(config) = config.as_table_mut() else {
        // This function doesn't make any sense if the root is not a table.
        return toml::Value::Table(subset);
    };

    'paths: for path in paths {
        let path_segments: Vec<_> = path.split('.').collect();
        let (&end, parents) = path_segments.split_last().unwrap_or((&"", &[]));

        // Locate the parent table to remove the value from.
        let mut source_cursor = &mut *config;
        for &segment in parents {
            source_cursor = if let Some(child) = source_cursor
                .get_mut(segment)
                .and_then(toml::Value::as_table_mut)
            {
                child
            } else {
                // We didn't find a value for this path, so move on to the next path.
                continue 'paths;
            }
        }

        // Extract the value to move.
        let Some(value) = source_cursor.remove(end) else {
            // We didn't find a value for this path, so move on to the next path.
            continue 'paths;
        };

        // Create a destination for that value.
        let mut destination_cursor = &mut subset;
        for &segment in &path_segments[..path_segments.len() - 1] {
            // Because we initialize `subset` to be a table, and only add additional values that
            // exist in `config`, it's impossible for the value here to not be a table.
            destination_cursor = destination_cursor
                .entry(segment)
                .or_insert_with(|| toml::Value::Table(toml::value::Table::new()))
                .as_table_mut()
                .unwrap();
        }

        destination_cursor.insert(end.to_owned(), value);
    }

    toml::Value::Table(subset)
}

pub fn toggle_configuration(context: &Context, name: &str, key: &str) {
    let mut doc = get_configuration_edit(context);

    match handle_toggle_configuration(&mut doc, name, key) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        _ => write_configuration(context, &doc),
    }
}

fn handle_toggle_configuration(doc: &mut DocumentMut, name: &str, key: &str) -> Result<(), String> {
    if name.is_empty() || key.is_empty() {
        return Err("Empty table keys are not supported".to_owned());
    }

    let table = doc.as_table_mut();

    let values = table
        .get_mut(name)
        .ok_or_else(|| format!("Given module '{name}' not found in config file"))?
        .as_table_like_mut()
        .ok_or_else(|| format!("Given config entry '{key}' is not a module"))?;

    let old_value = values
        .get(key)
        .ok_or_else(|| format!("Given config key '{key}' must exist in config file"))?;

    let old = old_value
        .as_bool()
        .ok_or_else(|| format!("Given config key '{key}' must be in 'boolean' format"))?;

    let mut new_value = toml_edit::value(!old);
    // Above code already checks if it is a value (bool)
    *new_value.as_value_mut().unwrap().decor_mut() = old_value.as_value().unwrap().decor().clone();

    values.insert(key, new_value);
    Ok(())
}

pub fn get_configuration(context: &Context) -> toml::Table {
    let starship_config = StarshipConfig::initialize(&context.get_config_path_os());

    starship_config.config.unwrap_or_default()
}

pub fn get_configuration_edit(context: &Context) -> DocumentMut {
    let config_file_path = context.get_config_path_os();
    let toml_content = StarshipConfig::read_config_content_as_str(&config_file_path);

    toml_content
        .unwrap_or_default()
        .parse::<DocumentMut>()
        .expect("Failed to load starship config")
}

pub fn write_configuration(context: &Context, doc: &DocumentMut) {
    let config_path = context.get_config_path_os().unwrap_or_else(|| {
        eprintln!("config path required to write configuration");
        process::exit(1);
    });

    let config_str = doc.to_string();

    File::create(config_path)
        .and_then(|mut file| file.write_all(config_str.as_ref()))
        .expect("Error writing starship config");
}

pub fn edit_configuration(
    context: &Context,
    editor_override: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Argument currently only used for testing, but could be used to specify
    // an editor override on the command line.
    let config_path = context.get_config_path_os().unwrap_or_else(|| {
        eprintln!("config path required to edit configuration");
        process::exit(1);
    });

    let editor_cmd = shell_words::split(&get_editor(editor_override))?;
    let mut command = match utils::create_command(&editor_cmd[0]) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!(
                "Unable to find editor {:?}. Are $VISUAL and $EDITOR set correctly?",
                editor_cmd[0]
            );
            return Err(Box::new(e));
        }
    };

    let res = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(&editor_cmd[1..])
        .arg(config_path)
        .status();

    if let Err(e) = res {
        eprintln!("Unable to launch editor {editor_cmd:?}");
        return Err(Box::new(e));
    }

    Ok(())
}

fn get_editor(editor_override: Option<&str>) -> String {
    if let Some(cmd) = editor_override {
        cmd.to_string()
    } else {
        get_editor_internal(std::env::var("VISUAL").ok(), std::env::var("EDITOR").ok())
    }
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

#[cfg(test)]
mod tests {
    use std::{fs::create_dir, io};

    use tempfile::TempDir;
    use toml_edit::Item;

    use crate::{
        context::{Shell, Target},
        context_env::Env,
    };

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
        let actual = get_editor_internal(Some(String::new()), Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_empty_editor_empty() {
        let actual = get_editor_internal(Some(String::new()), Some(String::new()));
        assert_eq!(STD_EDITOR, actual);
    }
    #[test]
    fn visual_empty_editor_not_set() {
        let actual = get_editor_internal(Some(String::new()), None);
        assert_eq!(STD_EDITOR, actual);
    }

    #[test]
    fn visual_not_set_editor_set() {
        let actual = get_editor_internal(None, Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_not_set_editor_empty() {
        let actual = get_editor_internal(None, Some(String::new()));
        assert_eq!(STD_EDITOR, actual);
    }
    #[test]
    fn visual_not_set_editor_not_set() {
        let actual = get_editor_internal(None, None);
        assert_eq!(STD_EDITOR, actual);
    }

    #[test]
    fn no_panic_when_editor_unparsable() {
        let outcome = edit_configuration(&Default::default(), Some("\"vim"));
        assert!(outcome.is_err());
    }

    #[test]
    fn no_panic_when_editor_not_found() {
        let outcome = edit_configuration(&Default::default(), Some("this_editor_does_not_exist"));
        assert!(outcome.is_err());
    }

    #[test]
    fn test_extract_toml_paths() {
        let config = toml::toml! {
            extract_root = true
            ignore_root = false

            [extract_section]
            ok = true

            [extract_section.subsection]
            ok = true

            [ignore_section]
            ok = false

            [extract_subsection]
            ok = false

            [extract_subsection.extracted]
            ok = true

            [extract_subsection.ignored]
            ok = false
        };
        let expected_config = toml::toml! {
            extract_root = true

            [extract_section]
            ok = true

            [extract_section.subsection]
            ok = true

            [extract_subsection.extracted]
            ok = true
        };
        let actual_config = extract_toml_paths(
            toml::Value::Table(config),
            &[
                "extract_root".to_owned(),
                "extract_section".to_owned(),
                "extract_subsection.extracted".to_owned(),
            ],
        );

        assert_eq!(toml::Value::Table(expected_config), actual_config);
    }

    fn create_doc() -> DocumentMut {
        let config = concat!(
            " # comment\n",
            "  [status] # comment\n",
            "disabled =    false # comment\n",
            "# comment\n",
            "\n"
        );

        config.parse::<DocumentMut>().unwrap()
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
        assert!(handle_update_configuration(&mut doc, ".a.a.a.a.a", "true").is_err());
    }

    #[test]
    fn test_update_config_deep() {
        let mut doc = create_doc();

        handle_update_configuration(&mut doc, "a.b.c.d.e.f.g.h", "true").unwrap();

        assert!(doc["a"]["b"]["c"]["d"]["e"]["f"]["g"]["h"]
            .as_bool()
            .unwrap())
    }

    #[test]
    fn write_and_get_configuration_test() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let context = setup_config(&dir, true, StarshipConfigEnvScenario::NotSpecified)?;
        let mut doc = get_configuration_edit(&context);
        doc["directory"]["format"] = Item::Value("myformat".into());
        write_configuration(&context, &doc);
        let doc_reloaded = get_configuration_edit(&context);
        assert_eq!(
            "myformat",
            doc_reloaded["directory"]["format"].as_str().unwrap()
        );
        dir.close()
    }

    const PRINT_CONFIG_DEFAULT: &str = "[custom]";
    const PRINT_CONFIG_HOME: &str = "[custom.home]";
    const PRINT_CONFIG_ENV: &str = "[custom.STARSHIP_CONFIG]";

    #[test]
    fn print_configuration_scenarios() -> io::Result<()> {
        run_print_configuration_test(
            "~/.config/starship.toml, no STARSHIP_CONFIG uses home",
            true,
            StarshipConfigEnvScenario::NotSpecified,
            PRINT_CONFIG_HOME,
        )?;
        run_print_configuration_test(
            "no ~/.config/starship.toml, no STARSHIP_CONFIG uses default",
            false,
            StarshipConfigEnvScenario::NotSpecified,
            PRINT_CONFIG_DEFAULT,
        )?;
        run_print_configuration_test(
            "~/.config/starship.toml, STARSHIP_CONFIG nonexisting file uses default",
            true,
            StarshipConfigEnvScenario::NonExistingFile,
            PRINT_CONFIG_DEFAULT,
        )?;
        run_print_configuration_test(
            "~/.config/starship.toml, STARSHIP_CONFIG existing file uses STARSHIP_CONFIG file",
            true,
            StarshipConfigEnvScenario::ExistingFile,
            PRINT_CONFIG_ENV,
        )?;
        Ok(())
    }

    enum StarshipConfigEnvScenario {
        NotSpecified,
        NonExistingFile,
        ExistingFile,
    }

    fn run_print_configuration_test(
        message: &str,
        home_file_exists: bool,
        starship_config_env_scenario: StarshipConfigEnvScenario,
        expected_first_line: &str,
    ) -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let context = setup_config(&dir, home_file_exists, starship_config_env_scenario)?;
        let config = print_configuration(&context, false, &["custom".to_string()]);
        let first_line = config.split('\n').next().unwrap();
        assert_eq!(expected_first_line, first_line, "{message}");
        dir.close()
    }

    fn setup_config(
        dir: &TempDir,
        home_file_exists: bool,
        starship_config_env_scenario: StarshipConfigEnvScenario,
    ) -> io::Result<Context> {
        let config_path = dir.path().to_path_buf().join(".config");
        create_dir(&config_path)?;
        let home_starship_toml = config_path.join("starship.toml");
        let env_toml = dir.path().join("env.toml");
        if home_file_exists {
            let mut home_file = File::create(home_starship_toml)?;
            home_file.write_all(PRINT_CONFIG_HOME.as_bytes())?;
        }

        let env_starship_config = match starship_config_env_scenario {
            StarshipConfigEnvScenario::NotSpecified => None,
            StarshipConfigEnvScenario::NonExistingFile => Some(env_toml),
            StarshipConfigEnvScenario::ExistingFile => {
                let mut env_toml_file = File::create(&env_toml)?;
                env_toml_file.write_all(PRINT_CONFIG_ENV.as_bytes())?;
                Some(env_toml)
            }
        };

        let mut env = Env::default();
        if let Some(v) = env_starship_config {
            env.insert("STARSHIP_CONFIG", v.to_string_lossy().to_string());
        }
        env.insert(
            "HOME",
            dir.path().to_path_buf().to_string_lossy().to_string(),
        );

        Ok(Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            Default::default(),
            Default::default(),
            env,
        ))
    }
}
