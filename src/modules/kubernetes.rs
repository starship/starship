use yaml_rust::YamlLoader;

use std::env;
use std::path;

use super::{Context, Module, RootModuleConfig};

use crate::configs::kubernetes::KubernetesConfig;
use crate::formatter::StringFormatter;
use crate::utils;

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

fn get_kube_ns(filename: path::PathBuf, current_ctx: String) -> Option<String> {
    let contents = utils::read_file(filename).ok()?;

    let yaml_docs = YamlLoader::load_from_str(&contents).ok()?;
    if yaml_docs.is_empty() {
        return None;
    }
    let conf = &yaml_docs[0];

    let ns = conf["contexts"].as_vec().and_then(|contexts| {
        contexts
            .iter()
            .filter_map(|ctx| Some((ctx, ctx["name"].as_str()?)))
            .find(|(_, name)| *name == current_ctx)
            .and_then(|(ctx, _)| ctx["context"]["namespace"].as_str())
    })?;

    if ns.is_empty() {
        return None;
    }
    Some(ns.to_owned())
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("kubernetes");
    let config: KubernetesConfig = KubernetesConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let default_config_file = context.get_home()?.join(".kube").join("config");

    let kube_cfg = context
        .get_env("KUBECONFIG")
        .unwrap_or(default_config_file.to_str()?.to_string());

    let kube_ctx = env::split_paths(&kube_cfg).find_map(get_kube_context)?;

    let kube_ns =
        env::split_paths(&kube_cfg).find_map(|filename| get_kube_ns(filename, kube_ctx.clone()));

    // Parse config under `display`.
    // Select the first style that matches the context_pattern
    let display_style = config
        .display
        .iter()
        .find(|display| {
            regex::Regex::new(display.context_pattern)
                .map(|re| re.is_match(&kube_ctx))
                .unwrap_or(false)
        })
        .map(|display| display.style)
        .unwrap_or(config.style);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "context" => match config.context_aliases.get(&kube_ctx) {
                    None => Some(Ok(kube_ctx.as_str())),
                    Some(&alias) => Some(Ok(alias)),
                },
                "namespace" => kube_ns.as_ref().map(|s| Ok(s.as_str())),
                _ => None,
            })
            .parse(None)
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

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::env;
    use std::fs::File;
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
    fn test_ctx_alias() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let filename = dir.path().join("config");

        let mut file = File::create(&filename)?;
        file.write_all(
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
        file.sync_all()?;

        let actual = ModuleRenderer::new("kubernetes")
            .path(dir.path())
            .env("KUBECONFIG", filename.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                [kubernetes.context_aliases]
                "test_context" = "test_alias"
            })
            .collect();

        let expected = Some(format!("{} in ", Color::Cyan.bold().paint("☸ test_alias")));
        assert_eq!(expected, actual);

        dir.close()
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
                env::join_paths([&filename_cc, &filename_ctx].iter())
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
                env::join_paths([&filename_ctx, &filename_cc].iter())
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
    fn config_context_and_ns() -> std::io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let config_path = tempdir.path().join("config");
        let mut file = File::create(&config_path)?;

        let input = r#"
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
"#;
        file.write_all(input.as_bytes())?;
        file.sync_all()?;

        // verify the default style is applied
        let actual = ModuleRenderer::new("kubernetes")
            .env("KUBECONFIG", config_path.to_string_lossy().as_ref())
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

        // verify the configured style is applied
        let actual = ModuleRenderer::new("kubernetes")
            .env("KUBECONFIG", config_path.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"
            })
            .collect();
        let expected = Some(format!(
            "{} in ",
            Color::Red.bold().paint("☸ test_context (test_namespace)")
        ));

        assert_eq!(expected, actual);

        // verify that [[Kubernetes.display]] overrides the configured style
        let actual = ModuleRenderer::new("kubernetes")
            .env("KUBECONFIG", config_path.to_string_lossy().as_ref())
            .config(toml::toml! {
                [kubernetes]
                disabled = false
                style = "bold red"

                [[kubernetes.display]]
                context_pattern = "test"
                style = "bold green"
            })
            .collect();
        let expected = Some(format!(
            "{} in ",
            Color::Green.bold().paint("☸ test_context (test_namespace)")
        ));
        assert_eq!(expected, actual);

        tempdir.close()
    }
}
