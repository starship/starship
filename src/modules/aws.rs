use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;
use ini::Ini;
use once_cell::unsync::OnceCell;

use super::{Context, Module, ModuleConfig};

use crate::configs::aws::AwsConfig;
use crate::formatter::StringFormatter;
use crate::utils::render_time;

type Profile = String;
type Region = String;
type AwsConfigFile = OnceCell<Option<Ini>>;
type AwsCredsFile = OnceCell<Option<Ini>>;

fn get_credentials_file_path(context: &Context) -> Option<PathBuf> {
    context
        .get_env("AWS_SHARED_CREDENTIALS_FILE")
        .or_else(|| context.get_env("AWS_CREDENTIALS_FILE"))
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

// Initialize the AWS config file once
fn get_config<'a>(context: &Context, config: &'a OnceCell<Option<Ini>>) -> Option<&'a Ini> {
    config
        .get_or_init(|| {
            let path = get_config_file_path(context)?;
            Ini::load_from_file(path).ok()
        })
        .as_ref()
}

// Initialize the AWS credentials file once
fn get_creds<'a>(context: &Context, config: &'a OnceCell<Option<Ini>>) -> Option<&'a Ini> {
    config
        .get_or_init(|| {
            let path = get_credentials_file_path(context)?;
            Ini::load_from_file(path).ok()
        })
        .as_ref()
}

// Get the section for a given profile name in the config file.
fn get_profile_config<'a>(
    config: &'a Ini,
    profile: Option<&Profile>,
) -> Option<&'a ini::Properties> {
    match profile {
        Some(profile) => config.section(Some(format!("profile {profile}"))),
        None => config.section(Some("default")),
    }
}

// Get the section for a given profile name in the credentials file.
fn get_profile_creds<'a>(
    config: &'a Ini,
    profile: Option<&Profile>,
) -> Option<&'a ini::Properties> {
    match profile {
        None => config.section(Some("default")),
        _ => config.section(profile),
    }
}

fn get_aws_region_from_config(
    context: &Context,
    aws_profile: &Option<Profile>,
    aws_config: &AwsConfigFile,
) -> Option<Region> {
    let config = get_config(context, aws_config)?;
    let section = get_profile_config(config, aws_profile.as_ref())?;

    section.get("region").map(std::borrow::ToOwned::to_owned)
}

fn get_aws_profile_and_region(
    context: &Context,
    aws_config: &AwsConfigFile,
) -> (Option<Profile>, Option<Region>) {
    let profile_env_vars = ["AWSU_PROFILE", "AWS_VAULT", "AWSUME_PROFILE", "AWS_PROFILE"];
    let region_env_vars = ["AWS_REGION", "AWS_DEFAULT_REGION"];
    let profile = profile_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var));
    let region = region_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var));
    match (profile, region) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(p), None) => (
            Some(p.clone()),
            get_aws_region_from_config(context, &Some(p), aws_config),
        ),
        (None, None) => (None, get_aws_region_from_config(context, &None, aws_config)),
    }
}

fn get_credentials_duration(
    context: &Context,
    aws_profile: Option<&Profile>,
    aws_creds: &AwsCredsFile,
) -> Option<i64> {
    let expiration_env_vars = ["AWS_CREDENTIAL_EXPIRATION", "AWSUME_EXPIRATION"];
    let expiration_date = if let Some(expiration_date) = expiration_env_vars
        .iter()
        .find_map(|env_var| context.get_env(env_var))
    {
        chrono::DateTime::parse_from_rfc3339(&expiration_date).ok()
    } else {
        let creds = get_creds(context, aws_creds)?;
        let section = get_profile_creds(creds, aws_profile)?;

        let expiration_keys = ["expiration", "x_security_token_expires"];
        expiration_keys
            .iter()
            .find_map(|expiration_key| section.get(expiration_key))
            .and_then(|expiration| DateTime::parse_from_rfc3339(expiration).ok())
    }?;

    Some(expiration_date.timestamp() - chrono::Local::now().timestamp())
}

fn alias_name(name: Option<String>, aliases: &HashMap<String, &str>) -> Option<String> {
    name.as_ref()
        .and_then(|n| aliases.get(n))
        .map(|&a| a.to_string())
        .or(name)
}

fn has_credential_process_or_sso(
    context: &Context,
    aws_profile: Option<&Profile>,
    aws_config: &AwsConfigFile,
    aws_creds: &AwsCredsFile,
) -> Option<bool> {
    let config = get_config(context, aws_config)?;
    let credentials = get_creds(context, aws_creds);

    let empty_section = ini::Properties::new();
    // We use the aws_profile here because `get_profile_config()` treats None
    // as "special" and falls back to the "[default]"; otherwise this tries
    // to look up "[profile default]" which doesn't exist
    let config_section = get_profile_config(config, aws_profile).or(Some(&empty_section))?;

    let credential_section = match credentials {
        Some(credentials) => get_profile_creds(credentials, aws_profile),
        None => None,
    };

    Some(
        config_section.contains_key("credential_process")
            || config_section.contains_key("sso_start_url")
            || credential_section?.contains_key("credential_process")
            || credential_section?.contains_key("sso_start_url"),
    )
}

fn has_defined_credentials(
    context: &Context,
    aws_profile: Option<&Profile>,
    aws_creds: &AwsCredsFile,
) -> Option<bool> {
    let valid_env_vars = [
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_SESSION_TOKEN",
    ];

    // accept if set through environment variable
    if valid_env_vars
        .iter()
        .any(|env_var| context.get_env(env_var).is_some())
    {
        return Some(true);
    }

    let creds = get_creds(context, aws_creds)?;
    let section = get_profile_creds(creds, aws_profile)?;
    Some(section.contains_key("aws_access_key_id"))
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("aws");
    let config: AwsConfig = AwsConfig::try_load(module.config);

    let aws_config = OnceCell::new();
    let aws_creds = OnceCell::new();

    let (aws_profile, aws_region) = get_aws_profile_and_region(context, &aws_config);
    if aws_profile.is_none() && aws_region.is_none() {
        return None;
    }

    // only display if credential_process is defined or has valid credentials
    if !config.force_display
        && !has_credential_process_or_sso(context, aws_profile.as_ref(), &aws_config, &aws_creds)
            .unwrap_or(false)
        && !has_defined_credentials(context, aws_profile.as_ref(), &aws_creds).unwrap_or(false)
    {
        return None;
    }

    let duration = {
        get_credentials_duration(context, aws_profile.as_ref(), &aws_creds).map(|duration| {
            if duration > 0 {
                render_time((duration * 1000) as u128, false)
            } else {
                config.expiration_symbol.to_string()
            }
        })
    };

    let mapped_region = alias_name(aws_region, &config.region_aliases);

    let mapped_profile = alias_name(aws_profile, &config.profile_aliases);

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
                "profile" => mapped_profile.as_ref().map(Ok),
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
    use nu_ansi_term::Color;
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
    fn profile_set_with_alias() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "CORPORATION-CORP_astronauts_ACCESS_GROUP")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [aws.profile_aliases]
                CORPORATION-CORP_astronauts_ACCESS_GROUP = "astro"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_and_profile_both_set_with_alias() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "CORPORATION-CORP_astronauts_ACCESS_GROUP")
            .env("AWS_REGION", "ap-southeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [aws.profile_aliases]
                CORPORATION-CORP_astronauts_ACCESS_GROUP = "astro"
                [aws.region_aliases]
                ap-southeast-2 = "au"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro (au) ")
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
                "AWS_SHARED_CREDENTIALS_FILE",
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
credential_process = /opt/bin/awscreds-retriever

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
credential_process = /opt/bin/awscreds-retriever
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            NaiveDateTime::from_timestamp_opt(chrono::Local::now().timestamp() + 1800, 0).unwrap(),
            Utc,
        );

        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .env(
                "AWS_CREDENTIAL_EXPIRATION",
                now_plus_half_hour.to_rfc3339_opts(SecondsFormat::Secs, true),
            )
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-2) [30m] ")
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
            NaiveDateTime::from_timestamp_opt(chrono::Local::now().timestamp() + 1800, 0).unwrap(),
            Utc,
        );

        let expiration_date = now_plus_half_hour.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let expiration_keys = ["expiration", "x_security_token_expires"];
        expiration_keys.iter().for_each(|key| {
            file.write_all(
                format!(
                    "[astronauts]
aws_access_key_id=dummy
aws_secret_access_key=dummy
{key}={expiration_date}
"
                )
                .as_bytes(),
            )
            .unwrap();

            let actual = ModuleRenderer::new("aws")
                .env("AWS_PROFILE", "astronauts")
                .env("AWS_REGION", "ap-northeast-2")
                .env(
                    "AWS_SHARED_CREDENTIALS_FILE",
                    credentials_path.to_string_lossy().as_ref(),
                )
                .collect();

            let actual_variant = ModuleRenderer::new("aws")
                .env("AWS_PROFILE", "astronauts")
                .env("AWS_REGION", "ap-northeast-2")
                .env(
                    "AWS_CREDENTIALS_FILE",
                    credentials_path.to_string_lossy().as_ref(),
                )
                .collect();

            assert_eq!(
                actual, actual_variant,
                "both AWS_SHARED_CREDENTIALS_FILE and AWS_CREDENTIALS_FILE should work"
            );

            // In principle, "30m" should be correct. However, bad luck in scheduling
            // on shared runners may delay it.
            let possible_values = [
                "30m2s", "30m1s", "30m", "29m59s", "29m58s", "29m57s", "29m56s", "29m55s",
            ];
            let possible_values = possible_values.map(|duration| {
                let segment_colored = format!("☁️  astronauts (ap-northeast-2) [{duration}] ");
                Some(format!(
                    "on {}",
                    Color::Yellow.bold().paint(segment_colored)
                ))
            });

            assert!(
                possible_values.contains(&actual),
                "time is not in range: {actual:?}"
            );
        });

        dir.close()
    }

    #[test]
    fn profile_and_region_set_show_duration() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
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
            NaiveDateTime::from_timestamp_opt(chrono::Local::now().timestamp() - 1800, 0).unwrap(),
            Utc,
        );

        let symbol = "!!!";

        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                expiration_symbol = symbol
            })
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .env(
                "AWS_CREDENTIAL_EXPIRATION",
                now.to_rfc3339_opts(SecondsFormat::Secs, true),
            )
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint(format!("☁️  astronauts (ap-northeast-2) [{symbol}] "))
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

    #[test]
    fn missing_any_credentials() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let credential_path = dir.path().join("credentials");
        File::create(&credential_path)?;

        let config_path = dir.path().join("config");
        let mut config_file = File::create(&config_path)?;

        config_file.write_all(
            "[default]
region = us-east-1
output = json

[profile astronauts]
region = us-east-2
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env(
                "AWS_CREDENTIALS_FILE",
                credential_path.to_string_lossy().as_ref(),
            )
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn missing_any_credentials_but_display_empty() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[profile astronauts]
region = us-east-2
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                force_display = true
            })
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts (us-east-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn access_key_credential_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let credentials_path = dir.path().join("credentials");
        let mut file = File::create(&credentials_path)?;

        file.write_all(
            "[astronauts]
aws_access_key_id=dummy
aws_secret_access_key=dummy
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env(
                "AWS_SHARED_CREDENTIALS_FILE",
                credentials_path.to_string_lossy().as_ref(),
            )
            .collect();

        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn credential_process_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[default]
region = ap-northeast-2
credential_process = /opt/bin/awscreds-retriever
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn credential_process_set_in_credentials() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let credential_path = dir.path().join("credentials");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[default]
region = ap-northeast-2
"
            .as_bytes(),
        )?;

        let mut file = File::create(&credential_path)?;

        file.write_all(
            "[default]
credential_process = /opt/bin/awscreds-for-tests
"
            .as_bytes(),
        )?;
        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env(
                "AWS_CREDENTIALS_FILE",
                credential_path.to_string_lossy().as_ref(),
            )
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn sso_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut file = File::create(&config_path)?;

        file.write_all(
            "[default]
region = ap-northeast-2
sso_start_url = https://starship.rs/sso
sso_region = <SSO-Default-Region>
sso_account_id = <AWS ACCOUNT ID>
sso_role_name = <AWS-ROLE-NAME>
"
            .as_bytes(),
        )?;

        file.sync_all()?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn access_key_env_var_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn secret_access_key_env_var_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_SECRET_ACCESS_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn session_token_env_var_set() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_SESSION_TOKEN", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }
}
