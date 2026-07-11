/// Claude Code session data structures for statusline integration
use serde::{Deserialize, Deserializer};

/// Deserialize a field, treating explicit `null` the same as a missing field.
/// `#[serde(default)]` alone only covers missing fields, not explicit `null`.
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// Claude Code session data
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ClaudeCodeData {
    pub cwd: Option<String>,
    pub model: ModelInfo,
    pub context_window: ContextWindow,
    pub cost: Option<CostInfo>,
    pub workspace: Option<Workspace>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ContextWindow {
    pub context_window_size: u64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub used_percentage: f32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub current_usage: CurrentUsage,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CurrentUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CostInfo {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_api_duration_ms: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Workspace {
    pub current_dir: String,
    pub project_dir: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializes_session_start_payload_with_null_context_window_fields() {
        let payload = r#"{
            "model": {
                "id": "claude-opus-4-7[1m]",
                "display_name": "Opus 4.7 (1M context)"
            },
            "context_window": {
                "total_input_tokens": 0,
                "total_output_tokens": 0,
                "context_window_size": 1000000,
                "current_usage": null,
                "used_percentage": null,
                "remaining_percentage": null
            }
        }"#;

        let data: ClaudeCodeData =
            serde_json::from_str(payload).expect("session-start payload must deserialize");

        assert_eq!(data.model.display_name, "Opus 4.7 (1M context)");
        assert_eq!(data.context_window.context_window_size, 1_000_000);
        assert_eq!(data.context_window.used_percentage, 0.0);
        assert_eq!(data.context_window.current_usage.input_tokens, 0);
    }

    #[test]
    fn test_deserializes_fully_populated_payload() {
        let payload = r#"{
            "model": {
                "id": "claude-opus-4-7[1m]",
                "display_name": "Opus 4.7 (1M context)"
            },
            "context_window": {
                "total_input_tokens": 1234,
                "total_output_tokens": 567,
                "context_window_size": 1000000,
                "used_percentage": 12.5,
                "current_usage": {
                    "input_tokens": 100,
                    "output_tokens": 50,
                    "cache_creation_input_tokens": 10,
                    "cache_read_input_tokens": 20
                }
            }
        }"#;

        let data: ClaudeCodeData =
            serde_json::from_str(payload).expect("populated payload must deserialize");

        assert_eq!(data.context_window.total_input_tokens, 1234);
        assert!((data.context_window.used_percentage - 12.5).abs() < f32::EPSILON);
        assert_eq!(
            data.context_window.current_usage.cache_read_input_tokens,
            20
        );
    }

    #[test]
    fn test_missing_context_window_fields_default() {
        let payload = r#"{ "context_window": {} }"#;
        let data: ClaudeCodeData =
            serde_json::from_str(payload).expect("missing fields must default");
        assert_eq!(data.context_window.context_window_size, 0);
        assert_eq!(data.context_window.used_percentage, 0.0);
        assert_eq!(data.context_window.current_usage.input_tokens, 0);
    }

    #[test]
    fn test_wrong_type_still_errors() {
        let payload = r#"{
            "context_window": {
                "used_percentage": "not a number"
            }
        }"#;

        let err = serde_json::from_str::<ClaudeCodeData>(payload)
            .expect_err("wrong-type field must not be silently defaulted");
        let msg = err.to_string();
        assert!(
            msg.contains("invalid type") && msg.contains("f32"),
            "error should be a serde type error, got: {msg}"
        );
    }
}
