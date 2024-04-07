use serde::{Deserialize, Serialize, Deserializer};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use super::{Context, Module, ModuleConfig};

use crate::configs::azurerm::AzureRMConfig;
use crate::formatter::StringFormatter;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AzureRMContext {
    default_context_key: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    contexts: HashMap<String, PSAzureContext>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureContext {
    account: PSAzureAccount,
    #[serde(default, deserialize_with="parse_azurerm_subscription")]
    subscription: PSAzureSubscription,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureAccount {
  id: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "PascalCase", default)]
struct PSAzureSubscription {
  name: String,
  id: String, 
}

fn parse_azurerm_subscription<'de, D>(d: D) -> Result<PSAzureSubscription, D::Error> where D: Deserializer<'de> {
  Deserialize::deserialize(d)
      .map(|x: Option<_>| {
          x.unwrap_or_default()
      })
}


pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("azurerm");
    let config = AzureRMConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let azurerm_context: Option<PSAzureContext> = get_azurerm_context_info(context);
    
    if azurerm_context.is_none() {
        log::info!("Could not find Context in AzureRmContext.json");
        return None;
    }
    let azurerm_context = azurerm_context.unwrap();
    let subscription = azurerm_context.subscription;
    let account = azurerm_context.account;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "subscription" => Some(Ok(config
                    .subscription_aliases
                    .get(&subscription.name)
                    .copied()
                    .unwrap_or(&subscription.name))),
                "username" => Some(Ok(&account.id)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `azurerm`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_azurerm_context_info(context: &Context) -> Option<PSAzureContext> {
    let mut config_path = get_config_file_location(context)?;
    config_path.push("AzureRmContext.json");

    let azurerm_contexts = load_azurerm_context(&config_path)?;
    let azurerm_context_key = azurerm_contexts.default_context_key;
    let azurerm_context = azurerm_contexts.contexts.get(&azurerm_context_key)?.to_owned();
  
    Some(azurerm_context)
}

fn load_azurerm_context(config_path: &PathBuf) -> Option<AzureRMContext> {
  let json_data = fs::read_to_string(config_path).ok()?;
  let sanitized_json_data = json_data.strip_prefix('\u{feff}').unwrap_or(&json_data);
  if let Ok(azurerm_contexts) = serde_json::from_str::<AzureRMContext>(sanitized_json_data) {
      Some(azurerm_contexts)
  } else {
      log::info!("Failed to parse AzureRM Context.");
      None
  }
}

fn get_config_file_location(context: &Context) -> Option<PathBuf> {
    context
        .get_env("AZURE_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let mut home = context.get_home()?;
            home.push(".azure");
            Some(home)
        })
}