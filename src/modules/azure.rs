use dirs::home_dir;
use encoding_rs::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

type JValue = serde_json::Value;

use crate::configs::azure::AzureConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("azure");
    let config = AzureConfig::try_load(module.config);

    let subscription_name = get_azure_subscription_name()?;

    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "subscription" => Some(subscription_name.to_string()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `azure.format`");
        return None;
    };

    module.set_segments(formatter.parse(None));

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn get_azure_config_file_location() -> Option<PathBuf> {
    env::var("AZURE_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let mut home = home_dir()?;
            home.push(".azure");
            Some(home)
        })
}

fn get_azure_subscription_name() -> Option<String> {
    let mut azure_profile_json = get_azure_config_file_location()?;
    azure_profile_json.push("azureProfile.json");

    let mut cloud_config = get_azure_config_file_location()?;
    cloud_config.push("clouds.config");

    let file = File::open(&cloud_config).ok()?;
    let reader = BufReader::new(file);
    let cloud_config_lines = reader.lines().filter_map(Result::ok);

    let subscription_line = cloud_config_lines
        .skip_while(|line| line != "[AzureCloud]")
        .find(|line| line.starts_with("subscription"))?;

    let current_subscription_id = subscription_line.split('=').nth(1)?.trim();

    let file = File::open(&azure_profile_json).ok()?;
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer).ok()?;
    let bytes = buffer.as_mut_slice();
    let decodedbuffer = UTF_8.decode_with_bom_removal(bytes).0;

    let parsed_json: JValue = serde_json::from_str(&decodedbuffer).ok()?;

    let subscriptions = parsed_json["subscriptions"].as_array()?;

    subscriptions
        .iter()
        .find_map(|s| find_subscription(s, current_subscription_id))
}

fn find_subscription(subscription: &JValue, current_subscription_id: &str) -> Option<String> {
    let subscription_id = subscription["id"].as_str()?;
    if subscription_id == current_subscription_id {
        let subscription_name = subscription["name"].as_str()?;
        return Some(subscription_name.to_string());
    }
    None
}
