use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use super::{Context, Module, RootModuleConfig};

use crate::configs::aws::AwsConfig;
use crate::formatter::StringFormatter;

type Profile = String;
type Region = String;

fn get_aws_region_from_config(context: &Context, aws_profile: Option<&str>) -> Option<Region> {
    let config_location = context
        .get_env("AWS_CONFIG_FILE")
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

fn get_aws_profile_and_region(context: &Context) -> (Option<Profile>, Option<Region>) {
    match (
        context
            .get_env("AWS_VAULT")
            .or_else(|| context.get_env("AWS_PROFILE")),
        context
            .get_env("AWS_DEFAULT_REGION")
            .or_else(|| context.get_env("AWS_REGION")),
    ) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(ref p), None) => (
            Some(p.to_owned()),
            get_aws_region_from_config(context, Some(p)),
        ),
        (None, None) => (None, get_aws_region_from_config(context, None)),
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

    let (aws_profile, aws_region) = get_aws_profile_and_region(context);
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
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn no_region_set() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws").collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn region_set() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-2")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  (ap-northeast-2)")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn region_set_with_alias() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-southeast-2")
            .config(toml::toml! {
                [aws.region_aliases]
                ap-southeast-2 = "au"
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Yellow.bold().paint("☁️  (au)")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn default_region_set() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-2")
            .env("AWS_DEFAULT_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  (ap-northeast-1)")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn profile_set() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn profile_set_from_aws_vault() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_VAULT", "astronauts-vault")
            .env("AWS_PROFILE", "astronauts-profile")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts-vault")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn profile_and_region_set() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-2")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts(ap-northeast-2)")
        ));

        assert_eq!(expected, actual);
        Ok(())
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
            "on {} ",
            Color::Yellow.bold().paint("☁️  (us-east-1)")
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
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts(us-east-2)")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn profile_and_region_set_with_display_all() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .env("AWS_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts(ap-northeast-1)")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn profile_set_with_display_all() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_PROFILE", "astronauts")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  astronauts")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn region_set_with_display_all() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-1")
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  (ap-northeast-1)")
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn profile_and_region_set_with_display_region() -> io::Result<()> {
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
        Ok(())
    }

    #[test]
    fn profile_and_region_set_with_display_profile() -> io::Result<()> {
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
        Ok(())
    }

    #[test]
    fn region_set_with_display_profile() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .env("AWS_REGION", "ap-northeast-1")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$profile]($style) "
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Yellow.bold().paint("☁️  ")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn region_not_set_with_display_region() -> io::Result<()> {
        let actual = ModuleRenderer::new("aws")
            .config(toml::toml! {
                [aws]
                format = "on [$symbol$region]($style) "
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }
}
