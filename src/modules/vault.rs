use super::{Context, Module, ModuleConfig};
use crate::configs::vault::VaultConfig;
use crate::formatter::StringFormatter;
use chrono::{DateTime, FixedOffset, Local};
use serde_json::Value;

/// Build the Vault module if enabled and valid.
pub fn module<'a>(context: &'a Context<'a>) -> Option<Module<'a>> {
    let module = context.new_module("vault");
    let config: VaultConfig = VaultConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let output = vault_lookup(context)?;
    parse_vault_output(&String::from_utf8_lossy(&output), &config, context)
}

/// Run `vault token lookup` and return JSON output.
fn vault_lookup<'a>(context: &'a Context<'a>) -> Option<Vec<u8>> {
    use std::env;
    use std::fs;

    if env::var("VAULT_TOKEN").is_err() {
        let Ok(home) = env::var("HOME") else {
            return None;
        };
        let Ok(_) = fs::read_to_string(format!("{home}/.vault-token")) else {
            return None;
        };
    }
    context
        .exec_cmd("vault", &["token", "lookup", "--format=json"])
        .map(|out| out.stdout.into_bytes())
}

/// Parse Vault JSON output and build the module if token is near expiry.
fn parse_vault_output<'a>(
    output: &str,
    config: &VaultConfig,
    context: &'a Context<'a>,
) -> Option<Module<'a>> {
    let json: Value = serde_json::from_str(output).ok()?;
    let expire_time = json["data"]["expire_time"].as_str()?;

    let Ok(expire_dt) = DateTime::parse_from_rfc3339(expire_time) else {
        return None;
    };

    let now = Local::now();
    let expire_local = expire_dt.with_timezone(&Local);
    let duration = expire_local.signed_duration_since(now);

    if duration.num_days() <= config.show_within_days as i64 {
        let Ok(dt) = expire_time.parse::<DateTime<FixedOffset>>() else {
            return None;
        };
        let expire_date_str = dt.format("%Y-%m-%d").to_string();
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
                    "expire_time" => Some(Ok(expire_time)),
                    "expire_date" => Some(Ok(&expire_date_str)),
                    _ => None,
                })
                .parse(None, Some(context))
        });

        let mut module = context.new_module("vault");
        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(error) => {
                log::warn!("Error in module `vault`: \n{error}");
                return None;
            }
        });
        return Some(module);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use chrono::{Duration, Local};
    use serde_json::json;

    /// Test that the Vault module renders correctly when expire_time is within allowed days.
    #[test]
    fn test_module_full_render_success() {
        let expire_time = (Local::now() + Duration::days(1)).to_rfc3339();
        let fake_json = json!({
            "data": {
                "expire_time": expire_time
            }
        })
        .to_string();

        let output = ModuleRenderer::new("vault")
            .env("VAULT_TOKEN", "present")
            .cmd(
                "vault token lookup --format=json",
                Some(CommandOutput {
                    stdout: fake_json,
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! {
                [vault]
                disabled = false
                show_within_days = 7
            })
            .collect();

        let expected_date = &expire_time[..10];
        let output_str = output.expect("Module should have rendered an output");

        assert!(output_str.contains("🔒"));
        assert!(output_str.contains(expected_date));
    }

    /// Test that the Vault module does not render when disabled in configuration.
    #[test]
    fn test_module_disabled() {
        let output = ModuleRenderer::new("vault")
            .config(toml::toml! {
                [vault]
                disabled = true
            })
            .collect();

        assert!(output.is_none());
    }

    /// Test that the Vault module does not render when the command fails (no output).
    #[test]
    fn test_module_command_failure() {
        // Mocking a command failure by passing None to .cmd()
        let output = ModuleRenderer::new("vault")
            .env("VAULT_TOKEN", "present")
            .cmd("vault token lookup --format=json", None)
            .collect();

        assert!(output.is_none());
    }

    /// Test that the Vault module does not render when expire_time is beyond allowed days.
    #[test]
    fn test_module_beyond_expiry_days() {
        let expire_time = (Local::now() + Duration::days(10)).to_rfc3339();
        let fake_json = json!({
            "data": { "expire_time": expire_time }
        })
        .to_string();

        let output = ModuleRenderer::new("vault")
            .env("VAULT_TOKEN", "present")
            .cmd(
                "vault token lookup --format=json",
                Some(CommandOutput {
                    stdout: fake_json,
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! {
                [vault]
                show_within_days = 7
            })
            .collect();

        assert!(output.is_none());
    }

    /// Test that the Vault module does not render when the command returns invalid JSON.
    #[test]
    fn test_module_invalid_json() {
        let output = ModuleRenderer::new("vault")
            .env("VAULT_TOKEN", "present")
            .cmd(
                "vault token lookup --format=json",
                Some(CommandOutput {
                    stdout: "invalid json".to_string(),
                    stderr: String::new(),
                }),
            )
            .collect();

        assert!(output.is_none());
    }
}
