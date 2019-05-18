use crate::utils;

// use std::collections::HashMap;
use dirs::home_dir;

static config: toml::value::Table = toml::value::Table::new();
// static config: HashMap<String, ModuleConfig> = HashMap::new();

// enum ModuleConfig {
//     ConfigValue(HashMap<String, String>),
//     SegmentConfig(HashMap<String, HashMap<String, String>>)
// }

pub fn initialize() -> Option<()> {
    let file_path = home_dir()?.join(".config/starship.toml");
    let toml_content = utils::read_file(&file_path.to_str()?).ok()?;

    config = toml::from_str(&toml_content).ok()?;

    Some(())
}

pub fn get_config(config_value: &str, default: &str) -> String {
    default.to_string()
}
