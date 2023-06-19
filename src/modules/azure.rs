use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use super::{Context, Module, ModuleConfig};

use crate::configs::azure::AzureConfig;
use crate::formatter::StringFormatter;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AzureProfile {
    installation_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    subscriptions: Vec<Subscription>,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Subscription {
    name: String,
    user: User,
    is_default: bool,
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("azure");
    let config = AzureConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let subscription: Option<Subscription> = get_azure_profile_info(context);

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
                "username" => Some(Ok(&subscription.user.name)),
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

fn get_azure_profile_info(context: &Context) -> Option<Subscription> {
    let mut config_path = get_config_file_location(context)?;
    config_path.push("azureProfile.json");

    let azure_profile = load_azure_profile(&config_path)?;
    azure_profile
        .subscriptions
        .into_iter()
        .find(|s| s.is_default)
}

fn load_azure_profile(config_path: &PathBuf) -> Option<AzureProfile> {
    let json_data = fs::read_to_string(config_path).ok()?;
    let sanitized_json_data = json_data.strip_prefix('\u{feff}').unwrap_or(&json_data);
    if let Ok(azure_profile) = serde_json::from_str::<AzureProfile>(sanitized_json_data) {
        Some(azure_profile)
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

#[cfg(test)]
mod tests {
    use crate::modules::azure::load_azure_profile;
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
                "isDefault": false,
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
            Color::Blue.bold().paint("󰠅 Subscription 1")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn user_name_set_correctly() -> io::Result<()> {
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
                "isDefault": false,
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
    fn subscription_name_empty() -> io::Result<()> {
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
                "isDefault": false,
                "tenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "environmentName": "AzureCloud",
                "homeTenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "managedByTenants": []
              },
              {
                "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
                "name": "",
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
            format = "on [$symbol($subscription:$username)]($style)"
            disabled = false
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Blue.bold().paint("󰠅 :user@domain.com")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn user_name_empty() -> io::Result<()> {
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
                "isDefault": false,
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
                  "name": "",
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
    fn user_name_missing_from_profile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let azure_profile_contents = r#"{
            "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
            "subscriptions": [
                {
                "id": "f568c543-d12e-de0b-3d85-69843598b565",
                "name": "Subscription 2",
                "state": "Enabled",
                "user": {
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
                "isDefault": false,
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
                  "name": "",
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
                "isDefault": false,
                "tenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "environmentName": "AzureCloud",
                "homeTenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "managedByTenants": []
              },
              {
                "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
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
    fn subscription_name_and_username_found() -> io::Result<()> {
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
                "isDefault": false, 
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

        let azure_profile_contents = r#"{
            "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
            "subscriptions": [
                {
                "id": "f568c543-d12e-de0b-3d85-69843598b565",
                "name": "VeryLongSubscriptionName",
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
                "name": "AnotherLongSubscriptionName",
                "state": "Enabled",
                "user": {
                  "name": "user@domain.com",
                  "type": "user"
                },
                "isDefault": false, 
                "tenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "environmentName": "AzureCloud",
                "homeTenantId": "a4e1bb4b-5330-2d50-339d-b9674d3a87bc",
                "managedByTenants": []
              },
              {
                "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
                "name": "TheLastLongSubscriptionName",
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
                format = "on [$symbol($subscription:$username)]($style)"
                disabled = false
                [azure.subscription_aliases]
                VeryLongSubscriptionName = "vlsn"
                AnotherLongSubscriptionName = "alsn"
                TheLastLongSubscriptionName = "tllsn"
            })
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Blue.bold().paint("󰠅 tllsn:user@domain.com")
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

        //let azure_profile_contents = "\u{feff}{\"installationId\": \"2652263e-40f8-11ed-ae3b-367ddada549c\", \"subscriptions\": []}";
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
    fn azure_profile_with_leading_char() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let bom = vec![239, 187, 191];
        let mut bom_str = String::from_utf8(bom).unwrap();

        let json_str =
            r#"{"installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3", "subscriptions": []}"#;

        bom_str.push_str(json_str);

        let dir_path_no_bom = save_string_to_file(&dir, bom_str, String::from("bom.json"))?;
        let sanitized_json = load_azure_profile(&dir_path_no_bom).unwrap();

        assert_eq!(
            sanitized_json.installation_id,
            "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3"
        );
        assert!(sanitized_json.subscriptions.is_empty());
        dir.close()
    }

    #[test]
    fn azure_profile_without_leading_char() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let json_str =
            r#"{"installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3", "subscriptions": []}"#;

        let dir_path_no_bom =
            save_string_to_file(&dir, json_str.to_string(), String::from("bom.json"))?;
        let sanitized_json = load_azure_profile(&dir_path_no_bom).unwrap();

        assert_eq!(
            sanitized_json.installation_id,
            "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3"
        );
        assert!(sanitized_json.subscriptions.is_empty());
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
