use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use super::{Context, Module, ModuleConfig};

type JValue = serde_json::Value;

use crate::configs::azure::AzureConfig;
use crate::formatter::StringFormatter;

type SubscriptionName = String;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("azure");
    let config = AzureConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let subscription_name: Option<SubscriptionName> = get_azure_subscription_name(context);
    if subscription_name.is_none() {
        log::info!("Could not find Azure subscription name");
        return None;
    };

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
                "subscription" => Some(Ok(subscription_name.as_ref().unwrap())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `azure`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_azure_subscription_name(context: &Context) -> Option<SubscriptionName> {
    let mut config_path = get_config_file_location(context)?;
    config_path.push("azureProfile.json");

    let parsed_json = parse_json(&config_path)?;

    let subscriptions = parsed_json.get("subscriptions")?.as_array()?;
    let subscription_name = subscriptions.iter().find_map(|s| {
        if s.get("isDefault")? == true {
            Some(s.get("name")?.as_str()?.to_string())
        } else {
            None
        }
    });
    if subscription_name.is_some() {
        subscription_name
    } else {
        log::info!("Could not find subscription name");
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

fn parse_json(json_file_path: &Path) -> Option<JValue> {
    let mut buffer: Vec<u8> = Vec::new();

    let json_file = File::open(json_file_path).ok()?;
    let mut reader = BufReader::new(json_file);
    reader.read_to_end(&mut buffer).ok()?;

    let bytes = buffer.as_mut_slice();
    let decodedbuffer = bytes.strip_prefix(&[239, 187, 191]).unwrap_or(bytes);

    if let Ok(parsed_json) = serde_json::from_slice(decodedbuffer) {
        Some(parsed_json)
    } else {
        log::info!("Failed to parse json");
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::azure::parse_json;
    use crate::test::ModuleRenderer;
    use ini::Ini;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::PathBuf;

    use tempfile::TempDir;

    fn generate_test_config(dir: &TempDir, azure_profile_contents: &str) -> io::Result<()> {
        save_string_to_file(
            dir,
            azure_profile_contents.to_string(),
            String::from("azureProfile.json"),
        )?;

        Ok(())
    }
    #[test]
    fn subscription_set_correctly() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azure_profile_contents = r#"{
            "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
            "subscriptions": [
                {
                "id": "f568c543-d12e-de0b-3d85-69843598b565",
                "name": "Subscription 2",
                "state": "Enabled",
                "user": {
                  "name": "user@domain.com",
                  "type": "user"
                },
                "isDefault": false,
                "tenantId": "0e8a15ec-b0f5-d355-7062-8ece54c59aee",
                "environmentName": "AzureCloud",
                "homeTenantId": "0e8a15ec-b0f5-d355-7062-8ece54c59aee",
                "managedByTenants": []
              },
              {
                "id": "d4442d26-ea6d-46c4-07cb-4f70b8ae5465",
                "name": "Subscription 3",
                "state": "Enabled",
                "user": {
                  "name": "user@domain.com",
                  "type": "user"
                },
                "tenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "environmentName": "AzureCloud",
                "homeTenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "managedByTenants": []
              },
              {
                "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
                "name": "Subscription 1",
                "state": "Enabled",
                "user": {
                  "name": "user@domain.com",
                  "type": "user"
                },
                "isDefault": true,
                "tenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
                "environmentName": "AzureCloud",
                "homeTenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
                "managedByTenants": []
              }
            ]
          }
        "#;

        generate_test_config(&dir, azure_profile_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azure")
            .config(toml::toml! {
            [azure]
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("ﴃ Subscription 1")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn subscription_azure_profile_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let mut clouds_config_ini = Ini::new();
        clouds_config_ini
            .with_section(Some("AzureCloud"))
            .set("subscription", "f3935dc9-92b5-9a93-da7b-42c325d86939");

        let azure_profile_contents = r#"{
            "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
            "subscriptions": []
          }
        "#;

        generate_test_config(&dir, azure_profile_contents)?;
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azure")
            .config(toml::toml! {
              [azure]
              disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn files_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let dir_path = &dir.path().to_string_lossy();

        let actual = ModuleRenderer::new("azure")
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn json_parsing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let bom = vec![239, 187, 191];
        let mut bom_str = String::from_utf8(bom).unwrap();
        let json_str = r#"{"testKey": "testValue"}"#;
        bom_str.push_str(json_str);

        let dir_path_no_bom = save_string_to_file(&dir, bom_str, String::from("bom.json"))?;
        let parsed_json = parse_json(&dir_path_no_bom).unwrap();

        assert_eq!(parsed_json.get("testKey").unwrap(), "testValue");
        dir.close()
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
}
