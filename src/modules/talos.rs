use serde_json::Value as JsonValue;
use yaml_rust2::{Yaml, YamlLoader};

use std::borrow::Cow;

use super::{Context, Module, ModuleConfig};

use crate::configs::talos::TalosConfig;
use crate::formatter::StringFormatter;
use crate::utils;

#[derive(Default)]
struct TalosCtxComponents {
    cluster: Option<String>,
    roles: Option<Vec<String>>,
    endpoints: Option<Vec<String>>,
}

fn get_current_talos_context_name<T: DataValue>(document: &T) -> Option<&str> {
    document
        .get("context")
        .and_then(DataValue::as_str)
        .filter(|s| !s.is_empty())
}

fn get_talos_ctx_components<T: DataValue>(
    document: &T,
    current_ctx_name: &str,
) -> Option<TalosCtxComponents> {
    document
        .get("contexts")?
        .get(current_ctx_name)
        .map(|ctx| TalosCtxComponents {
            cluster: ctx
                .get("cluster")
                .and_then(DataValue::as_str)
                .map(String::from),
            roles: ctx
                .get("roles")
                .and_then(DataValue::as_array)
                .and_then(|roles| {
                    roles
                        .iter()
                        .map(|role| DataValue::as_str(*role).map(String::from))
                        .collect()
                }),
            endpoints: ctx
                .get("endpoints")
                .and_then(DataValue::as_array)
                .and_then(|endpoints| {
                    endpoints
                        .iter()
                        .map(|endpoint| DataValue::as_str(*endpoint).map(String::from))
                        .collect()
                }),
        })
}

fn get_aliased_name<'a>(
    pattern: Option<&'a str>,
    current_value: Option<&str>,
    alias: Option<&'a str>,
) -> Option<String> {
    let replacement = alias.or(current_value)?.to_string();
    let Some(pattern) = pattern else {
        // If user pattern not set, treat it as a match-all pattern
        return Some(replacement);
    };
    // If a pattern is set, but we have no value, there is no match
    let value = current_value?;
    if value == pattern {
        return Some(replacement);
    }
    let re = match regex::Regex::new(&format!("^{pattern}$")) {
        Ok(re) => re,
        Err(error) => {
            log::warn!(
                "Could not compile regular expression `{}`:\n{}",
                &format!("^{pattern}$\n"),
                error
            );
            return None;
        }
    };
    let replaced = re.replace(value, replacement.as_str());
    match replaced {
        Cow::Owned(replaced) => Some(replaced),
        // It didn't match...
        Cow::Borrowed(_) => None,
    }
}

#[derive(Debug)]
enum Document {
    Json(JsonValue),
    Yaml(Yaml),
}

trait DataValue {
    fn get(&self, key: &str) -> Option<&Self>;
    fn as_str(&self) -> Option<&str>;
    fn as_array(&self) -> Option<Vec<&Self>>;
}

impl DataValue for JsonValue {
    fn get(&self, key: &str) -> Option<&Self> {
        self.get(key)
    }

    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    fn as_array(&self) -> Option<Vec<&Self>> {
        self.as_array().map(|arr| arr.iter().collect())
    }
}

impl DataValue for Yaml {
    fn get(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Hash(map) => map.get(&Self::String(key.to_string())),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    fn as_array(&self) -> Option<Vec<&Self>> {
        match self {
            Self::Array(arr) => Some(arr.iter().collect()),
            _ => None,
        }
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("talos");
    let config: TalosConfig = TalosConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let have_env_config = !config.detect_env_vars.is_empty();
    let have_env_vars = have_env_config.then(|| context.detect_env_vars(&config.detect_env_vars));

    // If we have some config for doing the directory scan then we use it but if we don't then we
    // assume we should treat it like the module is enabled to preserve backward compatibility.
    let have_scan_config = [
        &config.detect_files,
        &config.detect_folders,
        &config.detect_extensions,
    ]
    .into_iter()
    .any(|v| !v.is_empty());

    let is_talos_project = have_scan_config.then(|| {
        context.try_begin_scan().is_some_and(|scanner| {
            scanner
                .set_files(&config.detect_files)
                .set_folders(&config.detect_folders)
                .set_extensions(&config.detect_extensions)
                .is_match()
        })
    });

    if !is_talos_project.or(have_env_vars).unwrap_or(true) {
        return None;
    }

    let default_config_file = context.get_home()?.join(".talos").join("config");

    let talos_cfg = context
        .get_env("TALOSCONFIG")
        .unwrap_or(default_config_file.to_str()?.to_string());

    let talosconfig = parse_talosconfig(utils::read_file(&talos_cfg).ok());

    let current_talos_ctx_name = talosconfig.as_ref().and_then(|d| match d {
        Document::Json(json) => get_current_talos_context_name(json),
        Document::Yaml(yaml) => get_current_talos_context_name(yaml),
    })?;

    let ctx_error = || {
        log::warn!(
            "Invalid TALOSCONFIG: identified current context `{}`, but couldn't find the context in any config file(s): `{}`.\n",
            &current_talos_ctx_name,
            &talos_cfg
        );
        TalosCtxComponents::default()
    };

    let ctx_components: TalosCtxComponents = match &talosconfig {
        Some(Document::Json(json)) => {
            get_talos_ctx_components(json, current_talos_ctx_name).unwrap_or_else(ctx_error)
        }
        Some(Document::Yaml(yaml)) => {
            get_talos_ctx_components(yaml, current_talos_ctx_name).unwrap_or_else(ctx_error)
        }
        None => ctx_error(),
    };

    let endpoints_string = ctx_components
        .endpoints
        .as_ref()
        .map(|endpoints| endpoints.join(" "));

    let roles_string = ctx_components.roles.as_ref().map(|roles| roles.join(" "));

    // Select the first style that matches the context_pattern
    let (matched_context_config, display_context, display_endpoints, display_roles) = config
        .contexts
        .iter()
        .find_map(|context_config| {
            let context_alias = get_aliased_name(
                Some(context_config.context_pattern),
                Some(current_talos_ctx_name),
                context_config.context_alias,
            )?;

            Some((
                Some(context_config),
                context_alias,
                &endpoints_string,
                &roles_string,
            ))
        })
        .unwrap_or_else(|| {
            (
                None,
                current_talos_ctx_name.to_string(),
                &endpoints_string,
                &roles_string,
            )
        });

    let display_style = matched_context_config
        .and_then(|ctx_cfg| ctx_cfg.style)
        .unwrap_or(config.style);
    let display_symbol = matched_context_config
        .and_then(|ctx_cfg| ctx_cfg.symbol)
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
                "context" => Some(Ok(Cow::Borrowed(display_context.as_str()))),
                "cluster" => ctx_components
                    .cluster
                    .as_ref()
                    .map(|talos_cluster| Ok(Cow::Borrowed(talos_cluster.as_str()))),
                "endpoints" => display_endpoints
                    .as_ref()
                    .map(|cluster_endpoints| Ok(Cow::Borrowed(cluster_endpoints.as_str()))),
                "roles" => display_roles
                    .as_ref()
                    .map(|user_roles| Ok(Cow::Borrowed(user_roles.as_str()))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `talos`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn parse_talosconfig(raw_talosconfig: Option<String>) -> Option<Document> {
    match raw_talosconfig {
        Some(value) => match value.chars().next() {
            // Parsing as json is about an order of magnitude faster than parsing
            // as yaml, so do that if possible.
            Some('{') => match serde_json::from_str(&value) {
                Ok(json) => Some(Document::Json(json)),
                Err(_) => parse_yaml(&value),
            },
            _ => parse_yaml(&value),
        },
        _ => None,
    }
}

fn parse_yaml(s: &str) -> Option<Document> {
    YamlLoader::load_from_str(s)
        .ok()
        .and_then(|yaml| yaml.into_iter().next().map(Document::Yaml))
}

#[cfg(test)]
mod tests {
    use crate::configs::talos::DEFAULT_FORMAT_STRING;
    use crate::modules::talos::Document;
    use crate::modules::talos::parse_talosconfig;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{File, create_dir};
    use std::io::{self, Write};

    const DEFAULT_TEST_TALOSCONFIG: &[u8; 100] = b"
contexts:
  test_context:
    endpoints:
      - endpoint1
      - endpoint2
context: test_context
";

    #[test]
    fn test_none_when_disabled() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .collect();

        assert_eq!(None, actual);

        dir.close()
    }

    #[test]
    fn test_none_when_no_detected_files_folders_or_env_vars() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_files = ["talos.ext"]
                detect_extensions = ["talos"]
                detect_folders = ["talos_folder"]
                detect_env_vars = ["talos_env_var"]
            })
            .collect();

        assert_eq!(None, actual);

        dir.close()
    }

    #[test]
    fn test_with_detected_files_folder_and_env_vars() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let dir_with_file = tempfile::tempdir()?;
        File::create(dir_with_file.path().join("talos.ext"))?.sync_all()?;

        let actual_file = ModuleRenderer::new("talos")
            .path(dir_with_file.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_files = ["talos.ext"]
                detect_extensions = ["talos"]
                detect_folders = ["talos_folder"]
            })
            .collect();

        let dir_with_ext = tempfile::tempdir()?;
        File::create(dir_with_ext.path().join("test.talos"))?.sync_all()?;

        let actual_ext = ModuleRenderer::new("talos")
            .path(dir_with_ext.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_files = ["talos.ext"]
                detect_extensions = ["talos"]
                detect_folders = ["talos_folder"]
            })
            .collect();

        let dir_with_dir = tempfile::tempdir()?;
        create_dir(dir_with_dir.path().join("talos_folder"))?;

        let actual_dir = ModuleRenderer::new("talos")
            .path(dir_with_dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_files = ["talos.ext"]
                detect_extensions = ["talos"]
                detect_folders = ["talos_folder"]
            })
            .collect();

        let empty_dir = tempfile::tempdir()?;

        let actual_env_var = ModuleRenderer::new("talos")
            .path(empty_dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .env("TEST_TALOS_ENV", "foo")
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_env_vars = ["TEST_TALOS_ENV"]
            })
            .collect();

        let actual_none = ModuleRenderer::new("talos")
            .path(empty_dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                detect_files = ["talos.ext"]
            })
            .collect();

        let expected = Some(format!(
            "{} via {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context"),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));

        assert_eq!(expected, actual_file);
        assert_eq!(expected, actual_ext);
        assert_eq!(expected, actual_dir);
        assert_eq!(expected, actual_env_var);
        assert_eq!(None, actual_none);

        dir.close()
    }

    fn base_test_ctx_alias(ctx_name: &str, config: toml::Table, expected: &str) -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            format!(
                "
contexts:
  {ctx_name}:
    endpoints:
      - endpoint1
      - endpoint2
context: {ctx_name}
"
            )
            .as_bytes(),
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(config)
            .collect();

        let final_expected = Some(format!(
            "{} via {} in ",
            Color::Fixed(208).bold().paint(expected),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));
        assert_eq!(final_expected, actual);

        dir.close()
    }

    #[test]
    fn test_ctx_alias_simple() -> io::Result<()> {
        base_test_ctx_alias(
            "test_context",
            toml::toml! {
                [talos]
                disabled = false
                [[talos.contexts]]
                context_pattern = "test_context"
                context_alias = "test_alias"
            },
            "󰰥 test_alias",
        )
    }

    #[test]
    fn test_ctx_alias_regex() -> io::Result<()> {
        base_test_ctx_alias(
            "namespace/openshift-cluster/user",
            toml::toml! {
                [talos]
                disabled = false
                [[talos.contexts]]
                context_pattern = ".*/openshift-cluster/.*"
                context_alias = "test_alias"
            },
            "󰰥 test_alias",
        )
    }

    #[test]
    fn test_ctx_alias_regex_replace() -> io::Result<()> {
        base_test_ctx_alias(
            "cluster_infra-talos-1",
            toml::toml! {
                [talos]
                disabled = false
                [[talos.contexts]]
                context_pattern = "cluster_infra-(?P<name>\\w+-1)"
                context_alias = "ctx: $name"
            },
            // "󰰥 example: cluster-1",
            "󰰥 ctx: talos-1",
        )
    }

    #[test]
    fn test_ctx_alias_broken_regex() -> io::Result<()> {
        base_test_ctx_alias(
            "input",
            toml::toml! {
                [talos]
                disabled = false
                [[talos.contexts]]
                context_pattern = "input[.*"
                context_alias = "this does not match"
            },
            "󰰥 input",
        )
    }

    #[test]
    fn test_config_with_multiple_ctxs() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
contexts:
  test_context1:
    endpoints:
      - endpoint1
      - endpoint2
  test_context2:
    endpoints:
      - endpoint3
      - endpoint4
context: test_context1
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} via {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context1"),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_with_cluster_field() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
contexts:
  test_context:
    endpoints:
      - endpoint1
      - endpoint2
    cluster: test_cluster
context: test_context
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                format = DEFAULT_FORMAT_STRING
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{}({}) via {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context"),
            Color::Fixed(208).paint("test_cluster"),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_no_cluster_field() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                format = DEFAULT_FORMAT_STRING
                disabled = false
            })
            .collect();

        // let expected = Some("☸ test_user test_namespace".to_string());
        let expected = Some(format!(
            "{} via {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context"),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_with_roles() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
contexts:
  test_context:
    endpoints:
      - endpoint1
      - endpoint2
    roles:
      - role1
      - role2
context: test_context
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                format = DEFAULT_FORMAT_STRING
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} via {} as {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context"),
            Color::Fixed(208).paint("endpoint1 endpoint2"),
            Color::Fixed(208).paint("role1 role2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_overwrites_defaults() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
contexts:
  test_context:
    cluster: test_cluster
context: test_context
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                style = "bold red"

                [[talos.contexts]]
                context_pattern = "test.*"
                style = "bold green"
                symbol = "§ "
            })
            .collect();

        let expected = Some(format!(
            "{}({}) via  in ",
            Color::Green.bold().paint("§ test_context"),
            Color::Green.bold().paint("test_cluster"),
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_contexts_does_not_match() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                style = "bold red"
                contexts = [
                    {context_pattern = "tests_.*", style = "bold green", symbol = "§ "},
                ]
            })
            .collect();

        let expected = Some(format!(
            "{} via {} in ",
            Color::Red.bold().paint("󰰥 test_context"),
            Color::Red.bold().paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_bad_regex_should_not_panic() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(DEFAULT_TEST_TALOSCONFIG)?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                style = "bold red"
                contexts = [
                    {context_pattern = "tests_(.*", style = "bold green", symbol = "§ "},
                ]
            })
            .collect();

        let expected = Some(format!(
            "{} via {} in ",
            Color::Red.bold().paint("󰰥 test_context"),
            Color::Red.bold().paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_json_talosconfig_is_parsed_as_json() {
        let json_talosconfig = r#"{
  "apiVersion": "v1",
  "clusters": [],
  "contexts": [
    {
      "context": {
        "user": "test_user",
        "namespace": "test_namespace"
      },
      "name": "test_context"
    }
  ],
  "current-context": "test_context",
  "kind": "Config",
  "preferences": {},
  "users": []
}"#
        .to_string();

        let talosconfig = Some(json_talosconfig);
        let actual = parse_talosconfig(talosconfig).unwrap();
        match actual {
            Document::Json(..) => {}
            _ => panic!("Expected Document::Json, got {actual:?}"),
        }
    }

    #[test]
    fn fallback_to_yaml_parsing() {
        let json_talosconfig = r#"{
  "apiVersion": v1,
  "clusters": [],
  "contexts": [
    {
      "context": {
        "user": test_user,
        "namespace": test_namespace
      },
      "name": test_context
    }
  ],
  "current-context": test_context,
  "kind": Config,
  "preferences": {},
  "users": []
}"#
        .to_string();

        let talosconfig = Some(json_talosconfig);
        let actual = parse_talosconfig(talosconfig).unwrap();
        match actual {
            Document::Yaml(..) => {}
            Document::Json(_) => panic!("Expected Document::Yaml, got {actual:?}"),
        }
    }

    #[test]
    fn test_parse_json_talosconfig() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            br#"{
  "contexts": {
    "test_context": {
      "endpoints": [ "endpoint1", "endpoint2" ],
      "cluster": "test_cluster"
    }
  },
  "context": "test_context"
}
"#,
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("talos")
            .path(dir.path())
            .env("TALOSCONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [talos]
                disabled = false
                format = DEFAULT_FORMAT_STRING
            })
            .collect();

        let expected = Some(format!(
            "{}({}) via {} in ",
            Color::Fixed(208).bold().paint("󰰥 test_context"),
            Color::Fixed(208).paint("test_cluster"),
            Color::Fixed(208).paint("endpoint1 endpoint2")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
