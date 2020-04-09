use ansi_term::Color;
use std::fs::File;
use std::io::{self, Write};

use tempfile;

use crate::common::{self};
use tempfile::TempDir;

fn generate_test_config(dir: &TempDir, cloud_config_contents: &str, azure_profile_contents: &str) {
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
    let output = common::render_module("azure")
        .env("AZURE_CONFIG_DIR", dir_path.as_ref())
        .output()?;
    let expected = format!("on ☁️ {} ", Color::Blue.bold().paint("Subscription 1"));
    let actual = String::from_utf8(output.stdout).unwrap();
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
    let output = common::render_module("azure")
        .env("AZURE_CONFIG_DIR", dir_path.as_ref())
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
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
    let output = common::render_module("azure")
        .env("AZURE_CONFIG_DIR", dir_path.as_ref())
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn files_missing() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let dir_path = &dir.path().to_string_lossy();

    let output = common::render_module("azure")
        .env("AZURE_CONFIG_DIR", dir_path.as_ref())
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}
