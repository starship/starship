use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

use super::{Context, Module, RootModuleConfig};

use crate::configs::gcloud::GcloudConfig;
use crate::formatter::StringFormatter;

type Account = String;
type Project = String;
type Region = String;
type Active = String;

fn get_gcloud_account_from_config(current_config: &PathBuf) -> Option<Account> {
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

fn get_gcloud_project_from_config(current_config: &PathBuf) -> Option<Project> {
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

fn get_gcloud_region_from_config(current_config: &PathBuf) -> Option<Region> {
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

fn get_active_config(config_root: &PathBuf) -> Option<String> {
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
}

fn get_current_config_path() -> Option<PathBuf> {
    let config_dir = get_config_dir()?;
    let active_config = get_active_config(&config_dir)?;
    let current_config = config_dir.join(format!("configurations/config_{}", active_config));
    Some(current_config)
}

fn get_config_dir() -> Option<PathBuf> {
    let config_dir = env::var("CLOUDSDK_CONFIG")
        .ok()
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = dirs_next::home_dir()?;
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

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("gcloud");
    let config: GcloudConfig = GcloudConfig::try_load(module.config);

    let config_path = get_current_config_path()?;
    let gcloud_account = get_gcloud_account_from_config(&config_path);
    let gcloud_project = get_gcloud_project_from_config(&config_path);
    let gcloud_region = get_gcloud_region_from_config(&config_path);
    let config_dir = get_config_dir()?;
    let gcloud_active: Option<Active> = get_active_config(&config_dir);

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
                "account" => gcloud_account.as_ref().map(Ok),
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
