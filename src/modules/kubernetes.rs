use dirs;
use yaml_rust::YamlLoader;

use std::env;
use std::path;

use super::{Context, Module, RootModuleConfig};

use crate::configs::kubernetes::KubernetesConfig;
use crate::utils;

const KUBERNETES_PREFIX: &str = "on ";

fn get_kube_context(contents: &str) -> Option<(String, String)> {
    let yaml_docs = YamlLoader::load_from_str(&contents).ok()?;
    if yaml_docs.is_empty() {
        return None;
    }
    let conf = &yaml_docs[0];

    let current_ctx = conf["current-context"].as_str()?;

    if current_ctx.is_empty() {
        return None;
    }

    let ns = conf["contexts"]
        .as_vec()
        .and_then(|contexts| {
            contexts
                .iter()
                .filter_map(|ctx| Some((ctx, ctx["name"].as_str()?)))
                .find(|(_, name)| *name == current_ctx)
                .and_then(|(ctx, _)| ctx["context"]["namespace"].as_str())
        })
        .unwrap_or("");

    Some((current_ctx.to_string(), ns.to_string()))
}

fn parse_kubectl_file(filename: &path::PathBuf) -> Option<(String, String)> {
    let contents = utils::read_file(filename).ok()?;
    get_kube_context(&contents)
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let kube_cfg = match env::var("KUBECONFIG") {
        Ok(paths) => env::split_paths(&paths)
            .filter_map(|filename| parse_kubectl_file(&filename))
            .nth(0),
        Err(_) => {
            let filename = dirs::home_dir()?.join(".kube").join("config");
            parse_kubectl_file(&filename)
        }
    };

    match kube_cfg {
        Some(kube_cfg) => {
            let (kube_ctx, kube_ns) = kube_cfg;

            let mut module = context.new_module("kubernetes");
            let config: KubernetesConfig = KubernetesConfig::try_load(module.config);
            if config.disabled {
                return None;
            };

            module.set_style(config.style);
            module.get_prefix().set_value(KUBERNETES_PREFIX);

            module.create_segment("symbol", &config.symbol);
            module.create_segment("context", &config.context.with_value(&kube_ctx));
            if kube_ns != "" {
                module.create_segment(
                    "namespace",
                    &config.namespace.with_value(&format!(" ({})", kube_ns)),
                );
            }
            Some(module)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_config() {
        let input = "";
        let result = get_kube_context(&input);
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_no_config() {
        let input = r#"
apiVersion: v1
clusters: []
contexts: []
current-context: ""
kind: Config
preferences: {}
users: []
"#;
        let result = get_kube_context(&input);
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_only_context() {
        let input = r#"
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
"#;
        let result = get_kube_context(&input);
        let expected = Some(("test_context".to_string(), "".to_string()));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_context_and_ns() {
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
        let result = get_kube_context(&input);
        let expected = Some(("test_context".to_string(), "test_namespace".to_string()));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_multiple_contexts() {
        let input = r#"
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
"#;
        let result = get_kube_context(&input);
        let expected = Some(("test_context".to_string(), "test_namespace".to_string()));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_broken_config() {
        let input = r#"
---
dummy_string
"#;
        let result = get_kube_context(&input);
        let expected = None;

        assert_eq!(result, expected);
    }
}
