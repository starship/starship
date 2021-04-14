use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, path::Path};

use super::{Context, Module, RootModuleConfig};

use crate::configs::gcloud::GcloudConfig;
use crate::formatter::StringFormatter;

type Account = String;
type Project = String;
type Region = String;
type Active = String;

fn get_gcloud_account_from_config(current_config: &Path) -> Option<Account> {
    let file = File::open(&current_config).ok()?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let account_line = lines
        .skip_while(|line| line != "[core]")
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .find(|line| line.starts_with("account"))?;
    let account = account_line.split('=').nth(1)?.trim();
    Some(account.to_string())
}

fn get_gcloud_project_from_config(current_config: &Path) -> Option<Project> {
    let file = File::open(&current_config).ok()?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let project_line = lines
        .skip_while(|line| line != "[core]")
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .find(|line| line.starts_with("project"))?;
    let project = project_line.split('=').nth(1)?.trim();
    Some(project.to_string())
}

fn get_gcloud_region_from_config(current_config: &Path) -> Option<Region> {
    let file = File::open(&current_config).ok()?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let region_line = lines
        .skip_while(|line| line != "[compute]")
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .find(|line| line.starts_with("region"))?;
    let region = region_line.split('=').nth(1)?.trim();
    Some(region.to_string())
}

fn get_active_config(context: &Context, config_root: &Path) -> Option<String> {
    let config_name = context.get_env("CLOUDSDK_ACTIVE_CONFIG_NAME").or_else(|| {
        let path = config_root.join("active_config");
        let file = File::open(&path).ok()?;
        let reader = BufReader::new(file);
        let first_line = match reader.lines().next() {
            Some(res) => res,
            None => Err(Error::new(ErrorKind::NotFound, "empty")),
        };
        match first_line {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    })?;
    Some(config_name)
}

fn get_current_config_path(context: &Context) -> Option<PathBuf> {
    let config_dir = get_config_dir(context)?;
    let active_config = get_active_config(context, &config_dir)?;
    let current_config = config_dir.join(format!("configurations/config_{}", active_config));
    Some(current_config)
}

fn get_config_dir(context: &Context) -> Option<PathBuf> {
    let config_dir = context
        .get_env("CLOUDSDK_CONFIG")
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = context.get_home()?;
            home.push(".config/gcloud");
            Some(home)
        })?;
    Some(config_dir)
}

fn alias_region(region: String, aliases: &HashMap<String, &str>) -> String {
    match aliases.get(&region) {
        None => region.to_string(),
        Some(alias) => (*alias).to_string(),
    }
}

fn map_account(account: String) -> (Option<String>, Option<String>) {
    match account.find('@') {
        Some(index) => {
            let (account, at) = account.split_at(index);
            (Some(account.to_owned()), Some(at.to_owned()))
        }
        None => (Some(account), None),
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("gcloud");
    let config: GcloudConfig = GcloudConfig::try_load(module.config);

    let config_path = get_current_config_path(context)?;
    let gcloud_account = get_gcloud_account_from_config(&config_path);
    let gcloud_project = get_gcloud_project_from_config(&config_path);
    let gcloud_region = get_gcloud_region_from_config(&config_path);
    let config_dir = get_config_dir(context)?;
    let gcloud_active: Option<Active> = get_active_config(context, &config_dir);

    if gcloud_account.is_none()
        && gcloud_project.is_none()
        && gcloud_region.is_none()
        && gcloud_active.is_none()
    {
        return None;
    }

    let mapped_region = if let Some(gcloud_region) = gcloud_region {
        Some(alias_region(gcloud_region, &config.region_aliases))
    } else {
        None
    };

    let (account, at) = match gcloud_account {
        Some(account) => map_account(account),
        None => (None, None),
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
                "account" => account.as_ref().map(Ok),
                "account_at" => at.as_ref().map(Ok),
                "project" => gcloud_project.as_ref().map(Ok),
                "region" => mapped_region.as_ref().map(Ok),
                "active" => gcloud_active.as_ref().map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::error!("Error in module `gcloud`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, File};
    use std::io::{self, Write};

    use ansi_term::Color;

    use crate::test::ModuleRenderer;

    #[test]
    fn account_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations/config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"[core]
account = foo@example.com
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("☁️ foo@example.com")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn account_and_region_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations/config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"[core]
account = foo@example.com

[compute]
region = us-central1
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("☁️ foo@example.com(us-central1)")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn account_and_region_set_with_alias() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations/config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"[core]
account = foo@example.com

[compute]
region = us-central1
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .config(toml::toml! {
                [gcloud.region_aliases]
                us-central1 = "uc1"
            })
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("☁️ foo@example.com(uc1)")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn active_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default1")?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .config(toml::toml! {
                [gcloud]
                format = "on [$symbol$active]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️ default1")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn project_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations/config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"[core]
project = abc
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .config(toml::toml! {
                [gcloud]
                format = "on [$symbol$project]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️ abc")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn region_not_set_with_display_region() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .config(toml::toml! {
                [gcloud]
                format = "on [$symbol$region]($style) "
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn active_config_manually_overridden() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations/config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"[core]
project = default
",
        )?;

        let config_overridden_path = dir.path().join("configurations/config_overridden");
        let mut config_overridden_file = File::create(&config_overridden_path)?;
        config_overridden_file.write_all(
            b"[core]
project = overridden
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .env("CLOUDSDK_ACTIVE_CONFIG_NAME", "overridden")
            .config(toml::toml! {
                [gcloud]
                format = "on [$symbol$project]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️ overridden")));

        assert_eq!(actual, expected);
        dir.close()
    }
}
