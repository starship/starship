use encoding_rs::*;
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

    let subscription_name = get_azure_subscription_name(context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map(|variable| match variable {
                "subscription" => Some(Ok(subscription_name.to_string())),
                _ => None,
            })
            .parse(None)
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

fn get_azure_config_file_location(context: &Context) -> Option<PathBuf> {
    context
        .get_env("AZURE_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let mut home = dirs_next::home_dir()?;
            home.push(".azure");
            Some(home)
        })
}

fn get_azure_subscription_name(context: &Context) -> Option<String> {
    let mut azure_profile_json = get_azure_config_file_location(context)?;
    azure_profile_json.push("azureProfile.json");

    let mut cloud_config = get_azure_config_file_location(context)?;
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

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io::{self, Write};

    use tempfile::TempDir;

    fn generate_test_config(
        dir: &TempDir,
        cloud_config_contents: &str,
        azure_profile_contents: &str,
    ) {
        let clouds_config_path = dir.path().join("clouds.config");
        let mut clouds_config_file = File::create(&clouds_config_path).unwrap();

        clouds_config_file
            .write_all(cloud_config_contents.as_bytes())
            .unwrap();

        let azure_profile_path = dir.path().join("azureProfile.json");
        let mut azure_profile_file = File::create(&azure_profile_path).unwrap();

        azure_profile_file
            .write_all(azure_profile_contents.as_bytes())
            .unwrap()
    }

    #[test]
    fn subscription_set_correctly() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let cloud_config_contents = r#"[AzureCloud]
subscription = f3935dc9-92b5-9a93-da7b-42c325d86939
"#;

        let azure_profile_contents = r#"{
    "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
    "subscriptions": [
      {
        "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
        "name": "Subscription 1",
        "state": "Enabled",
        "user": {
          "name": "user@domain.com",
          "type": "user"
        },
        "isDefault": false,
        "tenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
        "environmentName": "AzureCloud",
        "homeTenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
        "managedByTenants": []
      },
      {
        "id": "f568c543-d12e-de0b-3d85-69843598b565",
        "name": "Subscription 2",
        "state": "Enabled",
        "user": {
          "name": "user@domain.com",
          "type": "user"
        },
        "isDefault": true,
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
      }
    ]
  }
"#;

        generate_test_config(&dir, cloud_config_contents, azure_profile_contents);
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azure")
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = Some(format!(
            "on ☁️ {} ",
            Color::Blue.bold().paint("Subscription 1")
        ));
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn subscription_missmatch_between_cloud_config_and_azure_profile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let cloud_config_contents = r#"[AzureCloud]
subscription = f3935dc9-92b5-9a93-da7b-42c325d86940
"#;

        let azure_profile_contents = r#"{
    "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
    "subscriptions": [
      {
        "id": "f3935dc9-92b5-9a93-da7b-42c325d86939",
        "name": "Subscription 1",
        "state": "Enabled",
        "user": {
          "name": "user@domain.com",
          "type": "user"
        },
        "isDefault": false,
        "tenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
        "environmentName": "AzureCloud",
        "homeTenantId": "f0273a19-7779-e40a-00a1-53b8331b3bb6",
        "managedByTenants": []
      },
      {
        "id": "f568c543-d12e-de0b-3d85-69843598b565",
        "name": "Subscription 2",
        "state": "Enabled",
        "user": {
          "name": "user@domain.com",
          "type": "user"
        },
        "isDefault": true,
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
      }
    ]
  }
"#;

        generate_test_config(&dir, cloud_config_contents, azure_profile_contents);
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azure")
            .env("AZURE_CONFIG_DIR", dir_path.as_ref())
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn subscription_azure_profile_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let cloud_config_contents = r#"[AzureCloud]
subscription = f3935dc9-92b5-9a93-da7b-42c325d86940
"#;

        let azure_profile_contents = r#"{
    "installationId": "3deacd2a-b9db-77e1-aa42-23e2f8dfffc3",
    "subscriptions": []
  }
"#;

        generate_test_config(&dir, cloud_config_contents, azure_profile_contents);
        let dir_path = &dir.path().to_string_lossy();
        let actual = ModuleRenderer::new("azure")
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
}
