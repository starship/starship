use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

fn ex(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    let output = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    Ok(output)
}

/// Creates a module with the current k8s context/namespace
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let maybe_ctx: Option<String> = ex("kubectl", &["config", "current-context"])
        .map(|s| Some(s))
        .or::<String>(Ok(None))
        .unwrap();
    let maybe_namespace: Option<String> = ex(
        "kubectl",
        &[
            "config",
            "view",
            "--minify",
            "--output",
            "jsonpath={..namespace}",
        ],
    )
    .map(|s| Some(s))
    .or::<String>(Ok(None))
    .unwrap();

    match (maybe_ctx, maybe_namespace) {
        (Some(ctx), Some(namespace)) => {
            if ctx.trim() == "" {
                return None;
            }
            let mut module = context.new_module("kubernetes")?;
            module.set_style(Color::Cyan.bold());
            module.get_prefix().set_value("at ");

            module.new_segment("symbol", "☸ ");
            module.new_segment("context", &ctx.trim());

            if namespace.trim() != "" {
                module.new_segment("namespace", &format!(" ({})", namespace.trim()));
            }
            Some(module)
        }
        _ => None,
    }
}
