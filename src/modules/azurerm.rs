use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::{Context, Module, ModuleConfig};

use crate::configs::azurerm::AzureRMConfig;
use crate::formatter::StringFormatter;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AzureRMContext {
    default_context_key: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    contexts: HashMap<String, PSAzureContext>,
}

// TODO: Remove Clone trait
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureContext {
    // #[serde(deserialize_with = "parse_azurerm_subscription")]
    subscription: Option<PSAzureSubscription>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureSubscription {
    name: String,
    id: String,
    extended_properties: PSAzureSubscriptionProperties
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PSAzureSubscriptionProperties {
    environment: String,
    account: String,
    home_tenant: String
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
    let subscription = azurerm_context.subscription.unwrap();
        
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
                "username" => Some(Ok(&subscription.extended_properties.account)),
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
    let azurerm_context = azurerm_contexts
        .contexts
        .get(&azurerm_context_key)?
        .to_owned();

    if azurerm_context.subscription.is_none() {
      return None;
    }

    return Some(azurerm_context);
    
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

#[cfg(test)]
mod tests {
    use crate::modules::azurerm::load_azurerm_context;
    use crate::test::ModuleRenderer;
    use ini::Ini;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::PathBuf;

    use tempfile::TempDir;

    fn generate_test_config(dir: &TempDir, azurerm_context_contents: &str) -> io::Result<()> {
        save_string_to_file(
            dir,
            azurerm_context_contents.to_string(),
            String::from("AzureRmContext.json"),
        )?;
        
        Ok(())
    }

    fn save_string_to_file(
        dir: &TempDir,
        contents: String,
        file_name: String,
    ) -> Result<PathBuf, io::Error> {
        let bom_file_path = dir.path().join(file_name);
        let mut bom_file = File::create(&bom_file_path)?;
        bom_file.write_all(contents.as_bytes())?;
        bom_file.sync_all()?;
        Ok(bom_file_path)
    }

    #[test]
    fn subscription_set_correctly() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
              "Subscription 1": {
                "Subscription": {
                  "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                  "Name": "Subscription 1",
                  "State": "Enabled",
                  "ExtendedProperties": {
                    "AuthorizationSource": "RoleBased",
                    "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                    "Environment": "AzureCloud",
                    "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                    "Account": "user@domain.com"
                  }
                },
                "VersionProfile": null,
                "TokenCache": {
                  "CacheData": null
                },
                "ExtendedProperties": {}
              }
            },
            "ExtendedProperties": {}
          }"#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("󰠅 Subscription 1")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn subscription_name_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
        {
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
                "Subscription 1": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
            }        
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }


    #[test]
    fn subscription_name_missing_from_profile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
        {
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
                "Subscription 1": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
            }        
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }


    #[test]
    fn user_name_set_correctly() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
                "Subscription 1": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "Subscription 1",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
        }         
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Blue.bold().paint("󰠅 user@domain.com")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }
    
    #[test]
    fn user_name_missing_from_profile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
              "Subscription 1": {
                "Subscription": {
                  "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                  "Name": "Subscription 1",
                  "State": "Enabled",
                  "ExtendedProperties": {
                    "AuthorizationSource": "RoleBased",
                    "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                    "Environment": "AzureCloud",
                    "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                  }
                },
                "VersionProfile": null,
                "TokenCache": {
                  "CacheData": null
                },
                "ExtendedProperties": {}
              }
            },
            "ExtendedProperties": {}
          }          
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }


    #[test]
    fn user_name_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"
        {
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
                "Subscription 1": {
                "Subscription": {
                    "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                    "Name": "Subscription 1",
                    "State": "Enabled",
                    "ExtendedProperties": {
                        "AuthorizationSource": "RoleBased",
                        "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                        "Environment": "AzureCloud",
                        "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                        "Account": ""
                    }
                },
                "VersionProfile": null,
                "TokenCache": {
                    "CacheData": null
                },
                "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
        }        
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Blue.bold().paint("󰠅 Subscription 1:")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }


    #[test]
    fn subscription_name_and_username_found() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Subscription 1",
            "EnvironmentTable": {},
            "Contexts": {
              "Subscription 1": {
                "Subscription": {
                  "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                  "Name": "Subscription 1",
                  "State": "Enabled",
                  "ExtendedProperties": {
                    "AuthorizationSource": "RoleBased",
                    "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                    "Environment": "AzureCloud",
                    "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                    "Account": "user@domain.com"
                  }
                },
                "VersionProfile": null,
                "TokenCache": {
                  "CacheData": null
                },
                "ExtendedProperties": {}
              }
            },
            "ExtendedProperties": {}
          }          
        "#;

        generate_test_config(&dir, azurerm_context_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azurerm")
            .config(toml::toml! {
            [azurerm]
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Blue.bold().paint("󰠅 Subscription 1:user@domain.com")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }


    #[test]
    fn subscription_name_with_alias() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "VeryLongSubscriptionName",
            "EnvironmentTable": {},
            "Contexts": {
                "Subscription 1": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "Subscription 1",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                },
                "VeryLongSubscriptionName": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "VeryLongSubscriptionName",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
        }"#;

            generate_test_config(&dir, azurerm_context_contents)?;
            let dir_path = &dir.path().to_string_lossy();
            let actual = ModuleRenderer::new("azurerm")
                .config(toml::toml! {
                    [azurerm]
                    format = "on [$symbol($subscription:$username)]($style)"
                    disabled = false
                    [azurerm.subscription_aliases]
                    VeryLongSubscriptionName = "vlsn"
                    AnotherLongSubscriptionName = "alsn"
                    TheLastLongSubscriptionName = "tllsn"
                })
                .env("AZURE_CONFIG_DIR", dir_path.as_ref())
                .collect();
            let expected = Some(format!(
                "on {}",
                Color::Blue.bold().paint("󰠅 vlsn:user@domain.com")
            ));
            assert_eq!(actual, expected);
            dir.close()
    }

    #[test]
    fn active_context_subscription_is_null() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Context A",
            "EnvironmentTable": {},
            "Contexts": {
                "Context A": {
                    "Subscription": null
                },
                "Context B": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "Subscription 1",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                },
                "Context C": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "VeryLongSubscriptionName",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
        }"#;

            generate_test_config(&dir, azurerm_context_contents)?;
            let dir_path = &dir.path().to_string_lossy();
            let actual = ModuleRenderer::new("azurerm")
                .config(toml::toml! {
                    [azurerm]
                    format = "on [$symbol($subscription)]($style)"
                    disabled = false
                })
                .env("AZURE_CONFIG_DIR", dir_path.as_ref())
                .collect();
            let expected = None;
            assert_eq!(actual, expected);
            dir.close()
    }

    #[test]
    fn inactive_context_subscription_is_null() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azurerm_context_contents = r#"{
            "DefaultContextKey": "Context B",
            "EnvironmentTable": {},
            "Contexts": {
                "Context A": {
                    "Subscription": null
                },
                "Context B": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "Subscription 1",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                },
                "Context C": {
                    "Subscription": {
                        "Id": "9602c987-ef4d-4fe9-befb-c89158c8ad20",
                        "Name": "VeryLongSubscriptionName",
                        "State": "Enabled",
                        "ExtendedProperties": {
                            "AuthorizationSource": "RoleBased",
                            "HomeTenant": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Environment": "AzureCloud",
                            "Tenants": "8cf684f2-bc34-4b60-b75d-a75ab47ee27b",
                            "Account": "user@domain.com"
                        }
                    },
                    "VersionProfile": null,
                    "TokenCache": {
                        "CacheData": null
                    },
                    "ExtendedProperties": {}
                }
            },
            "ExtendedProperties": {}
        }"#;

            generate_test_config(&dir, azurerm_context_contents)?;
            let dir_path = &dir.path().to_string_lossy();
            let actual = ModuleRenderer::new("azurerm")
                .config(toml::toml! {
                    [azurerm]
                    disabled = false
                })
                .env("AZURE_CONFIG_DIR", dir_path.as_ref())
                .collect();
            let expected = Some(format!(
                "on {} ",
                Color::Blue.bold().paint("󰠅 Subscription 1")
            ));
            assert_eq!(actual, expected);
            dir.close()
    }

    #[test]
    fn files_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let dir_path = &dir.path().to_string_lossy();

        let actual = ModuleRenderer::new("azurerm")
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }

}