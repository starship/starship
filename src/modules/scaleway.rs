use std::collections::HashMap;
use std::path::PathBuf;

use yaml_rust2::{Yaml, YamlLoader};

use crate::config::ModuleConfig;
use crate::configs::scaleway::ScalewayConfig;
use crate::context::Context;
use crate::formatter::StringFormatter;
use crate::module::Module;

/// Values resolved from the Scaleway config file.
struct ScwConfigFile {
    /// Root settings overlaid with the active profile's settings.
    values: HashMap<String, String>,
    /// The resolved active profile name, if any.
    profile: Option<String>,
}

/// Locate the Scaleway config file, following the scaleway SDK precedence:
/// `$SCW_CONFIG_PATH` > `$XDG_CONFIG_HOME/scw/config.{yaml,yml}` > `$HOME/.config/scw/config.{yaml,yml}`.
fn config_path(context: &Context) -> Option<PathBuf> {
    if let Some(path) = context.get_env("SCW_CONFIG_PATH") {
        return Some(PathBuf::from(path));
    }
    let config_home = context
        .get_env("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| context.get_home().map(|home| home.join(".config")))?;
    ["config.yaml", "config.yml"]
        .iter()
        .map(|file| config_home.join("scw").join(file))
        .find(|path| path.exists())
}

/// Collect the string values of a YAML mapping into a `HashMap`.
fn yaml_strings(yaml: &Yaml) -> HashMap<String, String> {
    let mut values = HashMap::new();
    if let Some(hash) = yaml.as_hash() {
        for (key, value) in hash {
            if let (Some(key), Some(value)) = (key.as_str(), value.as_str()) {
                values.insert(key.to_string(), value.to_string());
            }
        }
    }
    values
}

/// Load the config file, overlaying the active profile's settings on top of the
/// root settings. The active profile is `$SCW_PROFILE` if set, otherwise the
/// `active_profile` (or legacy `profile`) key of the config file.
fn read_config_file(context: &Context, profile_env: Option<&str>) -> Option<ScwConfigFile> {
    let path = config_path(context)?;
    let content = crate::utils::read_file(&path).ok()?;
    let root = YamlLoader::load_from_str(&content)
        .ok()?
        .into_iter()
        .next()?;

    let mut values = yaml_strings(&root);

    let profile = profile_env.map(ToOwned::to_owned).or_else(|| {
        root.as_hash().and_then(|hash| {
            hash.get(&Yaml::String("active_profile".into()))
                .or_else(|| hash.get(&Yaml::String("profile".into())))
                .and_then(Yaml::as_str)
                .map(ToOwned::to_owned)
        })
    });

    if let Some(name) = &profile {
        let active_profile = root
            .as_hash()
            .and_then(|hash| hash.get(&Yaml::String("profiles".into())))
            .and_then(Yaml::as_hash)
            .and_then(|profiles| profiles.get(&Yaml::String(name.clone())));
        if let Some(active_profile) = active_profile {
            values.extend(yaml_strings(active_profile));
        }
    }

    Some(ScwConfigFile { values, profile })
}

/// Creates a module for displaying the current Scaleway configuration.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("scaleway");
    let config: ScalewayConfig = ScalewayConfig::try_load(module.config);

    const SCW_ACCESS_KEY: &str = "SCW_ACCESS_KEY";
    const SCW_SECRET_KEY: &str = "SCW_SECRET_KEY";
    const SCW_DEFAULT_ORGANIZATION_ID: &str = "SCW_DEFAULT_ORGANIZATION_ID";
    const SCW_DEFAULT_PROJECT_ID: &str = "SCW_DEFAULT_PROJECT_ID";
    const SCW_DEFAULT_REGION: &str = "SCW_DEFAULT_REGION";
    const SCW_DEFAULT_ZONE: &str = "SCW_DEFAULT_ZONE";
    const SCW_API_URL: &str = "SCW_API_URL";
    const SCW_PROFILE: &str = "SCW_PROFILE";

    let profile_env = context
        .get_env(SCW_PROFILE)
        .filter(|profile| !profile.is_empty());
    let file = read_config_file(context, profile_env.as_deref()).unwrap_or_else(|| ScwConfigFile {
        values: HashMap::new(),
        profile: profile_env,
    });
    let values = &file.values;

    // Environment variables override the config file.
    let access_key = context
        .get_env(SCW_ACCESS_KEY)
        .or_else(|| values.get("access_key").cloned());
    let secret_key = context
        .get_env(SCW_SECRET_KEY)
        .or_else(|| values.get("secret_key").cloned());
    let organization = context
        .get_env(SCW_DEFAULT_ORGANIZATION_ID)
        .or_else(|| values.get("default_organization_id").cloned());
    let project = context
        .get_env(SCW_DEFAULT_PROJECT_ID)
        .or_else(|| values.get("default_project_id").cloned());
    let region = context
        .get_env(SCW_DEFAULT_REGION)
        .or_else(|| values.get("default_region").cloned());
    let zone = context
        .get_env(SCW_DEFAULT_ZONE)
        .or_else(|| values.get("default_zone").cloned());
    let profile = file.profile;
    let api_url = context.get_env(SCW_API_URL).or_else(|| {
        // The generated config writes `apiurl` at the root, profiles use `api_url`.
        values
            .get("api_url")
            .or_else(|| values.get("apiurl"))
            .cloned()
    });

    let has_credentials = access_key.is_some() && secret_key.is_some();
    let has_config = organization.is_some()
        || project.is_some()
        || region.is_some()
        || zone.is_some()
        || profile.is_some()
        || api_url.is_some();

    if !config.force_display && !has_credentials && !has_config {
        return None;
    }

    // Apply aliases so raw IDs and URLs can be shown as friendly names.
    let aliased = |value: String, aliases: &HashMap<String, &str>| {
        aliases
            .get(&value)
            .map(ToString::to_string)
            .unwrap_or(value)
    };
    let organization = organization.map(|v| aliased(v, &config.organization_aliases));
    let project = project.map(|v| aliased(v, &config.project_aliases));
    let region = region.map(|v| aliased(v, &config.region_aliases));
    let api_url = api_url.map(|v| aliased(v, &config.api_url_aliases));

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
                "organization" => organization.as_ref().map(Ok),
                "project" => project.as_ref().map(Ok),
                "zone" => zone.as_ref().map(Ok),
                "region" => region.as_ref().map(Ok),
                "profile" => profile.as_ref().map(Ok),
                "api_url" => api_url.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `scaleway`: \n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs;
    use std::io;
    use std::io::Write;

    fn project_id() -> &'static str {
        "11111111-1111-1111-1111-111111111111"
    }

    #[test]
    fn no_configuration() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        assert_eq!(None, module_renderer.collect());
        dir.close()
    }

    #[test]
    fn profile_env() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer.env("SCW_PROFILE", "production").collect();
        let expected = Some(format!("on {}", Color::Purple.paint("☁️ production ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn project_env() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_DEFAULT_PROJECT_ID", project_id())
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Purple.paint(format!("☁️ {} ", project_id()))
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn zone_env() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_DEFAULT_ZONE", "fr-par-1")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!("on {}", Color::Purple.paint("☁️ (fr-par-1) ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn all_variables_with_aliases() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_PROFILE", "production")
            .env("SCW_DEFAULT_ORGANIZATION_ID", "11111111-1111-1111-1111-111111111111")
            .env("SCW_DEFAULT_PROJECT_ID", "22222222-2222-2222-2222-222222222222")
            .env("SCW_DEFAULT_ZONE", "fr-par-1")
            .env("SCW_DEFAULT_REGION", "fr-par")
            .env("SCW_API_URL", "https://api.scaleway.com")
            .config(toml::toml! {
                [scaleway]
                format = "on [$symbol$profile/$organization/$project/$zone/$region/$api_url]($style)"
                [scaleway.organization_aliases]
                "11111111-1111-1111-1111-111111111111" = "acme"
                [scaleway.project_aliases]
                "22222222-2222-2222-2222-222222222222" = "prod"
                [scaleway.region_aliases]
                "fr-par" = "paris"
                [scaleway.api_url_aliases]
                "https://api.scaleway.com" = "prod-api"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Purple.paint("☁️ production/acme/prod/fr-par-1/paris/prod-api")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn config_file_active_profile_overrides_root() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let config_path = dir.path().join("config.yaml");
        let mut file = fs::File::create(&config_path)?;
        file.write_all(
            b"default_region: fr-par\ndefault_zone: fr-par-1\nprofile: hashicorp\nprofiles:\n  hashicorp:\n    default_project_id: 00000000-0000-0000-0000-000000000000\n    default_zone: nl-ams-2\n    default_region: nl-ams\n",
        )?;
        file.sync_all()?;
        let actual = module_renderer
            .env("SCW_CONFIG_PATH", config_path.to_string_lossy().to_string())
            .config(toml::toml! {
                [scaleway]
                format = "on [$symbol($profile )($project )(\\($zone\\) )]($style)"
            })
            .collect();
        let expected = Some(format!(
            "on {}",
            Color::Purple.paint("☁️ hashicorp 00000000-0000-0000-0000-000000000000 (nl-ams-2) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn config_file_keeps_root_values_without_profile() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let config_path = dir.path().join("config.yaml");
        let mut file = fs::File::create(&config_path)?;
        file.write_all(b"default_region: fr-par\ndefault_zone: fr-par-1\n")?;
        file.sync_all()?;
        let actual = module_renderer
            .env("SCW_CONFIG_PATH", config_path.to_string_lossy().to_string())
            .config(toml::toml! {
                [scaleway]
                format = "on [$symbol($profile )($project )(\\($zone\\) )]($style)"
            })
            .collect();
        let expected = Some(format!("on {}", Color::Purple.paint("☁️ (fr-par-1) ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn force_display_no_configuration() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .config(toml::toml! {
                [scaleway]
                force_display = true
            })
            .collect();
        let expected = Some(format!("on {}", Color::Purple.paint("☁️ ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
