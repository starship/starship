use crate::utils;

use dirs::home_dir;

pub struct Config {
    data: toml::value::Table,
}

impl Config {
    pub fn initialize() -> Config {
        if let Some(file_data) = Config::config_from_file() {
            return Config {
                data: file_data
            };
        }

        Config {
            data: toml::value::Table::new(),
        }
    }

    fn config_from_file() -> Option<toml::value::Table> {
        let file_path = home_dir()?.join(".config/starship.toml");
        let toml_content = utils::read_file(&file_path.to_str()?).ok()?;

        let config = toml::from_str(&toml_content).ok()?;
        Some(config)
    }

    pub fn get_config(config_value: &str, default: &str) -> String {
        default.to_string()
    }
}
