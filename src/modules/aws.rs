use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use dirs::home_dir;

use super::{Context, Module};

use crate::config::{RootModuleConfig, SegmentConfig};
use crate::configs::aws::AwsConfig;

fn get_aws_region_from_config(aws_profile: &Option<String>) -> Option<String> {
    let config_location = env::var("AWS_CONFIG_FILE")
        .ok()
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = home_dir()?;
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

fn get_aws_region() -> Option<String> {
    env::var("AWS_DEFAULT_REGION")
        .ok()
        .or_else(|| {
            let aws_profile = env::var("AWS_PROFILE").ok();
            let aws_region = get_aws_region_from_config(&aws_profile);

            match (aws_profile, aws_region) {
                (Some(profile), Some(region)) => Some(format!("{}({})", region, profile)),
                (Some(profile), None) => Some(profile),
                (None, Some(region)) => Some(region),
                (None, None) => None,
            }
        })
        .or_else(|| env::var("AWS_REGION").ok())
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const AWS_PREFIX: &str = "on ";

    let aws_region = get_aws_region()?;
    if aws_region.is_empty() {
        return None;
    }

    let mut module = context.new_module("aws");
    let config: AwsConfig = AwsConfig::try_load(module.config);

    module.set_style(config.style);

    module.get_prefix().set_value(AWS_PREFIX);

    module.create_segment("symbol", &config.symbol);
    module.create_segment(
        "region",
        &SegmentConfig {
            value: &aws_region,
            style: None,
        },
    );

    Some(module)
}
