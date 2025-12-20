use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::ModuleConfig;
use crate::context::Context;
use crate::formatter::StringFormatter;
use crate::module::Module;
use serde::{Deserialize, Serialize};
use serde_yaml;
use shellexpand;

/// The `scaleway` module shows the current Scaleway profile, project and region.
///
/// The module will display information if any of the following are set:
/// - `SCW_ACCESS_KEY` and `SCW_SECRET_KEY` environment variables
/// - `SCW_DEFAULT_PROJECT_ID` environment variable
/// - `SCW_DEFAULT_REGION` or `SCW_DEFAULT_ZONE` environment variables
/// - `SCW_PROFILE` environment variable
/// - `SCW_API_URL` environment variable
///
/// # Environment Variables
/// - `SCW_ACCESS_KEY`: The access key for Scaleway API
/// - `SCW_SECRET_KEY`: The secret key for Scaleway API
/// - `SCW_DEFAULT_PROJECT_ID`: The default project ID
/// - `SCW_DEFAULT_REGION`: The default region
/// - `SCW_DEFAULT_ZONE`: The default availability zone
/// - `SCW_PROFILE`: The config profile to use
/// - `SCW_API_URL`: The URL of the Scaleway API
///
/// # Configuration
/// The module can be configured in the starship configuration file with the following options:
/// - `format`: The format string for the module output
/// - `style`: The style for the module text
/// - `symbol`: The symbol to display before the scaleway information
/// - `project_aliases`: A table of project ID aliases
/// - `region_aliases`: A table of region aliases
/// - `api_url_aliases`: A table of API URL aliases
/// - `force_display`: Whether to display the module even if no credentials are found
///
/// # Example
/// ```toml
/// [scaleway]
/// format = "via [$symbol$project($region)($api_url)]($style) "
/// style = "violet"
/// symbol = "☁️ "
///
/// [scaleway.project_aliases]
/// "11111111-1111-1111-1111-111111111111" = "my-project"
///
/// [scaleway.region_aliases]
/// "fr-par" = "paris"
///
/// [scaleway.api_url_aliases]
/// "https://api.scaleway.com" = "prod"
/// "https://api.scaleway.test" = "test"
/// ```
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ScalewayConfig<'a> {
    /// The module format string
    pub format: &'a str,
    /// The style for the module
    pub style: &'a str,
    /// Symbol to display before the scaleway information
    pub symbol: &'a str,
    /// Aliases for project IDs
    pub project_aliases: HashMap<String, &'a str>,
    /// Aliases for regions
    pub region_aliases: HashMap<String, &'a str>,
    /// Aliases for API URLs
    pub api_url_aliases: HashMap<String, &'a str>,
    /// Whether to display the module even if no credentials are found
    pub force_display: bool,
}

impl Default for ScalewayConfig<'_> {
    fn default() -> Self {
        Self {
            format: "$symbol$project($region) ",
            style: "violet",
            symbol: "☁️ ",
            project_aliases: HashMap::new(),
            region_aliases: HashMap::new(),
            api_url_aliases: HashMap::new(),
            force_display: false,
        }
    }
}

/// Reads the Scaleway configuration file and returns values as a HashMap
fn read_config_file() -> HashMap<String, String> {
    let mut values = HashMap::new();

    // Check for config file in standard locations
    let config_paths = [
        // Linux/Unix standard
        "~/.config/scw/config.yaml",
        "~/.config/scw/config.yml",
        // Alternative location
        "~/.scwrc",
        // Current directory
        "./config.yaml",
        "./config.yml",
    ];

    for path_str in &config_paths {
        let path = shellexpand::full(path_str)
            .ok()
            .map(|p| Path::new(&*p).to_path_buf());
        if let Some(config_path) = path {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    // Parse top-level config
                    let de = serde_yaml::Deserializer::from_str(&content);
                    if let Ok(mut config_map) =
                        HashMap::<String, serde_yaml::Value>::deserialize(de)
                    {
                        // Extract top-level values first
                        let mut top_level_values = HashMap::new();
                        for (key, value) in &config_map {
                            if let Some(string_value) = value.as_str() {
                                top_level_values.insert(key.clone(), string_value.to_string());
                            }
                        }

                        // Insert top-level values
                        values.extend(top_level_values);

                        // Try to parse profiles if present
                        if let Some(profile_name) =
                            std::env::var("SCW_PROFILE").ok().or_else(|| {
                                config_map
                                    .remove("profile")
                                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                            })
                        {
                            if let Some(profiles) = config_map.remove("profiles") {
                                if let Some(profile) = profiles.as_mapping() {
                                    if let Some(profile_data) = profile.get(
                                        &serde_yaml::Value::String(profile_name.clone().into()),
                                    ) {
                                        if let Some(profile_map) = profile_data.as_mapping() {
                                            for (key, value) in profile_map {
                                                if let Some(key_str) = key.as_str() {
                                                    if let Some(value_str) = value.as_str() {
                                                        values.insert(
                                                            key_str.to_string(),
                                                            value_str.to_string(),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break; // Successfully read a config file, stop searching
            }
        }
    }

    values
}

/// Creates a module for displaying Scaleway configuration information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("scaleway");
    let config: ScalewayConfig = ScalewayConfig::try_load(module.config);

    // Environment variables for Scaleway configuration
    const SCW_ACCESS_KEY: &str = "SCW_ACCESS_KEY";
    const SCW_SECRET_KEY: &str = "SCW_SECRET_KEY";
    const SCW_DEFAULT_PROJECT_ID: &str = "SCW_DEFAULT_PROJECT_ID";
    const SCW_DEFAULT_REGION: &str = "SCW_DEFAULT_REGION";
    const SCW_DEFAULT_ZONE: &str = "SCW_DEFAULT_ZONE";
    const SCW_PROFILE: &str = "SCW_PROFILE";
    const SCW_API_URL: &str = "SCW_API_URL";

    // Try to read from YAML configuration file
    let mut config_file_values = read_config_file();

    // Get configuration values from environment variables, falling back to config file
    let scw_access_key = context
        .get_env(SCW_ACCESS_KEY)
        .or_else(|| config_file_values.remove("access_key"));
    let scw_secret_key = context
        .get_env(SCW_SECRET_KEY)
        .or_else(|| config_file_values.remove("secret_key"));
    let scw_project_id = context
        .get_env(SCW_DEFAULT_PROJECT_ID)
        .or_else(|| config_file_values.remove("default_project_id"));
    let scw_region = context.get_env(SCW_DEFAULT_REGION).or_else(|| {
        context.get_env(SCW_DEFAULT_ZONE).or_else(|| {
            config_file_values
                .remove("default_region")
                .or_else(|| config_file_values.remove("default_zone"))
        })
    });
    let scw_profile = context
        .get_env(SCW_PROFILE)
        .or_else(|| config_file_values.remove("profile"));
    let scw_api_url = context
        .get_env(SCW_API_URL)
        .or_else(|| config_file_values.remove("api_url"));

    // Check if we have any Scaleway configuration
    let has_credentials = scw_access_key.is_some() && scw_secret_key.is_some();
    let has_project = scw_project_id.is_some();
    let has_region = scw_region.is_some();
    let has_api_url = scw_api_url.is_some();

    // Don't display the module if there's no configuration and we're not forcing display
    if !config.force_display && !has_credentials && !has_project && !has_region && !has_api_url {
        return None;
    }

    // Helper function to map aliases
    fn alias_name(name: Option<&String>, aliases: &HashMap<String, &str>) -> Option<String> {
        name.and_then(|n| {
            aliases
                .get(n)
                .map(|&alias| alias.to_string())
                .or_else(|| Some(n.to_string()))
        })
    }

    // Apply aliases to project, region and API URL
    let mapped_project = alias_name(scw_project_id.as_ref(), &config.project_aliases);
    let region_to_display = scw_region
        .as_ref()
        .as_ref()
        .and_then(|r| alias_name(Some(r), &config.region_aliases));
    let api_url_to_display = scw_api_url
        .as_ref()
        .and_then(|url| alias_name(Some(url), &config.api_url_aliases));

    // Format the module output
    let parsed = StringFormatter::new(&config.format).and_then(|formatter| {
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
                "project" => mapped_project.as_ref().map(Ok),
                "region" => region_to_display.as_ref().map(Ok),
                "profile" => scw_profile.as_ref().map(Ok),
                "api_url" => api_url_to_display.as_ref().map(Ok),
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
    use std::io;

    #[test]
    fn no_configuration() {
        let actual = ModuleRenderer::new("scaleway").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn project_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env(
                "SCW_DEFAULT_PROJECT_ID",
                "11111111-1111-1111-1111-111111111111",
            )
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple
                .bold()
                .paint("☁️ 11111111-1111-1111-1111-111111111111 ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn region_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_DEFAULT_REGION", "fr-par")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("☁️ fr-par ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn zone_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_DEFAULT_ZONE", "fr-par-1")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("☁️ fr-par-1 ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn profile_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_PROFILE", "production")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("☁️ production ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn api_url_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_API_URL", "https://api.scaleway.com")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("☁️ https://api.scaleway.com ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn project_and_region_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env(
                "SCW_DEFAULT_PROJECT_ID",
                "11111111-1111-1111-1111-111111111111",
            )
            .env("SCW_DEFAULT_REGION", "fr-par")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue
                .bold()
                .paint("☁️ 11111111-1111-1111-1111-111111111111(fr-par) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn project_set_with_alias() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env(
                "SCW_DEFAULT_PROJECT_ID",
                "11111111-1111-1111-1111-111111111111",
            )
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .config(toml::toml! {
                [scaleway.project_aliases]
                "11111111-1111-1111-1111-111111111111" = "my-project"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("☁️ my-project ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn region_set_with_alias() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_DEFAULT_REGION", "fr-par")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .config(toml::toml! {
                [scaleway.region_aliases]
                fr-par = "paris"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("☁️ paris ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn api_url_set_with_alias() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env("SCW_API_URL", "https://api.scaleway.com")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .config(toml::toml! {
                [scaleway.api_url_aliases]
                "https://api.scaleway.com" = "prod"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("☁️ prod ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn all_fields_set() -> io::Result<()> {
        let (module_renderer, dir) = ModuleRenderer::new_with_home("scaleway")?;
        let actual = module_renderer
            .env(
                "SCW_DEFAULT_PROJECT_ID",
                "11111111-1111-1111-1111-111111111111",
            )
            .env("SCW_DEFAULT_REGION", "fr-par")
            .env("SCW_PROFILE", "production")
            .env("SCW_API_URL", "https://api.scaleway.com")
            .env("SCW_ACCESS_KEY", "dummy")
            .env("SCW_SECRET_KEY", "dummy")
            .config(toml::toml! {
                [scaleway.project_aliases]
                "11111111-1111-1111-1111-111111111111" = "my-project"
                [scaleway.region_aliases]
                fr-par = "paris"
                [scaleway.api_url_aliases]
                "https://api.scaleway.com" = "prod"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("☁️ my-project(paris)(prod) ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn force_display_with_no_credentials() {
        let actual = ModuleRenderer::new("scaleway")
            .config(toml::toml! {
                [scaleway]
                force_display = true
                format = "via [$symbol$project($region)($api_url)]($style) "
            })
            .collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("☁️ ")));

        assert_eq!(expected, actual);
    }
}
