use yaml_rust::YamlLoader;

use std::borrow::Cow;
use std::env;
use std::path;

use super::{Context, Module, ModuleConfig};

use crate::configs::kubernetes::KubernetesConfig;
use crate::formatter::StringFormatter;
use crate::utils;

struct KubeCtxComponents {
    user: Option<String>,
    namespace: Option<String>,
    cluster: Option<String>,
}

fn get_kube_context(filename: path::PathBuf) -> Option<String> {
    let contents = utils::read_file(filename).ok()?;

    let yaml_docs = YamlLoader::load_from_str(&contents).ok()?;
    if yaml_docs.is_empty() {
        return None;
    }
    let conf = &yaml_docs[0];

    let current_ctx = conf["current-context"].as_str()?;

    if current_ctx.is_empty() {
        return None;
    }
    Some(current_ctx.to_string())
}

fn get_kube_ctx_component(filename: path::PathBuf, current_ctx: &str) -> Option<KubeCtxComponents> {
    let contents = utils::read_file(filename).ok()?;

    let yaml_docs = YamlLoader::load_from_str(&contents).ok()?;
    if yaml_docs.is_empty() {
        return None;
    }
    let conf = &yaml_docs[0];

    let ctx_yaml = conf["contexts"].as_vec().and_then(|contexts| {
        contexts
            .iter()
            .filter_map(|ctx| Some((ctx, ctx["name"].as_str()?)))
            .find(|(_, name)| *name == current_ctx)
    });

    let ctx_components = KubeCtxComponents {
        user: ctx_yaml
            .and_then(|(ctx, _)| ctx["context"]["user"].as_str())
            .and_then(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.to_owned())
            }),
        namespace: ctx_yaml
            .and_then(|(ctx, _)| ctx["context"]["namespace"].as_str())
            .and_then(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.to_owned())
            }),
        cluster: ctx_yaml
            .and_then(|(ctx, _)| ctx["context"]["cluster"].as_str())
            .and_then(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.to_owned())
            }),
    };

    Some(ctx_components)
}

fn match_user_regex(regex: &str, text: &str) -> bool {
    match regex::Regex::new(regex) {
        Ok(re) => re.is_match(text),
        Err(error) => {
            log::warn!(
                "Could not compile regular expression `{}`:\n{}",
                regex,
                error
            );
            false
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

    // If we have some config for doing the directory scan then we use it but if we don't then we
    // assume we should treat it like the module is enabled to preserve backward compatibility.
    let have_scan_config = !(config.detect_files.is_empty()
        && config.detect_folders.is_empty()
        && config.detect_extensions.is_empty());

    let is_kube_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if have_scan_config && !is_kube_project {
        return None;
    }

    let default_config_file = context.get_home()?.join(".kube").join("config");

    let kube_cfg = context
        .get_env("KUBECONFIG")
        .unwrap_or(default_config_file.to_str()?.to_string());

    let kube_ctx = env::split_paths(&kube_cfg).find_map(get_kube_context)?;

    let ctx_components: Vec<KubeCtxComponents> = env::split_paths(&kube_cfg)
        .filter_map(|filename| get_kube_ctx_component(filename, &kube_ctx))
        .collect();

    let kube_user = ctx_components.iter().find_map(|ctx| {
        ctx.user
            .as_ref()
            .map(|kube_user| deprecated::get_kube_user_alias(&config, kube_user))
    });

    // Parse config under `display`.
    // Select the first style that matches the context_pattern and,
    // if it is defined, the user_pattern
    let matched_context_config = config.contexts.iter().find(|context_config| {
        let is_context_match = match_user_regex(context_config.context_pattern, &kube_ctx);
        if !is_context_match {
            return false;
        };

        if let Some(user_pattern) = context_config.user_pattern {
            if let Some(kube_user) = &kube_user {
                match_user_regex(user_pattern, kube_user)
            } else {
                // If a pattern is set, but we have no user, there is no match
                false
            }
        } else {
            // If user pattern not set, treat it as a match-all pattern
            true
        }
    });

    let display_context = match matched_context_config {
        Some(ctx_cfg) => match ctx_cfg.context_alias {
            Some(context_alias) => Cow::Borrowed(context_alias),
            None => deprecated::get_kube_context_alias(&config, &kube_ctx),
        },
        None => deprecated::get_kube_context_alias(&config, &kube_ctx),
    };

    let display_user = match matched_context_config {
        Some(ctx_cfg) => match ctx_cfg.user_alias {
            Some(user_alias) => Some(Cow::Borrowed(user_alias)),
            None => kube_user,
        },
        None => kube_user,
    };

    let display_style = match matched_context_config {
        Some(ctx_cfg) => ctx_cfg.style.unwrap_or(config.style),
        None => config.style,
    };
    let display_symbol = match matched_context_config {
        Some(ctx_cfg) => ctx_cfg.symbol.unwrap_or(config.symbol),
        None => config.symbol,
    };

    let kube_ns = ctx_components.iter().find(|&ctx| ctx.namespace.is_some());
    let kube_cluster = ctx_components.iter().find(|&ctx| ctx.cluster.is_some());

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
                "context" => Some(Ok(display_context.clone())),
                "namespace" => kube_ns.map(|ctx| {
                    // unwrap is safe as kube_ns only holds kube.namespace.is_some()
                    Ok(Cow::Borrowed(ctx.namespace.as_ref().unwrap().as_str()))
                }),
                "cluster" => kube_cluster.map(|ctx| {
                    // unwrap is safe as kube_cluster only holds kube.cluster.is_some()
                    Ok(Cow::Borrowed(ctx.cluster.as_ref().unwrap().as_str()))
                }),
                "user" => display_user.as_ref().map(|user| Ok(user.clone())),
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

mod deprecated {
    use super::KubernetesConfig;
    use std::borrow::Cow;
    use std::collections::HashMap;

    pub fn get_kube_user_alias<'a>(
        config: &'a KubernetesConfig,
        kube_user: &'a str,
    ) -> Cow<'a, str> {
        let user_alias = get_alias(&config.user_aliases, kube_user);
        match user_alias {
            Some(alias) => {
                log::warn!(
                        "Usage of 'user_aliases' is deprecated and will be removed in 2.0; Use 'contexts' with 'user_alias' instead. (`{}` -> `{}`)",
                        kube_user,
                        alias
                    );
                alias
            }
            None => Cow::Borrowed(kube_user),
        }
    }

    pub fn get_kube_context_alias<'a>(
        config: &'a KubernetesConfig,
        kube_ctx: &'a str,
    ) -> Cow<'a, str> {
        let context_alias = get_alias(&config.context_aliases, kube_ctx);
        match context_alias {
            Some(alias) => {
                log::warn!(
                        "Usage of 'context_aliases' is deprecated and will be removed in 2.0; Use 'contexts' with 'context_alias' instead. (`{}` -> `{}`)",
                        kube_ctx,
                        alias
                    );
                alias
            }
            None => Cow::Borrowed(kube_ctx),
        }
    }

    fn get_alias<'a>(
        aliases: &'a HashMap<String, &'a str>,
        alias_candidate: &'a str,
    ) -> Option<Cow<'a, str>> {
        if let Some(val) = aliases.get(alias_candidate) {
            return Some(Cow::Borrowed(val));
        }

        return aliases.iter().find_map(|(k, v)| {
            let re = regex::Regex::new(&format!("^{k}$")).ok()?;
            let replaced = re.replace(alias_candidate, *v);
            match replaced {
                Cow::Owned(replaced) => Some(Cow::Owned(replaced)),
                _ => None,
            }
        });
    }
}

#[cfg(test)]
mod tests {
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
    fn test_none_when_no_detected_files_or_folders() -> io::Result<()> {
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
            })
            .collect();

        assert_eq!(None, actual);

        dir.close()
    }

    #[test]
    fn test_with_detected_files_and_folder() -> io::Result<()> {
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

        let expected = Some(format!(
            "{} in ",
            Color::Cyan.bold().paint("☸ test_context")
        ));

        assert_eq!(expected, actual_file);
        assert_eq!(expected, actual_ext);
        assert_eq!(expected, actual_dir);

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
    fn test_multiple_config_files_with_ns() -> io::Result<()> {
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

        // And tes with context and namespace first
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
                context_pattern = "test"
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
                context_pattern = "test"
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
}
