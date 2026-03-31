use std::borrow::Cow;
use std::cell::OnceCell;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;
use ini::Ini;
use serde_json as json;
use sha1::{Digest, Sha1};

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
    let profile_env_vars = [
        "AWSU_PROFILE",
        "AWS_VAULT",
        "AWSUME_PROFILE",
        "AWS_PROFILE",
        "AWS_SSO_PROFILE",
    ];
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

// Get the SSO cache key input from a profile section, handling both direct sso_start_url
// and sso_session (which references a separate [sso-session <name>] section).
fn get_sso_cache_key_input(profile_section: &ini::Properties) -> Option<String> {
    profile_section
        .get("sso_session")
        .map(|s| s.to_string())
        .or_else(|| profile_section.get("sso_start_url").map(|s| s.to_string()))
}

fn get_credentials_duration(
    context: &Context,
    aws_profile: Option<&Profile>,
    aws_config: &AwsConfigFile,
    aws_creds: &AwsCredsFile,
) -> Option<i64> {
    let expiration_env_vars = [
        "AWS_CREDENTIAL_EXPIRATION",
        "AWS_SESSION_EXPIRATION",
        "AWSUME_EXPIRATION",
    ];
    let expiration_date = if let Some(expiration_date) = expiration_env_vars
        .into_iter()
        .find_map(|env_var| context.get_env(env_var))
    {
        // get expiration from environment variables
        chrono::DateTime::parse_from_rfc3339(&expiration_date).ok()
    } else if let Some(section) =
        get_creds(context, aws_creds).and_then(|creds| get_profile_creds(creds, aws_profile))
    {
        // get expiration from credentials file
        let expiration_keys = ["expiration", "x_security_token_expires"];
        expiration_keys
            .iter()
            .find_map(|expiration_key| section.get(expiration_key))
            .and_then(|expiration| DateTime::parse_from_rfc3339(expiration).ok())
    } else {
        // get expiration from cached SSO credentials
        let config = get_config(context, aws_config)?;
        let section = get_profile_config(config, aws_profile)?;
        let cache_key_input = get_sso_cache_key_input(section)?;
        // https://github.com/boto/botocore/blob/d7ff05fac5bf597246f9e9e3fac8f22d35b02e64/botocore/utils.py#L3350
        let cache_key = crate::utils::encode_to_hex(&Sha1::digest(cache_key_input.as_bytes()));
        // https://github.com/aws/aws-cli/blob/b3421dcdd443db95999364e94266c0337b45cc43/awscli/customizations/sso/utils.py#L89
        let mut sso_cred_path = context.get_home()?;
        sso_cred_path.push(format!(".aws/sso/cache/{cache_key}.json"));
        let sso_cred_json: json::Value =
            json::from_str(&crate::utils::read_file(&sso_cred_path).ok()?).ok()?;
        let expires_at = sso_cred_json.get("expiresAt")?.as_str();
        DateTime::parse_from_rfc3339(expires_at?).ok()
    }?;

    Some(expiration_date.timestamp() - chrono::Local::now().timestamp())
}

fn get_aliased_name<'a>(
    pattern: Option<&'a str>,
    current_value: Option<&str>,
    alias: Option<&'a str>,
) -> Option<String> {
    let replacement = alias.or(current_value)?.to_string();
    let Some(pattern) = pattern else {
        return Some(replacement);
    };
    let value = current_value?;
    if value == pattern {
        return Some(replacement);
    }
    let re = match regex::Regex::new(&format!("^{pattern}$")) {
        Ok(re) => re,
        Err(error) => {
            log::warn!(
                "Could not compile regular expression `{}`:\n{}",
                &format!("^{pattern}$"),
                error
            );
            return None;
        }
    };
    let replaced = re.replace(value, replacement.as_str());
    match replaced {
        Cow::Owned(replaced) => Some(replaced),
        Cow::Borrowed(_) => None,
    }
}

mod deprecated {
    use std::borrow::Cow;
    use std::collections::HashMap;

    pub fn get_alias<'a>(
        current_value: String,
        aliases: &'a HashMap<String, &'a str>,
        name: &'a str,
    ) -> Option<String> {
        let alias = if let Some(val) = aliases.get(current_value.as_str()) {
            Some((*val).to_string())
        } else {
            aliases.iter().find_map(|(k, v)| {
                let re = regex::Regex::new(&format!("^{k}$")).ok()?;
                let replaced = re.replace(current_value.as_str(), *v);
                match replaced {
                    Cow::Owned(replaced) => Some(replaced),
                    Cow::Borrowed(_) => None,
                }
            })
        };

        match alias {
            Some(alias) => {
                log::warn!(
                    "Usage of '{}_aliases' is deprecated and will be removed in 2.0; Use 'profiles' with '{}_alias' instead. (`{}` -> `{}`)",
                    &name,
                    &name,
                    &current_value,
                    &alias
                );
                Some(alias)
            }
            None => Some(current_value),
        }
    }
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
            || config_section.contains_key("sso_session")
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

// https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-settings
fn has_source_profile(
    context: &Context,
    aws_profile: Option<&Profile>,
    aws_config: &AwsConfigFile,
    aws_creds: &AwsCredsFile,
) -> Option<bool> {
    let config = get_config(context, aws_config)?;

    let config_section = get_profile_config(config, aws_profile)?;
    let source_profile = config_section
        .get("source_profile")
        .map(std::borrow::ToOwned::to_owned);

    let has_credential_process =
        has_credential_process_or_sso(context, source_profile.as_ref(), aws_config, aws_creds)
            .unwrap_or(false);
    let has_credentials =
        has_defined_credentials(context, source_profile.as_ref(), aws_creds).unwrap_or(false);

    Some(has_credential_process || has_credentials)
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

    // only display in the presence of credential_process, source_profile or valid credentials
    if !config.force_display
        && !has_credential_process_or_sso(context, aws_profile.as_ref(), &aws_config, &aws_creds)
            .unwrap_or(false)
        && !has_source_profile(context, aws_profile.as_ref(), &aws_config, &aws_creds)
            .unwrap_or(false)
        && !has_defined_credentials(context, aws_profile.as_ref(), &aws_creds).unwrap_or(false)
    {
        return None;
    }

    let duration = {
        get_credentials_duration(context, aws_profile.as_ref(), &aws_config, &aws_creds).map(
            |duration| {
                if duration > 0 {
                    render_time((duration * 1000) as u128, false)
                } else {
                    config.expiration_symbol.to_string()
                }
            },
        )
    };

    let (matched_profile_config, display_profile, display_region) = config
        .profiles
        .iter()
        .find_map(|profile_config| {
            let profile_alias = get_aliased_name(
                Some(profile_config.profile_pattern),
                aws_profile.as_deref(),
                profile_config.profile_alias,
            )?;

            let region_alias = get_aliased_name(
                profile_config.region_pattern,
                aws_region.as_deref(),
                profile_config.region_alias,
            );
            if matches!(
                (profile_config.region_pattern, &region_alias),
                (Some(_), None)
            ) {
                return None;
            }

            Some((Some(profile_config), Some(profile_alias), region_alias))
        })
        .unwrap_or_else(|| (None, aws_profile, aws_region));

    let display_profile = display_profile
        .map(|p| deprecated::get_alias(p, &config.profile_aliases, "profile"))
        .unwrap_or(None);
    let display_region = display_region
        .map(|r| deprecated::get_alias(r, &config.region_aliases, "region"))
        .unwrap_or(None);

    let display_style = matched_profile_config
        .and_then(|cfg| cfg.style)
        .unwrap_or(config.style);
    let display_symbol = matched_profile_config
        .and_then(|cfg| cfg.symbol)
        .unwrap_or(config.symbol);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(display_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "profile" => display_profile.as_ref().map(Ok),
                "region" => display_region.as_ref().map(Ok),
                "duration" => duration.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `aws`: \n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{File, create_dir};
    use std::io::{self, Write};

    #[test]
    fn profiles_simple_pattern_match() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "astro"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_regex_pattern_match() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "company-prod-astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "company-.*-astronauts"
                profile_alias = "astro"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_capture_group_replacement() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "company-prod-astronauts")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "company-(?P<env>[\\w-]+)-astronauts"
                profile_alias = "astro-$env"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro-prod ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_region_pattern_match() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-southeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "astro"
                region_pattern = "ap-southeast-.*"
                region_alias = "au"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro (au) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_region_pattern_no_match_skips_entry() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "us-east-1")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "astro-au"
                region_pattern = "ap-southeast-.*"
                region_alias = "au"

                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "astro-us"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astro-us (us-east-1) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_custom_style_and_symbol() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "production")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "production"
                style = "bold red"
                symbol = "🔴 "
            })
            .collect();
        let expected = Some(format!("on {}", Color::Red.bold().paint("🔴 production ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_no_match_falls_through() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "production"
                profile_alias = "prod"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_first_match_wins() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "first"

                [[aws.profiles]]
                profile_pattern = "astronauts"
                profile_alias = "second"
            })
            .collect();
        let expected = Some(format!("on {}", Color::Yellow.bold().paint("☁️  first ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn profiles_region_capture_group() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-southeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [[aws.profiles]]
                profile_pattern = "astronauts"
                region_pattern = "(?P<area>[a-z]+)-(?P<dir>[a-z]+)-(?P<num>\\d+)"
                region_alias = "$area-$num"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts (ap-2) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn no_region_set() {
        let actual = ModuleRenderer::new("aws").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn region_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        let actual = module_renderer
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn region_set_with_alias() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        let actual = module_renderer
            .env("AWS_REGION", "ap-southeast-2")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .config(toml::toml! {
                [aws.region_aliases]
                ap-southeast-2 = "au"
            })
            .collect();
        let expected = Some(format!("on {}", Color::Yellow.bold().paint("☁️  (au) ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn default_region_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        let actual = module_renderer
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_DEFAULT_REGION", "ap-northeast-1")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
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
    fn profile_set_from_awsssocli() {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_SSO_PROFILE", "astronauts-awsssocli")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts-awsssocli ")
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
        let creds_path = dir.path().join("credentials");
        create_dir(&creds_path)?;

        let config_path = dir.path().join("config");
        File::create(&config_path)?.sync_all()?;

        assert!(
            ModuleRenderer::new("aws")
                .env(
                    "AWS_SHARED_CREDENTIALS_FILE",
                    creds_path.to_string_lossy().as_ref(),
                )
                .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
                .collect()
                .is_none()
        );

        dir.close()
    }

    #[test]
    fn config_file_path_is_ignored_when_is_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        create_dir(&config_path)?;

        let creds_path = dir.path().join("credentials");
        File::create(&creds_path)?.sync_all()?;

        assert!(
            ModuleRenderer::new("aws")
                .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
                .env(
                    "AWS_SHARED_CREDENTIALS_FILE",
                    creds_path.to_string_lossy().as_ref(),
                )
                .collect()
                .is_none()
        );

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
    fn region_set_with_display_all() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        let actual = module_renderer
            .env("AWS_REGION", "ap-northeast-1")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-1) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
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
        use chrono::{DateTime, SecondsFormat, Utc};

        let expiration_env_vars = ["AWS_SESSION_EXPIRATION", "AWS_CREDENTIAL_EXPIRATION"];
        for env_var in expiration_env_vars {
            let now_plus_half_hour: DateTime<Utc> =
                DateTime::from_timestamp(chrono::Local::now().timestamp() + 1800, 0).unwrap();

            let actual = ModuleRenderer::new("aws")
                .env("AWS_PROFILE", "astronauts")
                .env("AWS_REGION", "ap-northeast-2")
                .env("AWS_ACCESS_KEY_ID", "dummy")
                .env(
                    env_var,
                    now_plus_half_hour.to_rfc3339_opts(SecondsFormat::Secs, true),
                )
                .collect();

            let possible_values = [
                "30m2s", "30m1s", "30m0s", "29m59s", "29m58s", "29m57s", "29m56s", "29m55s",
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
        }
    }

    #[test]
    fn expiration_date_set_from_file() -> io::Result<()> {
        use chrono::{DateTime, Utc};

        let dir = tempfile::tempdir()?;
        let credentials_path = dir.path().join("credentials");
        let mut file = File::create(&credentials_path)?;

        let now_plus_half_hour: DateTime<Utc> =
            DateTime::from_timestamp(chrono::Local::now().timestamp() + 1800, 0).unwrap();

        let expiration_date = now_plus_half_hour.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let expiration_keys = ["expiration", "x_security_token_expires"];
        for key in expiration_keys {
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

            let credentials_env_vars = ["AWS_SHARED_CREDENTIALS_FILE", "AWS_CREDENTIALS_FILE"];
            for env_var in credentials_env_vars {
                let actual = ModuleRenderer::new("aws")
                    .env("AWS_PROFILE", "astronauts")
                    .env("AWS_REGION", "ap-northeast-2")
                    .env(env_var, credentials_path.to_string_lossy().as_ref())
                    .collect();

                // In principle, "30m" should be correct. However, bad luck in scheduling
                // on shared runners may delay it.
                let possible_values = [
                    "30m2s", "30m1s", "30m0s", "29m59s", "29m58s", "29m57s", "29m56s", "29m55s",
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
            }
        }

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
        use chrono::{DateTime, SecondsFormat, Utc};

        let now: DateTime<Utc> =
            DateTime::from_timestamp(chrono::Local::now().timestamp() - 1800, 0).unwrap();

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
                "AWS_SESSION_EXPIRATION",
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
    fn sso_legacy_set() -> io::Result<()> {
        use chrono::{DateTime, SecondsFormat, Utc};

        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        std::fs::create_dir_all(dir.path().join(".aws/sso/cache"))?;

        let mut file = File::create(dir.path().join(".aws/config"))?;
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

        let mut file = File::create(
            dir.path()
                // SHA-1 of "https://starship.rs/sso"
                .join(".aws/sso/cache/a47a4e57aecc96b31b4f083543924bd6f828e65a.json"),
        )?;

        let one_second_ago: DateTime<Utc> =
            DateTime::from_timestamp(chrono::Local::now().timestamp() - 1, 0).unwrap();

        file.write_all(
            format!(
                r#"{{"expiresAt": "{}"}}"#,
                one_second_ago.to_rfc3339_opts(SecondsFormat::Secs, true)
            )
            .as_bytes(),
        )?;
        file.sync_all()?;

        let actual = module_renderer.collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2) [X] ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn sso_set() -> io::Result<()> {
        use chrono::{DateTime, SecondsFormat, Utc};

        let (module_renderer, dir) = ModuleRenderer::new_with_home("aws")?;
        std::fs::create_dir_all(dir.path().join(".aws/sso/cache"))?;

        let config_path = dir.path().join(".aws/config");
        let mut config = File::create(&config_path)?;
        config.write_all(
            "[profile astronauts]
sso_session = default
sso_account_id = 123456789011
sso_role_name = readOnly
region = us-west-2
output = json

[sso-session default]
sso_region = us-east-1
sso_start_url = https://starship.rs/sso
sso_registration_scopes = sso:account:access
"
            .as_bytes(),
        )?;
        config.sync_all()?;

        // SHA-1 of "default" = 7505d64a54e061b7acd54ccd58b49dc43500b635
        let mut cache_file = File::create(
            dir.path()
                .join(".aws/sso/cache/7505d64a54e061b7acd54ccd58b49dc43500b635.json"),
        )?;

        let one_second_ago: DateTime<Utc> =
            DateTime::from_timestamp(chrono::Local::now().timestamp() - 1, 0).unwrap();

        cache_file.write_all(
            format!(
                r#"{{"expiresAt": "{}"}}"#,
                one_second_ago.to_rfc3339_opts(SecondsFormat::Secs, true)
            )
            .as_bytes(),
        )?;
        cache_file.sync_all()?;

        let actual = module_renderer
            .env("AWS_PROFILE", "astronauts")
            .config(toml::toml! {
                [aws]
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow
                .bold()
                .paint("☁️  astronauts (us-west-2) [X] ")
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

    #[test]
    fn source_profile_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let credential_path = dir.path().join("credentials");
        let mut config = File::create(&config_path)?;
        config.write_all(
            "[profile astronauts]
source_profile = starship
"
            .as_bytes(),
        )?;
        let mut credentials = File::create(&credential_path)?;
        credentials.write_all(
            "[starship]
aws_access_key_id=dummy
aws_secret_access_key=dummy
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env(
                "AWS_CREDENTIALS_FILE",
                credential_path.to_string_lossy().as_ref(),
            )
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn source_profile_not_exists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut config = File::create(&config_path)?;
        config.write_all(
            "[profile astronauts]
source_profile = starship
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn source_profile_uses_credential_process() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("config");
        let mut config = File::create(&config_path)?;
        config.write_all(
            "[profile starship]
credential_process = /opt/bin/awscreds-retriever --username starship

[profile astronauts]
source_profile = starship
"
            .as_bytes(),
        )?;

        let actual = ModuleRenderer::new("aws")
            .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("☁️  astronauts ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
}
