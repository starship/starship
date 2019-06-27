use crate::utils;
use std::env;

use dirs::home_dir;

pub struct Config {
    data: toml::value::Table,
}

impl Config {
    /// Initialize the Config struct
    pub fn initialize() -> Config {
        if let Some(file_data) = Config::config_from_file() {
            return Config { data: file_data };
        }

        Config {
            data: toml::value::Table::new(),
        }
    }

    /// Create a config from a starship configuration file
    fn config_from_file() -> Option<toml::value::Table> {
        let file_path = match env::var("STARSHIP_CONFIG") {
            Ok(path) => {
                // Use $STARSHIP_CONFIG as the config path if available
                log::debug!("STARSHIP_CONFIG is set: {}", &path);
                path
            }
            Err(_) => {
                // Default to using ~/.config/starhip.toml
                log::debug!("STARSHIP_CONFIG is not set");
                let config_path = home_dir()?.join(".config/starship.toml");
                let config_path_str = config_path.to_str()?.to_owned();

                log::debug!("Using default config path: {}", config_path_str);
                config_path_str
            }
        };

        let toml_content = match utils::read_file(&file_path) {
            Ok(content) => {
                log::trace!("Config file content: \n{}", &content);
                Some(content)
            }
            Err(e) => {
                log::debug!("Unable to read config file content: \n{}", &e);
                None
            }
        }?;

        let config = toml::from_str(&toml_content).ok()?;
        log::debug!("Config found: \n{:?}", &config);
        Some(config)
    }

    /// Get the subset of the table for a module by its name
    pub fn get_module_config(&self, module_name: &str) -> Option<&toml::value::Table> {
        let module_config = self
            .data
            .get(module_name)
            .map(toml::Value::as_table)
            .unwrap_or(None);
        log::debug!("Config found for {}: {:?}", &module_name, &module_config);
        module_config
    }
}

/// Extends `toml::value::Table` with useful methods
pub trait TableExt {
    fn get_as_bool(&self, key: &str) -> Option<bool>;
}

impl TableExt for toml::value::Table {
    /// Get a key from a module's configuration as a boolean
    fn get_as_bool(&self, key: &str) -> Option<bool> {
        self.get(key).map(toml::Value::as_bool).unwrap_or(None)
    }
}

mod tests {
    use super::*;

    #[test]
    fn table_get_as_bool() {
        let mut table = toml::value::Table::new();

        // Use with boolean value
        table.insert("boolean".to_string(), toml::value::Value::Boolean(true));
        assert_eq!(table.get_as_bool("boolean"), Some(true));

        // Use with string value
        table.insert(
            "string".to_string(),
            toml::value::Value::String("true".to_string()),
        );
        assert_eq!(table.get_as_bool("string"), None);
    }
}
