use serde_json::Value as JsonValue;
use yaml_rust2::{Yaml, YamlLoader};

use std::borrow::Cow;
use std::env;

use super::{Context, Module, ModuleConfig};

use crate::configs::kubernetes::KubernetesConfig;
use crate::formatter::StringFormatter;
use crate::utils;

#[derive(Default)]
struct KubeCtxComponents {
    user: Option<String>,
    namespace: Option<String>,
    cluster: Option<String>,
}

fn get_current_kube_context_name<T: DataValue>(document: &T) -> Option<&str> {
    document
        .get("current-context")
        .and_then(DataValue::as_str)
        .filter(|s| !s.is_empty())
}

fn get_kube_ctx_components<T: DataValue>(
    document: &T,
    current_ctx_name: &str,
) -> Option<KubeCtxComponents> {
    document
        .get("contexts")?
        .as_array()?
        .iter()
        .find(|ctx| ctx.get("name").and_then(DataValue::as_str) == Some(current_ctx_name))
        .map(|ctx| KubeCtxComponents {
            user: ctx
                .get("context")
                .and_then(|v| v.get("user"))
                .and_then(DataValue::as_str)
                .map(String::from),
            namespace: ctx
                .get("context")
                .and_then(|v| v.get("namespace"))
                .and_then(DataValue::as_str)
                .map(String::from),
            cluster: ctx
                .get("context")
                .and_then(|v| v.get("cluster"))
                .and_then(DataValue::as_str)
                .map(String::from),
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
                &format!("^{pattern}$"),
                error
            );
            return None;
        }
    };
    let replaced = re.replace(value, replacement.as_str());
    match replaced {
        Cow::Owned(replaced) => Some(replaced),
        // It didn't match...
        _ => None,
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
    let mut module = context.new_module("kubernetes");
    let config: KubernetesConfig = KubernetesConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

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

    let is_kube_project = have_scan_config.then(|| {
        context.try_begin_scan().is_some_and(|scanner| {
            scanner
                .set_files(&config.detect_files)
                .set_folders(&config.detect_folders)
                .set_extensions(&config.detect_extensions)
                .is_match()
        })
    });

    if !is_kube_project.or(have_env_vars).unwrap_or(true) {
        return None;
    }

    let default_config_file = context.get_home()?.join(".kube").join("config");

    let kube_cfg = context
        .get_env("KUBECONFIG")
        .unwrap_or(default_config_file.to_str()?.to_string());

    let raw_kubeconfigs = env::split_paths(&kube_cfg).map(|file| utils::read_file(file).ok());
    let kubeconfigs = parse_kubeconfigs(raw_kubeconfigs);

    let current_kube_ctx_name = kubeconfigs.iter().find_map(|v| match v {
        Document::Json(json) => get_current_kube_context_name(json),
        Document::Yaml(yaml) => get_current_kube_context_name(yaml),
    })?;

    // Even if we have multiple config files, the first key wins
    // https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/
    // > Never change the value or map key. ... Example: If two files specify a red-user,
    // > use only values from the first file's red-user. Even if the second file has
    // > non-conflicting entries under red-user, discard them.
    // for that reason, we can pick the first context with that name
    let ctx_components: KubeCtxComponents = kubeconfigs.iter().find_map(|kubeconfig|  match kubeconfig {
        Document::Json(json) => get_kube_ctx_components(json, current_kube_ctx_name),
        Document::Yaml(yaml) => get_kube_ctx_components(yaml, current_kube_ctx_name),
    }).unwrap_or_else(|| {
            // TODO: figure out if returning is more sensible. But currently we have tests depending on this
            log::warn!(
                "Invalid KUBECONFIG: identified current-context `{}`, but couldn't find the context in any config file(s): `{}`.\n",
                &current_kube_ctx_name,
                &kube_cfg
                );
            KubeCtxComponents::default()
        });

    // Select the first style that matches the context_pattern and,
    // if it is defined, the user_pattern
    let (matched_context_config, display_context, display_user) = config
        .contexts
        .iter()
        .find_map(|context_config| {
            let context_alias = get_aliased_name(
                Some(context_config.context_pattern),
                Some(current_kube_ctx_name),
                context_config.context_alias,
            )?;

            let user_alias = get_aliased_name(
                context_config.user_pattern,
                ctx_components.user.as_deref(),
                context_config.user_alias,
            );
            if matches!((context_config.user_pattern, &user_alias), (Some(_), None)) {
                // defined pattern, but it didn't match
                return None;
            }

            Some((Some(context_config), context_alias, user_alias))
        })
        .unwrap_or_else(|| (None, current_kube_ctx_name.to_string(), ctx_components.user));

    // TODO: remove deprecated aliases after starship 2.0
    let display_context =
        deprecated::get_alias(display_context, &config.context_aliases, "context").unwrap();
    let display_user =
        display_user.and_then(|user| deprecated::get_alias(user, &config.user_aliases, "user"));

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
                "namespace" => ctx_components
                    .namespace
                    .as_ref()
                    .map(|kube_ns| Ok(Cow::Borrowed(kube_ns.as_str()))),
                "cluster" => ctx_components
                    .cluster
                    .as_ref()
                    .map(|kube_cluster| Ok(Cow::Borrowed(kube_cluster.as_str()))),
                "user" => display_user
                    .as_ref()
                    .map(|kube_user| Ok(Cow::Borrowed(kube_user.as_str()))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `kubernetes`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_kubeconfigs<I>(raw_kubeconfigs: I) -> Vec<Document>
where
    I: Iterator<Item = Option<String>>,
{
    raw_kubeconfigs
        .filter_map(|content| match content {
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
        })
        .collect()
}

fn parse_yaml(s: &str) -> Option<Document> {
    YamlLoader::load_from_str(s)
        .ok()
        .and_then(|yaml| yaml.into_iter().next().map(Document::Yaml))
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
            // simple match without regex
            Some((*val).to_string())
        } else {
            // regex match
            aliases.iter().find_map(|(k, v)| {
                let re = regex::Regex::new(&format!("^{k}$")).ok()?;
                let replaced = re.replace(current_value.as_str(), *v);
                match replaced {
                    // We have a match if the replaced string is different from the original
                    Cow::Owned(replaced) => Some(replaced),
                    _ => None,
                }
            })
        };

        match alias {
            Some(alias) => {
                log::warn!(
                        "Usage of '{}_aliases' is deprecated and will be removed in 2.0; Use 'contexts' with '{}_alias' instead. (`{}` -> `{}`)",
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

#[cfg(test)]
mod tests {
    use crate::modules::kubernetes::parse_kubeconfigs;
    use crate::modules::kubernetes::Document;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::env;
    use std::fs::{create_dir, File};
    use std::io::{self, Write};

    #[test]
    fn test_none_when_disabled() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .collect();

        assert_eq!(None, actual);

        dir.close()
    }

    #[test]
    fn test_none_when_no_detected_files_folders_or_env_vars() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_files = ["k8s.ext"]
                detect_extensions = ["k8s"]
                detect_folders = ["k8s_folder"]
                detect_env_vars = ["k8s_env_var"]
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
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let dir_with_file = tempfile::tempdir()?;
        File::create(dir_with_file.path().join("k8s.ext"))?.sync_all()?;

        let actual_file = ModuleRenderer::new("kubernetes")
            .path(dir_with_file.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_files = ["k8s.ext"]
                detect_extensions = ["k8s"]
                detect_folders = ["k8s_folder"]
            })
            .collect();

        let dir_with_ext = tempfile::tempdir()?;
        File::create(dir_with_ext.path().join("test.k8s"))?.sync_all()?;

        let actual_ext = ModuleRenderer::new("kubernetes")
            .path(dir_with_ext.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_files = ["k8s.ext"]
                detect_extensions = ["k8s"]
                detect_folders = ["k8s_folder"]
            })
            .collect();

        let dir_with_dir = tempfile::tempdir()?;
        create_dir(dir_with_dir.path().join("k8s_folder"))?;

        let actual_dir = ModuleRenderer::new("kubernetes")
            .path(dir_with_dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_files = ["k8s.ext"]
                detect_extensions = ["k8s"]
                detect_folders = ["k8s_folder"]
            })
            .collect();

        let empty_dir = tempfile::tempdir()?;

        let actual_env_var = ModuleRenderer::new("kubernetes")
            .path(empty_dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .env("TEST_K8S_ENV", "foo")
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_env_vars = ["TEST_K8S_ENV"]
            })
            .collect();

        let actual_none = ModuleRenderer::new("kubernetes")
            .path(empty_dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                detect_files = ["k8s.ext"]
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context")
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
apiVersion: v1
clusters: []
contexts: []
current-context: {ctx_name}
kind: Config
preferences: {{}}
users: []
"
            )
            .as_bytes(),
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(config)
            .collect();

        let expected = Some(format!("{} in ", Color::Cyan.bold().paint(expected)));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_ctx_alias_simple() -> io::Result<()> {
        base_test_ctx_alias(
            "test_context",
            toml::toml! {
                [kubernetes]
                disabled = false
                [kubernetes.context_aliases]
                "test_context" = "test_alias"
                ".*" = "literal match has precedence"
            },
            "☸ test_alias",
        )
    }

    #[test]
    fn test_ctx_alias_regex() -> io::Result<()> {
        base_test_ctx_alias(
            "namespace/openshift-cluster/user",
            toml::toml! {
                [kubernetes]
                disabled = false
                [kubernetes.context_aliases]
                ".*/openshift-cluster/.*" = "test_alias"
            },
            "☸ test_alias",
        )
    }

    #[test]
    fn test_ctx_alias_regex_replace() -> io::Result<()> {
        base_test_ctx_alias(
            "gke_infra-cluster-28cccff6_europe-west4_cluster-1",
            toml::toml! {
                [kubernetes]
                disabled = false
                [kubernetes.context_aliases]
                "gke_.*_(?P<cluster>[\\w-]+)" = "example: $cluster"
            },
            "☸ example: cluster-1",
        )
    }

    #[test]
    fn test_config_context_ctx_alias_regex_replace() -> io::Result<()> {
        base_test_ctx_alias(
            "gke_infra-cluster-28cccff6_europe-west4_cluster-1",
            toml::toml! {
                [kubernetes]
                disabled = false
                [[kubernetes.contexts]]
                context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
                context_alias = "example: $cluster"
            },
            "☸ example: cluster-1",
        )
    }

    #[test]
    fn test_ctx_alias_broken_regex() -> io::Result<()> {
        base_test_ctx_alias(
            "input",
            toml::toml! {
                [kubernetes]
                disabled = false
                [kubernetes.context_aliases]
                "input[.*" = "this does not match"
            },
            "☸ input",
        )
    }

    #[test]
    fn test_single_config_file_no_ns() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_single_config_file_with_ns() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_single_config_file_with_multiple_ctxs() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: another_cluster
      user: another_user
      namespace: another_namespace
    name: another_context
  - context:
      cluster: test_cluster
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_multiple_config_files_with_context_defined_once() -> io::Result<()> {
        // test that we get the current context from the first config file in the KUBECONFIG,
        // no matter if it is only defined in the latter
        let dir = tempfile::tempdir()?;

        let filename_cc = dir.path().join("config_cc");

        let mut file_cc = File::create(&filename_cc)?;
        file_cc.write_all(
            b"
apiVersion: v1
clusters: []
contexts: []
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file_cc.sync_all()?;

        let filename_ctx = dir.path().join("config_ctx");
        let mut file_ctx = File::create(&filename_ctx)?;
        file_ctx.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
      namespace: test_namespace
    name: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file_ctx.sync_all()?;

        // Test current_context first
        let actual_cc_first = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env(
                "KUBECONFIG",
                env::join_paths([&filename_cc, &filename_ctx])
                    .unwrap()
                    .to_string_lossy(),
            )
            .config(toml::toml! {
                [kubernetes]
                disabled = false
            })
            .collect();

        // And test with context and namespace first
        let actual_ctx_first = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env(
                "KUBECONFIG",
                env::join_paths([&filename_ctx, &filename_cc])
                    .unwrap()
                    .to_string_lossy(),
            )
            .config(toml::toml! {
                [kubernetes]
                disabled = false
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual_cc_first);
        assert_eq!(expected, actual_ctx_first);

        dir.close()
    }

    #[test]
    fn test_multiple_config_files_with_context_defined_twice() -> io::Result<()> {
        // tests that, if two files contain the same context,
        // only the context config from the first is used.
        let dir = tempfile::tempdir()?;

        let config1 = dir.path().join("config1");

        let mut file1 = File::create(&config1)?;
        file1.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster1
      namespace: test_namespace1
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file1.sync_all()?;

        let config2 = dir.path().join("config2");

        let mut file2 = File::create(&config2)?;
        file2.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster2
      user: test_user2
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file2.sync_all()?;

        let paths1 = [config1.clone(), config2.clone()];
        let kubeconfig_content1 = env::join_paths(paths1.iter()).unwrap();

        let actual1 = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", kubeconfig_content1.to_string_lossy())
            .config(toml::toml! {
                [kubernetes]
                format = "($user )($cluster )($namespace )"
                disabled = false
            })
            .collect();

        let expected1 = Some("test_cluster1 test_namespace1 ".to_string());
        assert_eq!(expected1, actual1);

        let paths2 = [config2, config1];
        let kubeconfig_content2 = env::join_paths(paths2.iter()).unwrap();

        let actual2 = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", kubeconfig_content2.to_string_lossy())
            .config(toml::toml! {
                [kubernetes]
                format = "($user )($cluster )($namespace )"
                disabled = false
            })
            .collect();

        let expected2 = Some("test_user2 test_cluster2 ".to_string());
        assert_eq!(expected2, actual2);

        dir.close()
    }

    fn base_test_user_alias(
        user_name: &str,
        config: toml::Table,
        expected: &str,
    ) -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            format!(
                "
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: {user_name}
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {{}}
users: []
"
            )
            .as_bytes(),
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(config)
            .collect();

        let expected = Some(format!("{} in ", Color::Cyan.bold().paint(expected)));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_user_alias_simple() -> io::Result<()> {
        base_test_user_alias(
            "test_user",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [kubernetes.user_aliases]
                "test_user" = "test_alias"
                ".*" = "literal match has precedence"
            },
            "☸ test_context (test_alias)",
        )
    }

    #[test]
    fn test_user_alias_regex() -> io::Result<()> {
        base_test_user_alias(
            "openshift-cluster/user",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [kubernetes.user_aliases]
                "openshift-cluster/.*" = "test_alias"
            },
            "☸ test_context (test_alias)",
        )
    }

    #[test]
    fn test_user_alias_regex_replace() -> io::Result<()> {
        base_test_user_alias(
            "gke_infra-user-28cccff6_europe-west4_cluster-1",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [kubernetes.user_aliases]
                "gke_.*_(?P<cluster>[\\w-]+)" = "example: $cluster"
            },
            "☸ test_context (example: cluster-1)",
        )
    }

    #[test]
    fn test_config_context_user_alias_regex_replace() -> io::Result<()> {
        base_test_user_alias(
            "gke_infra-user-28cccff6_europe-west4_cluster-1",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [[kubernetes.contexts]]
                context_pattern = ".*"
                user_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
                user_alias = "example: $cluster"
            },
            "☸ test_context (example: cluster-1)",
        )
    }

    #[test]
    fn test_user_alias_broken_regex() -> io::Result<()> {
        base_test_user_alias(
            "input",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [kubernetes.user_aliases]
                "input[.*" = "this does not match"
            },
            "☸ test_context (input)",
        )
    }

    #[test]
    fn test_user_should_use_default_if_no_matching_alias() -> io::Result<()> {
        base_test_user_alias(
            "gke_infra-user-28cccff6_europe-west4_cluster-1",
            toml::toml! {
                [kubernetes]
                disabled = false
                format = "[$symbol$context( \\($user\\))]($style) in "
                [kubernetes.user_aliases]
                "([A-Z])\\w+" = "this does not match"
                "gke_infra-user-28cccff6" = "this does not match"
            },
            "☸ test_context (gke_infra-user-28cccff6_europe-west4_cluster-1)",
        )
    }

    #[test]
    fn test_kube_user() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                format = "($user)"
                disabled = false
            })
            .collect();

        let expected = Some("test_user".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_kube_cluster() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                format = "($cluster)"
                disabled = false
            })
            .collect();

        let expected = Some("test_cluster".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_kube_user_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      cluster: test_cluster
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                format = "$symbol($user )($cluster )($namespace)"
                disabled = false
            })
            .collect();

        let expected = Some("☸ test_cluster test_namespace".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_kube_cluster_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                format = "$symbol($user )($cluster )($namespace)"
                disabled = false
            })
            .collect();

        let expected = Some("☸ test_user test_namespace".to_string());
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
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"

                [[kubernetes.contexts]]
                context_pattern = "test.*"
                style = "bold green"
                symbol = "§ "
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Green.bold().paint("§ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_both_pattern_must_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                format = "$symbol$context ($user )"

                [[kubernetes.contexts]]
                context_pattern = "test.*"
                user_pattern = "test.*"
                context_alias = "yy"
                user_alias = "xx"
                symbol = "§ "
            })
            .collect();

        let expected = Some("§ yy xx ".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_only_one_pattern_matches() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                format = "$symbol$context ($user )"

                [[kubernetes.contexts]]
                context_pattern = "test.*"
                user_pattern = "test_BAD.*"
                context_alias = "yy"
                user_alias = "xx"
                symbol = "§ "
            })
            .collect();

        let expected = Some("☸ test_context test_user ".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_uses_aliases() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"
                format = "$symbol($user )($context )($cluster )($namespace)"

                [[kubernetes.contexts]]
                context_pattern = "test.*"
                context_alias = "xyz"
                user_alias = "abc"
                symbol = "§ "
            })
            .collect();

        let expected = Some("§ abc xyz test_namespace".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_user_pattern_does_not_match() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"
                format = "$symbol($user )($context )($cluster )($namespace)"

                [[kubernetes.contexts]]
                context_pattern = "test"
                user_pattern = "not_matching"
                context_alias = "xyz"
                user_alias = "abc"
                symbol = "§ "
            })
            .collect();

        let expected = Some("☸ test_user test_context test_namespace".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_contexts_does_not_match() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"
                contexts = [
                    {context_pattern = "tests_.*", style = "bold green", symbol = "§ "},
                ]
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Red.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_config_context_bad_regex_should_not_panic() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            b"
apiVersion: v1
clusters: []
contexts:
  - context:
      user: test_user
      namespace: test_namespace
    name: test_context
current-context: test_context
kind: Config
preferences: {}
users: []
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"
                contexts = [
                    {context_pattern = "tests_(.*", style = "bold green", symbol = "§ "},
                ]
            })
            .collect();

        let expected = Some(format!(
            "{} in ",
            Color::Red.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_json_kubeconfig_is_parsed_as_json() -> std::io::Result<()> {
        let json_kubeconfig = r#"{
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

        let kubeconfigs = [Some(json_kubeconfig)];
        let results = parse_kubeconfigs(kubeconfigs.iter().cloned());
        let actual = results.first().unwrap();
        match actual {
            Document::Json(..) => {}
            _ => panic!("Expected Document::Json, got {actual:?}"),
        }
        Ok(())
    }

    #[test]
    fn fallback_to_yaml_parsing() -> std::io::Result<()> {
        let json_kubeconfig = r#"{
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

        let kubeconfigs = [Some(json_kubeconfig)];
        let results = parse_kubeconfigs(kubeconfigs.iter().cloned());
        let actual = results.first().unwrap();
        match actual {
            Document::Yaml(..) => {}
            _ => panic!("Expected Document::Yaml, got {actual:?}"),
        }
        Ok(())
    }

    #[test]
    fn test_parse_json_kubeconfig() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let filename = dir.path().join("config");
        let mut file = File::create(&filename)?;
        file.write_all(
            br#"{
  "contexts": [
    {
      "name": "test_context",
      "context": {
        "user": "test_user",
        "namespace": "test_namespace"
      }
    }
  ],
  "current-context": "test_context",
  "kind": "Config",
  "apiVersion": "v1"
}
"#,
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                format = "($user )($context )($cluster )($namespace)"
            })
            .collect();

        let expected = Some("test_user test_context test_namespace".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }
}
