use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use super::{Context, Module, RootModuleConfig};

use crate::configs::aws::AwsConfig;
use crate::formatter::StringFormatter;

type Profile = String;
type Region = String;

fn get_aws_region_from_config(aws_profile: Option<&str>) -> Option<Region> {
    let config_location = env::var("AWS_CONFIG_FILE")
        .ok()
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = dirs_next::home_dir()?;
            home.push(".aws/config");
            Some(home)
        })?;

    let file = File::open(&config_location).ok()?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);

    let region_line = if let Some(ref aws_profile) = aws_profile {
        lines
            .skip_while(|line| line != &format!("[profile {}]", aws_profile))
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("region"))
    } else {
        lines
            .skip_while(|line| line != "[default]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("region"))
    }?;

    let region = region_line.split('=').nth(1)?;
    let region = region.trim();

    Some(region.to_string())
}

fn get_aws_profile_and_region() -> (Option<Profile>, Option<Region>) {
    match (
        env::var("AWS_VAULT")
            .or_else(|_| env::var("AWS_PROFILE"))
            .ok(),
        env::var("AWS_DEFAULT_REGION")
            .or_else(|_| env::var("AWS_REGION"))
            .ok(),
    ) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(ref p), None) => (Some(p.to_owned()), get_aws_region_from_config(Some(p))),
        (None, None) => (None, get_aws_region_from_config(None)),
    }
}

fn alias_region(region: String, aliases: &HashMap<String, &str>) -> String {
    match aliases.get(&region) {
        None => region,
        Some(alias) => (*alias).to_string(),
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("aws");
    let config: AwsConfig = AwsConfig::try_load(module.config);

    let (aws_profile, aws_region) = get_aws_profile_and_region();
    if aws_profile.is_none() && aws_region.is_none() {
        return None;
    }

    let mapped_region = if let Some(aws_region) = aws_region {
        Some(alias_region(aws_region, &config.region_aliases))
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
                "profile" => aws_profile.as_ref().map(Ok),
                "region" => mapped_region.as_ref().map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::error!("Error in module `aws`: \n{}", error);
            return None;
        }
    });

    Some(module)
}
