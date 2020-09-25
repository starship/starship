use yaml_rust::YamlLoader;

use std::env;
use std::path;

use super::{Context, Module, RootModuleConfig};

use crate::configs::kubernetes::KubernetesConfig;
use crate::formatter::StringFormatter;
use crate::utils;

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
    let kube_cfg = match context.get_env("KUBECONFIG") {
        Some(paths) => env::split_paths(&paths)
            .filter_map(|filename| parse_kubectl_file(&filename))
            .next(),
        None => {
            let filename = dirs_next::home_dir()?.join(".kube").join("config");
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
                        "context" => match config.context_aliases.get(&kube_ctx) {
                            None => Some(Ok(kube_ctx.as_str())),
                            Some(&alias) => Some(Ok(alias)),
                        },
                        _ => None,
                    })
                    .map(|variable| match variable {
                        "namespace" => {
                            if kube_ns != "" {
                                Some(Ok(kube_ns.as_str()))
                            } else {
                                None
                            }
                        }
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
