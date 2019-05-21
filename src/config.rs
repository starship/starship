use crate::utils;

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
        let file_path = home_dir()?.join(".config/starship.toml");
        let toml_content = utils::read_file(&file_path.to_str()?).ok()?;
        log::trace!("Config file content: \n{}", &toml_content);

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
