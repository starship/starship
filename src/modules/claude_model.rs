use super::{Context, Module, ModuleConfig};
use crate::configs::claude_model::ClaudeModelConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_model");
    let config = ClaudeModelConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // Read Claude Code data from Context
    let claude_data = context.claude_code_data.as_ref()?;

    // Check if there's an alias for this model
    let model_display = config
        .model_aliases
        .get(&claude_data.model.id)
        .or_else(|| {
            config
                .model_aliases
                .get(claude_data.model.display_name.as_str())
        })
        .copied()
        .unwrap_or(&claude_data.model.display_name);

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
                "model" => Some(Ok(model_display)),
                "model_id" => Some(Ok(claude_data.model.id.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `claude_model`: {error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn test_without_data() {
        let actual = ModuleRenderer::new("claude_model").collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_with_model_alias() {
        let data = crate::context::ClaudeCodeData {
            cwd: None,
            model: crate::context::ModelInfo {
                id: "global.anthropic.claude-sonnet-4-5-20250929-v1:0".to_string(),
                display_name: "Claude Sonnet 4.5 (AWS Bedrock)".to_string(),
            },
            context_window: crate::context::ContextWindow::default(),
            cost: None,
            workspace: None,
        };

        let actual = ModuleRenderer::new("claude_model")
            .config(toml::toml! {
                [claude_model]
                symbol = ""
                [claude_model.model_aliases]
                "global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("Sonnet 4.5")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_without_alias_uses_display_name() {
        let data = crate::context::ClaudeCodeData {
            cwd: None,
            model: crate::context::ModelInfo {
                id: "claude-3-5-sonnet".to_string(),
                display_name: "Claude 3.5 Sonnet".to_string(),
            },
            context_window: crate::context::ContextWindow::default(),
            cost: None,
            workspace: None,
        };

        let actual = ModuleRenderer::new("claude_model")
            .config(toml::toml! {
                [claude_model]
                symbol = ""
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Blue.bold().paint("Claude 3.5 Sonnet")
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alias_by_display_name() {
        let data = crate::context::ClaudeCodeData {
            cwd: None,
            model: crate::context::ModelInfo {
                id: "some-long-vendor-specific-id".to_string(),
                display_name: "Claude Sonnet 4.5 (Vendor Proxy)".to_string(),
            },
            context_window: crate::context::ContextWindow::default(),
            cost: None,
            workspace: None,
        };

        let actual = ModuleRenderer::new("claude_model")
            .config(toml::toml! {
                [claude_model]
                symbol = ""
                [claude_model.model_aliases]
                "Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("Sonnet")));
        assert_eq!(actual, expected);
    }
}
