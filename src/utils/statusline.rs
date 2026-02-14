/// Claude Code session data structures for statusline integration
use serde::Deserialize;

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
    /// Usage of the most recent API call
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
