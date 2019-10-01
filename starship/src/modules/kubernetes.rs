use ansi_term::Color;
use dirs;
use yaml_rust::YamlLoader;

use std::env;
use std::path;

use super::{Context, Module};
use crate::utils;

const KUBE_CHAR: &str = "☸ ";

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

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let filename = match env::var("KUBECONFIG") {
        Ok(path) => path::PathBuf::from(path),
        Err(_) => dirs::home_dir()?.join(".kube").join("config"),
    };

    let contents = utils::read_file(filename).ok()?;

    match get_kube_context(&contents) {
        Some(kube_cfg) => {
            let (kube_ctx, kube_ns) = kube_cfg;

            let mut module = context.new_module("kubernetes");

            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Cyan.bold());
            module.set_style(module_style);
            module.get_prefix().set_value("on ");

            module.new_segment("symbol", KUBE_CHAR);
            module.new_segment("context", &kube_ctx);
            if kube_ns != "" {
                module.new_segment("namespace", &format!(" ({})", kube_ns));
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
