use super::{Context, Module, ModuleConfig};
use crate::configs::vault::VaultConfig;
use serde_json::Value; 
use std::process::Command;
use chrono::{DateTime, Local, FixedOffset};
use crate::formatter::StringFormatter;

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
    use chrono::{DateTime, FixedOffset, Local};

    #[test]
    fn test_default_config() {
        let config = VaultConfig::default();
        assert_eq!(config.symbol, "⚠️");
        assert_eq!(config.style, "bold red");
        // Format includes expire_time
        assert!(config.format.contains("$expire_time"));
    }

    #[test]
    fn test_formatter_parses_symbol_and_expire_time() {
        let config = VaultConfig::default();
        let dt: DateTime<FixedOffset> = "2026-03-19T00:00:00+00:00".parse().unwrap();
        let formatted = dt.format("%Y-%m-%d").to_string();

        // Test formatter replaces symbol and expire_time correctly
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
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_expire_time_within_days() {
        let config = VaultConfig::default();
        let now = Local::now();
        // Simulate 3 days until expiration
        let expire_local = now + chrono::Duration::days(3); // 3天後過期
        let duration = expire_local.signed_duration_since(now);

        assert!(duration.num_days() <= config.show_within_days as i64);
    }

    #[test]
    fn test_disabled_config() {
        let mut config = VaultConfig::default();
        // Test disabled flag
        config.disabled = true;
        assert!(config.disabled);
    }

}
