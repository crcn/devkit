//! Command history tracking
//!
//! Tracks recently executed commands per project for quick access

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const MAX_HISTORY: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandHistoryEntry {
    /// Command ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Timestamp (seconds since epoch)
    pub timestamp: u64,
    /// How many times this command has been run
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandHistory {
    /// Recent commands (most recent first)
    pub recent: Vec<CommandHistoryEntry>,
}

impl CommandHistory {
    /// Create a new empty history
    pub fn new() -> Self {
        Self {
            recent: Vec::new(),
        }
    }

    /// Load history from disk
    pub fn load(repo_root: &Path) -> Result<Self> {
        let history_path = Self::history_file_path(repo_root);

        if !history_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&history_path)
            .context("Failed to read command history")?;

        let history: CommandHistory = serde_json::from_str(&content)
            .context("Failed to parse command history")?;

        Ok(history)
    }

    /// Save history to disk
    pub fn save(&self, repo_root: &Path) -> Result<()> {
        let history_path = Self::history_file_path(repo_root);

        // Ensure .dev directory exists
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize command history")?;

        fs::write(&history_path, content)
            .context("Failed to write command history")?;

        Ok(())
    }

    /// Record a command execution
    pub fn record(&mut self, command_id: &str, label: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check if command already exists in history
        if let Some(entry) = self.recent.iter_mut().find(|e| e.id == command_id) {
            entry.timestamp = now;
            entry.count += 1;
            entry.label = label.to_string();
        } else {
            // Add new entry
            self.recent.insert(
                0,
                CommandHistoryEntry {
                    id: command_id.to_string(),
                    label: label.to_string(),
                    timestamp: now,
                    count: 1,
                },
            );
        }

        // Sort by timestamp (most recent first)
        self.recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Keep only MAX_HISTORY entries
        self.recent.truncate(MAX_HISTORY);
    }

    /// Get recent commands
    pub fn recent_commands(&self) -> &[CommandHistoryEntry] {
        &self.recent
    }

    /// Get most frequently used commands
    pub fn frequent_commands(&self) -> Vec<&CommandHistoryEntry> {
        let mut sorted = self.recent.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.count.cmp(&a.count));
        sorted
    }

    fn history_file_path(repo_root: &Path) -> PathBuf {
        repo_root.join(".dev").join("history.json")
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}
