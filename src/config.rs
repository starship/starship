use crate::utils;
use std::env;

use dirs::home_dir;
use toml::value::Table;

pub trait Config {
    fn initialize() -> Table;
    fn config_from_file() -> Option<Table>;
    fn get_module_config(&self, module_name: &str) -> Option<&Table>;

    // Config accessor methods
    fn get_as_bool(&self, key: &str) -> Option<bool>;
    fn get_as_str(&self, key: &str) -> Option<&str>;
    fn get_as_i64(&self, key: &str) -> Option<i64>;
    fn get_as_array(&self, key: &str) -> Option<&Vec<toml::value::Value>>;

    // Internal implementation for accessors
    fn get_config(&self, key: &str) -> Option<&toml::value::Value>;
}

impl Config for Table {
    /// Initialize the Config struct
    fn initialize() -> Table {
        if let Some(file_data) = Self::config_from_file() {
            return file_data;
        }

        Self::new()
    }

    /// Create a config from a starship configuration file
    fn config_from_file() -> Option<Table> {
        let file_path = if let Ok(path) = env::var("STARSHIP_CONFIG") {
            // Use $STARSHIP_CONFIG as the config path if available
            log::debug!("STARSHIP_CONFIG is set: \n{}", &path);
            path
        } else {
            // Default to using ~/.config/starship.toml
            log::debug!("STARSHIP_CONFIG is not set");
            let config_path = home_dir()?.join(".config/starship.toml");
            let config_path_str = config_path.to_str()?.to_owned();

            log::debug!("Using default config path: {}", config_path_str);
            config_path_str
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
        log::debug!("Config parsed: \n{:?}", &config);
        Some(config)
    }

    /// Get the subset of the table for a module by its name
    fn get_module_config(&self, module_name: &str) -> Option<&toml::value::Table> {
        let module_config = self.get(module_name).and_then(toml::Value::as_table);

        if module_config.is_some() {
            log::debug!(
                "Config found for \"{}\": \n{:?}",
                &module_name,
                &module_config
            );
        } else {
            log::trace!("No config found for \"{}\"", &module_name);
        }

        module_config
    }

    /// Get the config value for a given key
    fn get_config(&self, key: &str) -> Option<&toml::value::Value> {
        log::trace!("Looking for config key \"{}\"", key);
        let config_value = self.get(key);

        if config_value.is_some() {
            log::trace!("Config found for \"{}\": {:?}", key, &config_value);
        } else {
            log::trace!("No value found for \"{}\"", key);
        }

        config_value
    }

    /// Get a key from a module's configuration as a boolean
    fn get_as_bool(&self, key: &str) -> Option<bool> {
        let value = self.get_config(key)?;
        let bool_value = value.as_bool();

        if bool_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a boolean. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        bool_value
    }

    /// Get a key from a module's configuration as a string
    fn get_as_str(&self, key: &str) -> Option<&str> {
        let value = self.get_config(key)?;
        let str_value = value.as_str();

        if str_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a string. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        str_value
    }

    /// Get a key from a module's configuration as an integer
    fn get_as_i64(&self, key: &str) -> Option<i64> {
        let value = self.get_config(key)?;
        let i64_value = value.as_integer();

        if i64_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be an integer. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        i64_value
    }

    /// Get a key from a module's configuration as a vector
    fn get_as_array(&self, key: &str) -> Option<&Vec<toml::value::Value>> {
        let value = self.get_config(key)?;
        let array_value = value.as_array();
        if array_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a array. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }
        array_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_get_as_bool() {
        let mut table = toml::value::Table::new();

        // Use with boolean value
        table.insert(String::from("boolean"), toml::value::Value::Boolean(true));
        assert_eq!(table.get_as_bool("boolean"), Some(true));

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("true")),
        );
        assert_eq!(table.get_as_bool("string"), None);
    }

    #[test]
    fn table_get_as_str() {
        let mut table = toml::value::Table::new();

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("hello")),
        );
        assert_eq!(table.get_as_str("string"), Some("hello"));

        // Use with boolean value
        table.insert(String::from("boolean"), toml::value::Value::Boolean(true));
        assert_eq!(table.get_as_str("boolean"), None);
    }

    #[test]
    fn table_get_as_i64() {
        let mut table = toml::value::Table::new();

        // Use with integer value
        table.insert(String::from("integer"), toml::value::Value::Integer(82));
        assert_eq!(table.get_as_i64("integer"), Some(82));

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("82")),
        );
        assert_eq!(table.get_as_bool("string"), None);
    }
}
