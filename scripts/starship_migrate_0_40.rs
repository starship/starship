/*
 * starship_migrate_0_40
 *
 * A script for migrating starship configuration file from before v0.40 to v0.40.
 *
 */
use dirs::home_dir;
use std::env;
use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;
use toml::Value;

use starship::config::{ModuleConfig, StarshipConfig};

fn main() {
    let file_path = get_file_path().expect("Fail to determine the path of the config file.");

    // Backup the original config
    let file_name = file_path
        .file_name()
        .expect("The config file does not have a file name");
    let mut backup_file_path = file_path.clone();
    backup_file_path.set_file_name(format!(
        "{}.bkup",
        file_name.to_str().expect("Invalid starship config path")
    ));
    copy(&file_path, &backup_file_path).expect("Failed to backup original config file");

    // Load starship config
    let mut starship_config = StarshipConfig::initialize();

    if let Some(mut config) = starship_config.config.as_mut() {
        update_starship_config(&mut config);
    }

    // Write to the config file
    let config = starship_config
        .config
        .expect("Failed to load starship config");
    let config_str =
        toml::to_string_pretty(&config).expect("Failed to serialize the config to string");
    File::create(&file_path)
        .and_then(|mut file| file.write_all(config_str.as_ref()))
        .expect("Error writing starship config");

    eprintln!(
        "Updated starship config written to `{}`.\nYou can find your original config file in `{}`.",
        &file_path.to_str().unwrap(),
        &backup_file_path.to_str().unwrap(),
    );
}

fn get_file_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("STARSHIP_CONFIG") {
        // Use $STARSHIP_CONFIG as the config path if available
        Some(PathBuf::from(path))
    } else {
        // Default to using ~/.config/starship.toml
        home_dir().map(|path| path.join(".config/starship.toml"))
    }
}

fn update_starship_config(config: &mut Value) {
    migrate_starship_root_config(config);
}

fn migrate_starship_root_config(root_config: &mut Value) {
    if let Some(table) = root_config.as_table_mut() {
        // Don't do anything in case there is a `format` key
        if table.contains_key("format") {
            return;
        }

        // Always add format to root even if there is no `add_newline` and `prompt_order`.
        let add_newline = table
            .get("add_newline")
            .and_then(bool::from_config)
            .unwrap_or(true);
        let prompt_order: Vec<&str> = table
            .get("prompt_order")
            .and_then(Vec::from_config)
            .unwrap_or_else(|| vec!["all"]);

        let newline_format = if add_newline { "\n" } else { "" };
        let prompt_format_list: Vec<String> = prompt_order
            .into_iter()
            .map(|module| format!("${}", module))
            .collect();
        let prompt_format = prompt_format_list.join("");
        let format = format!("{}{}", &newline_format, &prompt_format);

        table.remove("add_newline");
        table.remove("prompt_order");
        table.insert("format".to_owned(), Value::String(format));
    }
}

#[allow(dead_code)]
struct SegmentConfigValue {
    value: Option<String>,
    style: Option<String>,
}

impl From<&Value> for SegmentConfigValue {
    fn from(value: &Value) -> Self {
        let (value, style) = match value {
            Value::String(config_str) => (Some(config_str.clone()), None),
            Value::Table(ref config_table) => (
                config_table
                    .get("value")
                    .and_then(|value| value.as_str())
                    .map(|value| value.to_owned()),
                config_table
                    .get("style")
                    .and_then(|style| style.as_str())
                    .map(|value| value.to_owned()),
            ),
            _ => (None, None),
        };
        Self { value, style }
    }
}

impl SegmentConfigValue {
    /// Convert value of `SegmentConfig` to format string.
    #[allow(dead_code)]
    fn to_format(&self, name: &str) -> String {
        let Self { value, style } = self;
        match style {
            Some(style) => match value {
                Some(value) => format!("[{}]({})", value, style),
                None => format!("[${}]({})", name, style),
            },
            None => value.clone().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_segment_config_map() {
        let config = toml::toml! {
            symbol = { value = "R ", style = "red bold" }
        };
        let symbol = config.get("symbol").unwrap();
        assert_eq!(
            SegmentConfigValue::from(symbol).to_format("symbol"),
            "[R ](red bold)"
        );
    }

    #[test]
    fn test_parse_segment_config_style_only() {
        let config = toml::toml! {
            version.style = "red bold"
        };
        let version = config.get("version").unwrap();
        assert_eq!(
            SegmentConfigValue::from(version).to_format("version"),
            "[$version](red bold)"
        );
    }

    #[test]
    fn test_parse_segment_config_value_only() {
        let config = toml::toml! {
            symbol = "R "
        };
        let symbol = config.get("symbol").unwrap();
        assert_eq!(SegmentConfigValue::from(symbol).to_format("symbol"), "R ");

        let config = toml::toml! {
            symbol.value = "R "
        };
        let symbol = config.get("symbol").unwrap();
        assert_eq!(SegmentConfigValue::from(symbol).to_format("symbol"), "R ");
    }
}
