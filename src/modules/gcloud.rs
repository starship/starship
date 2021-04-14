use once_cell::sync::{Lazy, OnceCell};
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

use crate::configs::gcloud::GcloudConfig;
use crate::formatter::StringFormatter;
use crate::utils;

type Account = (String, Option<String>);

struct GcloudContext {
    config_name: String,
    config_path: PathBuf,
    config: OnceCell<String>,
}

impl GcloudContext {
    pub fn new(config_name: &str, config_path: &Path) -> Self {
        Self {
            config_name: config_name.to_string(),
            config_path: PathBuf::from(config_path),
            config: Default::default(),
        }
    }

    fn get_config(&self) -> Option<&str> {
        let config = self
            .config
            .get_or_try_init(|| utils::read_file(&self.config_path));
        match config {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn get_account(&self) -> Option<Account> {
        let config = self.get_config()?;
        let account_line = config
            .lines()
            .skip_while(|line| *line != "[core]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("account"))?;
        let account = account_line.splitn(2, '=').nth(1)?.trim();
        let mut segments = account.splitn(2, '@');
        Some((
            segments.next().map(String::from)?,
            segments.next().map(String::from),
        ))
    }

    pub fn get_project(&self) -> Option<String> {
        let config = self.get_config()?;
        let project_line = config
            .lines()
            .skip_while(|line| *line != "[core]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("project"))?;
        let project = project_line.splitn(2, '=').nth(1)?.trim();
        Some(project.to_string())
    }

    pub fn get_region(&self) -> Option<String> {
        let config = self.get_config()?;
        let region_line = config
            .lines()
            .skip_while(|line| *line != "[compute]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("region"))?;
        let region = region_line.splitn(2, '=').nth(1)?.trim();
        Some(region.to_string())
    }
}

fn get_current_config(context: &Context) -> Option<(String, PathBuf)> {
    let config_dir = get_config_dir(context)?;
    let name = get_active_config(context, &config_dir)?;
    let path = config_dir
        .join("configurations")
        .join(format!("config_{}", name));
    Some((name, path))
}

fn get_config_dir(context: &Context) -> Option<PathBuf> {
    context
        .get_env("CLOUDSDK_CONFIG")
        .map(PathBuf::from)
        .or_else(|| {
            let home = context.get_home()?;
            Some(home.join(".config").join("gcloud"))
        })
}

fn get_active_config(context: &Context, config_dir: &Path) -> Option<String> {
    context.get_env("CLOUDSDK_ACTIVE_CONFIG_NAME").or_else(|| {
        let path = config_dir.join("active_config");
        match utils::read_file(path) {
            Ok(data) => data.lines().next().map(String::from),
            Err(_) => None,
        }
    })
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("gcloud");
    let config: GcloudConfig = GcloudConfig::try_load(module.config);

    let (config_name, config_path) = get_current_config(context)?;
    let gcloud_context = GcloudContext::new(&config_name, &config_path);
    let account: Lazy<Option<Account>, _> = Lazy::new(|| gcloud_context.get_account());

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
                "account" => account
                    .deref()
                    .as_ref()
                    .map(|(account, _)| (*account).to_owned())
                    .map(Ok),
                "domain" => account
                    .deref()
                    .as_ref()
                    .and_then(|(_, domain)| (*domain).to_owned())
                    .map(Ok),
                "region" => gcloud_context
                    .get_region()
                    .map(|region| {
                        config
                            .region_aliases
                            .get(&region)
                            .map(|alias| (*alias).to_owned())
                            .unwrap_or(region)
                    })
                    .map(Ok),
                "project" => gcloud_context.get_project().map(Ok),
                "active" => Some(Ok(gcloud_context.config_name.to_owned())),
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
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
account = foo@example.com
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Blue.bold().paint("☁️  foo@example.com")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn account_with_custom_format_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let active_config_path = dir.path().join("active_config");
        let mut active_config_file = File::create(&active_config_path)?;
        active_config_file.write_all(b"default")?;

        create_dir(dir.path().join("configurations"))?;
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
account = foo@example.com
",
        )?;

        let actual = ModuleRenderer::new("gcloud")
            .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy())
            .config(toml::toml! {
                [gcloud]
                format = "on [$symbol$account(\\($region\\))]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️  foo")));

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
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
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
            Color::Blue.bold().paint("☁️  foo@example.com(us-central1)")
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
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
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
            Color::Blue.bold().paint("☁️  foo@example.com(uc1)")
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
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️  default1")));

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
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
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
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️  abc")));

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
        let config_default_path = dir.path().join("configurations").join("config_default");
        let mut config_default_file = File::create(&config_default_path)?;
        config_default_file.write_all(
            b"\
[core]
project = default
",
        )?;

        let config_overridden_path = dir.path().join("configurations").join("config_overridden");
        let mut config_overridden_file = File::create(&config_overridden_path)?;
        config_overridden_file.write_all(
            b"\
[core]
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
        let expected = Some(format!("on {} ", Color::Blue.bold().paint("☁️  overridden")));

        assert_eq!(actual, expected);
        dir.close()
    }
}
