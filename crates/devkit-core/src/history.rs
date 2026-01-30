//! Command history tracking

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const HISTORY_FILE: &str = "history.json";
const MAX_HISTORY_SIZE: usize = 100;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: u64,
    pub success: bool,
}

/// Load command history from cache
pub fn load_history() -> Result<Vec<HistoryEntry>> {
    let path = history_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)?;
    let history: Vec<HistoryEntry> = serde_json::from_str(&contents)?;

    Ok(history)
}

/// Save command history to cache
pub fn save_history(history: &[HistoryEntry]) -> Result<()> {
    let path = history_path()?;

    // Keep only last MAX_HISTORY_SIZE entries
    let trimmed: Vec<_> = history
        .iter()
        .rev()
        .take(MAX_HISTORY_SIZE)
        .rev()
        .cloned()
        .collect();

    let contents = serde_json::to_string_pretty(&trimmed)?;
    fs::write(&path, contents)?;

    Ok(())
}

/// Add a command to history
pub fn add_to_history(command: String, success: bool) -> Result<()> {
    let mut history = load_history()?;

    history.push(HistoryEntry {
        command,
        timestamp: current_timestamp(),
        success,
    });

    save_history(&history)?;

    Ok(())
}

/// Get last command from history
pub fn last_command() -> Result<Option<String>> {
    let history = load_history()?;
    Ok(history.last().map(|e| e.command.clone()))
}

/// Search history by pattern
pub fn search_history(pattern: &str) -> Result<Vec<HistoryEntry>> {
    let history = load_history()?;

    Ok(history
        .into_iter()
        .filter(|e| e.command.contains(pattern))
        .collect())
}

fn history_path() -> Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Failed to get cache directory"))?;

    let devkit_cache = cache_dir.join("devkit");
    fs::create_dir_all(&devkit_cache)?;

    Ok(devkit_cache.join(HISTORY_FILE))
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
