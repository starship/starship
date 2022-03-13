use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;

use super::{Context, Module, RootModuleConfig};

use crate::configs::aws::AwsConfig;
use crate::formatter::StringFormatter;
use crate::utils::{read_file, render_time};

type Profile = String;
type Region = String;

fn get_credentials_file_path(context: &Context) -> Option<PathBuf> {
    context
        .get_env("AWS_CREDENTIALS_FILE")
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = context.get_home()?;
            home.push(".aws/credentials");
            Some(home)
        })
}

fn get_config_file_path(context: &Context) -> Option<PathBuf> {
    context
        .get_env("AWS_CONFIG_FILE")
        .and_then(|path| PathBuf::from_str(&path).ok())
        .or_else(|| {
            let mut home = context.get_home()?;
            home.push(".aws/config");
            Some(home)
        })
}

fn get_aws_region_from_config(context: &Context, aws_profile: Option<&str>) -> Option<Region> {
    let config_location = get_config_file_path(context)?;

    let contents = read_file(&config_location).ok()?;

    let region_line = if let Some(aws_profile) = aws_profile {
        contents
            .lines()
            .skip_while(|line| line != &format!("[profile {}]", &aws_profile))
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("region"))
    } else {
        contents
            .lines()
            .skip_while(|&line| line != "[default]")
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("region"))
    }?;

    let region = region_line.split('=').nth(1)?;
    let region = region.trim();

    Some(region.to_string())
}

fn get_aws_profile_and_region(context: &Context) -> (Option<Profile>, Option<Region>) {
    let profile_env_vars = vec!["AWSU_PROFILE", "AWS_VAULT", "AWSUME_PROFILE", "AWS_PROFILE"];
    let region_env_vars = vec!["AWS_REGION", "AWS_DEFAULT_REGION"];
    let profile = profile_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var));
    let region = region_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var));
    match (profile, region) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(ref p), None) => (
            Some(p.clone()),
            get_aws_region_from_config(context, Some(p)),
        ),
        (None, None) => (None, get_aws_region_from_config(context, None)),
    }
}

fn get_credentials_duration(context: &Context, aws_profile: Option<&Profile>) -> Option<i64> {
    let expiration_env_vars = vec!["AWS_SESSION_EXPIRATION", "AWSUME_EXPIRATION"];
    let expiration_date = if let Some(expiration_date) = expiration_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var))
    {
        chrono::DateTime::parse_from_rfc3339(&expiration_date).ok()
    } else {
        let contents = read_file(get_credentials_file_path(context)?).ok()?;

        let profile_line = if let Some(aws_profile) = aws_profile {
            format!("[{}]", aws_profile)
        } else {
            "[default]".to_string()
        };

        let expiration_date_line = contents
            .lines()
            .skip_while(|line| line != &profile_line)
            .skip(1)
            .take_while(|line| !line.starts_with('['))
            .find(|line| line.starts_with("expiration"))?;

        let expiration_date = expiration_date_line.split('=').nth(1)?.trim();
        DateTime::parse_from_rfc3339(expiration_date).ok()
    }?;

    Some(expiration_date.timestamp() - chrono::Local::now().timestamp())
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

    let (aws_profile, aws_region) = get_aws_profile_and_region(context);
    if aws_profile.is_none() && aws_region.is_none() {
        return None;
    }

    let mapped_region = if let Some(aws_region) = aws_region {
        Some(alias_region(aws_region, &config.region_aliases))
    } else {
        None
    };

    let duration = {
        get_credentials_duration(context, aws_profile.as_ref()).map(|duration| {
            if duration > 0 {
                render_time((duration * 1000) as u128, false)
            } else {
                config.expiration_symbol.to_string()
            }
        })
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
                "duration" => duration.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `aws`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{create_dir, File};
    use std::io::{self, Write};

    #[test]
    #[ignore]
    fn no_region_set() {
        let actual = ModuleRenderer::new("aws").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-2")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_set_with_alias() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-southeast-2")
            .config(toml::toml! {
                [aws.region_aliases]
                ap-southeast-2 = "au"
            })
            .collect();
        let expected = Some(format!("on {}", Color::Yellow.bold().paint("☁️  (au) ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_region_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_DEFAULT_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_set_from_aws_vault() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_VAULT", "astronauts-vault")
            .env("AWS_PROFILE", "astronauts-profile")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts-vault ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_set_from_awsu() {
        let actual = ModuleRenderer::new("aws")
            .env("AWSU_PROFILE", "astronauts-awsu")
            .env("AWS_PROFILE", "astronauts-profile")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts-awsu ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_set_from_awsume() {
        let actual = ModuleRenderer::new("aws")
            .env("AWSUME_PROFILE", "astronauts-awsume")
            .env("AWS_PROFILE", "astronauts-profile")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts-awsume ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_and_region_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn credentials_file_is_ignored_when_is_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("credentials");
        create_dir(&config_path)?;

        assert!(ModuleRenderer::new("aws")
            .env(
                "AWS_CREDENTIALS_FILE",
                config_path.to_string_lossy().as_ref(),
            )
            .collect()
            .is_none());

        dir.close()
    }

    #[test]
    fn config_file_path_is_ignored_when_is_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        create_dir(&config_path)?;

        assert!(ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .collect()
            .is_none());

        dir.close()
    }

    #[test]
    fn default_profile_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[default]
region = us-east-1

[profile astronauts]
region = us-east-2
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (us-east-1) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn profile_and_config_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[default]
region = us-east-1

[profile astronauts]
region = us-east-2
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env("AWS_PROFILE", "astronauts")
            .config(toml::toml! {
                [aws]
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts (us-east-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn profile_and_region_set_with_display_all() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-1) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_set_with_display_all() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_set_with_display_all() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-1) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_and_region_set_with_display_region() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_DEFAULT_REGION", "ap-northeast-1")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$region]($style) "
            })
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  ap-northeast-1")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profile_and_region_set_with_display_profile() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-1")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$profile]($style) "
            })
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_set_with_display_profile() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-1")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$profile]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Yellow.bold().paint("☁️  ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn expiration_date_set() {
        use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};

        let now_plus_half_hour: DateTime<Utc> = chrono::DateTime::from_utc(
            NaiveDateTime::from_timestamp(chrono::Local::now().timestamp() + 1800, 0),
            Utc,
        );

        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                show_duration = true
            })
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env(
                "AWS_SESSION_EXPIRATION",
                now_plus_half_hour.to_rfc3339_opts(SecondsFormat::Secs, true),
            )
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-2) [30m]")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn expiration_date_set_from_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let credentials_path = dir.path().join("credentials");
        let mut file = File::create(&credentials_path)?;

        use chrono::{DateTime, NaiveDateTime, Utc};

        let now_plus_half_hour: DateTime<Utc> = chrono::DateTime::from_utc(
            NaiveDateTime::from_timestamp(chrono::Local::now().timestamp() + 1800, 0),
            Utc,
        );

        let expiration_date = now_plus_half_hour.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        file.write_all(
            format!(
                "[astronauts]
aws_access_key_id=dummy
aws_secret_access_key=dummy
expiration={}
",
                expiration_date
            )
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                show_duration = true
            })
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env(
                "AWS_CREDENTIALS_FILE",
                credentials_path.to_string_lossy().as_ref(),
            )
            .collect();

        // In principle, "30m" should be correct. However, bad luck in scheduling
        // on shared runners may delay it. Allow for up to 2 seconds of delay.
        let possible_values = ["30m", "29m59s", "29m58s"];
        let possible_values = possible_values.map(|duration| {
            let segment_colored = format!("☁️  astronauts (ap-northeast-2) [{}]", duration);
            Some(format!(
                "on {}",
                Color::Yellow.bold().paint(segment_colored)
            ))
        });

        assert!(possible_values.contains(&actual));

        dir.close()
    }

    #[test]
    fn profile_and_region_set_show_duration() {
        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                show_duration = true
            })
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn expiration_date_set_expired() {
        use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};

        let now: DateTime<Utc> = chrono::DateTime::from_utc(
            NaiveDateTime::from_timestamp(chrono::Local::now().timestamp() - 1800, 0),
            Utc,
        );

        let symbol = "!!!";

        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                show_duration = true
                expiration_symbol = symbol
            })
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env(
                "AWS_SESSION_EXPIRATION",
                now.to_rfc3339_opts(SecondsFormat::Secs, true),
            )
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint(format!("☁️  astronauts (ap-northeast-2) [{}]", symbol))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn region_not_set_with_display_region() {
        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$region]($style) "
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
