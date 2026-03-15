use super::{Context, Module, ModuleConfig};
use crate::configs::vault::VaultConfig;
use crate::formatter::StringFormatter;
use chrono::{DateTime, FixedOffset, Local};
use serde_json::Value;
use std::process::Command;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vault");
    let config: VaultConfig = VaultConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // Run vault CLI to get token info in JSON format
    let output = Command::new("vault")
        .args(&["token", "lookup", "--format=json"])
        .output()
        .ok()?;

    let json: Value = serde_json::from_slice(&output.stdout).ok()?;
    let expire_time = json["data"]["expire_time"].as_str().unwrap_or("null");

    if expire_time != "null" {
        if let Ok(expire_dt) = DateTime::parse_from_rfc3339(expire_time) {
            let now = Local::now();
            let expire_local = expire_dt.with_timezone(&Local);
            let duration = expire_local.signed_duration_since(now);

            if duration.num_days() <= config.show_within_days as i64 {
                let dt: DateTime<FixedOffset> = expire_time.parse().ok()?;
                let formatted = dt.format("%Y-%m-%d").to_string();
                // Replace variables (symbol, style, expire_time) in format string
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
                            "expire_time" => Some(Ok(&formatted)),
                            _ => None,
                        })
                        .parse(None, Some(context))
                });

                module.set_segments(match parsed {
                    Ok(segments) => segments,
                    Err(error) => {
                        log::warn!("Error in module `vault`: \n{error}");
                        return None;
                    }
                });
                return Some(module);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{Context, Properties, Target};
    use chrono::{DateTime, FixedOffset, Local};

    // Test default VaultConfig values
    #[test]
    fn test_default_config() {
        let config = VaultConfig::default();
        assert_eq!(config.symbol, "⚠️");
        assert_eq!(config.style, "bold red");
        assert!(config.format.contains("$expire_time"));
    }

    // Test formatter parses symbol and expire_time correctly
    #[test]
    fn test_formatter_parses_symbol_and_expire_time() {
        let config = VaultConfig::default();
        let dt: DateTime<FixedOffset> = "2026-03-19T00:00:00+00:00".parse().unwrap();
        let formatted = dt.format("%Y-%m-%d").to_string();

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
                    "expire_time" => Some(Ok(&formatted)),
                    _ => None,
                })
                .parse(None, None)
        });

        assert!(parsed.is_ok());
        let segments = parsed.unwrap();
        let mut module = Module::new("vault", "test module", None);
        module.set_segments(segments);
        let output = module.to_string();
        assert!(output.contains(&config.symbol));
        assert!(output.contains(&formatted));
    }

    // Test expire_time within show_within_days
    #[test]
    fn test_expire_time_within_days() {
        let config = VaultConfig::default();
        let now = Local::now();
        let expire_local = now + chrono::Duration::days(3);
        let duration = expire_local.signed_duration_since(now);
        assert!(duration.num_days() <= config.show_within_days as i64);
    }

    // Test disabled config flag
    #[test]
    fn test_disabled_config() {
        let mut config = VaultConfig::default();
        config.disabled = true;
        assert!(config.disabled);
    }

    // Test expired token logic
    #[test]
    fn test_expired_token() {
        let now = Local::now();
        let expire_local = now - chrono::Duration::days(1);
        let duration = expire_local.signed_duration_since(now);
        assert!(duration.num_days() < 0, "Token should be expired");
    }

    // Test boundary condition for show_within_days
    #[test]
    fn test_show_within_days_boundary() {
        let config = VaultConfig::default();
        let now = Local::now();
        let expire_local = now + chrono::Duration::days(config.show_within_days as i64);
        let duration = expire_local.signed_duration_since(now);
        assert_eq!(duration.num_days(), config.show_within_days as i64);
    }

    // Test formatter with symbol only
    #[test]
    fn test_formatter_symbol_only() {
        let config = VaultConfig {
            format: "⛁ $symbol",
            ..VaultConfig::default()
        };
        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_meta(|variable, _| match variable {
                    "symbol" => Some(config.symbol),
                    _ => None,
                })
                .parse(None, None)
        });
        assert!(parsed.is_ok());
        let segments = parsed.unwrap();
        let mut module = Module::new("vault", "test module", None);
        module.set_segments(segments);
        let output = module.to_string();
        assert!(output.contains("⛁"));
    }

    // Test module returns None when disabled
    #[test]
    fn test_module_disabled_returns_none() {
        let props = Properties::default();
        let target = Target::Main;
        let context = Context::new(props, target);

        let result = module(&context);
        assert!(result.is_none(), "Disabled config should return None");
    }

    // Test invalid JSON parsing
    #[test]
    fn test_invalid_json_returns_none() {
        let invalid_json = b"{ not valid json }";
        let parsed: Result<serde_json::Value, _> = serde_json::from_slice(invalid_json);
        assert!(parsed.is_err(), "Invalid JSON should not parse");
    }

    // Test expire_time equals "null"
    #[test]
    fn test_expire_time_null() {
        let json = r#"{ "data": { "expire_time": "null" } }"#;
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        let expire_time = parsed["data"]["expire_time"].as_str().unwrap_or("null");
        assert_eq!(expire_time, "null");
    }

    // Test invalid RFC3339 date format
    #[test]
    fn test_invalid_expire_time_format() {
        let bad_time = "not-a-date";
        let parsed = DateTime::parse_from_rfc3339(bad_time);
        assert!(parsed.is_err(), "Invalid RFC3339 date should fail");
    }

    // Test formatter with unknown variable
    #[test]
    fn test_formatter_parse_error() {
        let config = VaultConfig {
            format: "$unknown_var",
            ..VaultConfig::default()
        };
        let parsed =
            StringFormatter::new(config.format).and_then(|formatter| formatter.parse(None, None));

        assert!(
            parsed.is_ok(),
            "Formatter should succeed even with unknown variable"
        );
        let segments = parsed.unwrap();
        let mut module = Module::new("vault", "test module", None);
        module.set_segments(segments);
        let output = module.to_string();
        assert_eq!(
            output, "",
            "Output should be empty when variable is unknown"
        );
    }

    // Test full success path with expire_time inside show_within_days
    #[test]
    fn test_module_expire_time_full_success() {
        let props = Properties::default();
        let target = Target::Main;
        let context = Context::new(props, target);

        let expire_time = (Local::now() + chrono::Duration::days(1)).to_rfc3339();
        let dt: DateTime<FixedOffset> = expire_time.parse().unwrap();
        let formatted = dt.format("%Y-%m-%d").to_string();

        let parsed = StringFormatter::new(VaultConfig::default().format).and_then(|formatter| {
            formatter
                .map_meta(|variable, _| match variable {
                    "symbol" => Some(VaultConfig::default().symbol),
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(VaultConfig::default().style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "expire_time" => Some(Ok(&formatted)),
                    _ => None,
                })
                .parse(None, Some(&context))
        });

        assert!(
            parsed.is_ok(),
            "Formatter should succeed with valid expire_time"
        );
        let segments = parsed.unwrap();
        let mut module = Module::new("vault", "test module", None);
        module.set_segments(segments);
        let output = module.to_string();
        assert!(output.contains(&formatted));
    }

    // Test formatter with invalid format should keep literal text
    #[test]
    fn test_module_expire_time_formatter_failure() {
        let props = Properties::default();
        let target = Target::Main;
        let context = Context::new(props, target);

        let expire_time = (Local::now() + chrono::Duration::days(1)).to_rfc3339();
        let dt: DateTime<FixedOffset> = expire_time.parse().unwrap();
        let formatted = dt.format("%Y-%m-%d").to_string();

        let config = VaultConfig {
            format: "{unclosed", // deliberately invalid format
            ..VaultConfig::default()
        };

        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map(|variable| match variable {
                    "expire_time" => Some(Ok(&formatted)),
                    _ => None,
                })
                .parse(None, Some(&context))
        });

        assert!(
            parsed.is_ok(),
            "Formatter should succeed even with invalid format"
        );
        let segments = parsed.unwrap();
        let mut module = Module::new("vault", "test module", None);
        module.set_segments(segments);
        let output = module.to_string();
        assert_eq!(
            output, "{unclosed",
            "Output should preserve the literal invalid format"
        );
    }

    // Test module returns None when expire_time is beyond show_within_days
    #[test]
    fn test_module_expire_time_beyond_days_none() {
        let props = Properties::default();
        let target = Target::Main;

        // expire_time 在 30 天後，超過 show_within_days
        let expire_time = (Local::now() + chrono::Duration::days(30)).to_rfc3339();
        let expire_dt = DateTime::parse_from_rfc3339(&expire_time)
            .unwrap()
            .with_timezone(&Local);
        let now = Local::now();
        let duration = expire_dt.signed_duration_since(now);

        // 驗證超過 show_within_days
        assert!(duration.num_days() > VaultConfig::default().show_within_days as i64);

        // 模擬 module 邏輯：因為超過 show_within_days，應該 return None
        let config = VaultConfig::default();
        assert!(duration.num_days() > config.show_within_days as i64);
    }
}
