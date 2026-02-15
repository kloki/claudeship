#![allow(dead_code)]

use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Model {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub current_dir: String,
    pub project_dir: String,
}

#[derive(Deserialize)]
pub struct OutputStyle {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Cost {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_api_duration_ms: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
}

#[derive(Deserialize)]
pub struct CurrentUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub context_window_size: u64,
    pub used_percentage: Option<f64>,
    pub remaining_percentage: Option<f64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Deserialize)]
pub struct Vim {
    pub mode: String,
}

#[derive(Deserialize)]
pub struct Agent {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Input {
    pub cwd: String,
    pub session_id: String,
    pub transcript_path: String,
    pub version: String,
    pub exceeds_200k_tokens: bool,
    pub model: Model,
    pub workspace: Workspace,
    pub output_style: OutputStyle,
    pub cost: Cost,
    pub context_window: ContextWindow,
    pub vim: Option<Vim>,
    pub agent: Option<Agent>,
}

impl Input {
    pub fn from_stdin() -> Result<Self, Box<dyn std::error::Error>> {
        let mut raw = String::new();
        std::io::stdin().read_to_string(&mut raw)?;
        Ok(serde_json::from_str(&raw)?)
    }
}
