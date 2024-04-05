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
    // #[serde(default, skip)]
    // environment_table: any,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    contexts: HashMap<String, PSAzureContext>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extended_properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureContext {
    account: PSAzureAccount,
    tenant: PSAzureTenant,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // subscription: Option<PSAzureSubscription>,
    #[serde(default, deserialize_with="parse_subscription", flatten)]
    subscription: PSAzureSubscription,
    environment: PSAzureEnvironment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version_profile: Option<String>,
    // #[serde(default, skip)]
    // token_cache: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extended_properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "PascalCase", default)]
struct PSAzureSubscription {
  name: String,
  id: String,
  state: String,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  extended_properties: HashMap<String, String>,
}

fn parse_subscription<'de, D>(d: D) -> Result<PSAzureSubscription, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or_default()
        })
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureAccount {
  id: String,
  #[serde(rename(deserialize = "Type"))]
  account_type: String,
  credential: Option<String>,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  extended_properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureTenant {
  id: String,
  directory: Option<String>,
  is_home: bool,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  extended_properties: HashMap<String, String>,
}



#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureEnvironment {
  name: String,
  #[serde(rename(deserialize = "Type"))]
  environment_type: String,
  on_premise: bool,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  extended_properties: HashMap<String, String>,
  // ad_tenant: String
}


pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("azurerm");
    let config = AzureRMConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let subscription: Option<PSAzureSubscription> = get_azure_profile_info(context);

    if subscription.is_none() {
        log::info!("Could not find Subscriptions in azureProfile.json");
        return None;
    }

    let subscription = subscription.unwrap();

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
                // "username" => Some(Ok(&subscription.user.name)),
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

fn get_azure_profile_info(context: &Context) -> Option<PSAzureSubscription> {
    let mut config_path = get_config_file_location(context)?;
    config_path.push("AzureRmContext.json");

    let azurerm_contexts = load_azure_profile(&config_path)?;
    let azurerm_context_key = &azurerm_contexts.default_context_key;
    let azurerm_context  = azurerm_contexts.contexts.get(azurerm_context_key).unwrap();
       
    // let context = 
    //     azurerm_contexts
    //     .contexts
    //     .get(azurerm_context_key).unwrap_or_default();
    Some(azurerm_context.to_owned().subscription)
}

fn load_azure_profile(config_path: &PathBuf) -> Option<AzureRMContext> {
    let json_data = fs::read_to_string(config_path).ok()?;
    let sanitized_json_data = json_data.strip_prefix('\u{feff}').unwrap_or(&json_data);
    if let Ok(azurerm_contexts) = serde_json::from_str::<AzureRMContext>(sanitized_json_data) {
        // let azurerm_context_key = azure_profile.default_context_key;
        // let azurerm_context = azure_profile.contexts.get(&azurerm_context_key);
        // Some(azurerm_context)
        Some(azurerm_contexts)
    } else {
        log::info!("Failed to parse azure profile.");
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