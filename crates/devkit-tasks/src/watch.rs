//! File watching and auto-rerun functionality

use anyhow::{Context, Result};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

/// Watch configuration
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// File patterns to watch
    pub patterns: Vec<String>,
    /// Debounce delay in milliseconds
    pub debounce_ms: u64,
    /// Clear terminal on rerun
    pub clear_terminal: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            patterns: vec!["**/*.rs".to_string(), "**/*.toml".to_string()],
            debounce_ms: 500,
            clear_terminal: true,
        }
    }
}

/// Watch a directory and execute a callback on file changes
pub fn watch_and_run<F>(path: &Path, config: &WatchConfig, mut callback: F) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    println!("👀 Watching for changes... (press Ctrl+C to stop)");
    println!();

    // Run once initially
    if config.clear_terminal {
        clear_terminal();
    }
    callback()?;

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                // Only react to modification events
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                        let _ = tx.send(());
                    }
                    _ => {}
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(config.debounce_ms)),
    )?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    // Debounce mechanism
    let debounce_duration = Duration::from_millis(config.debounce_ms);
    let mut last_run = std::time::Instant::now();

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(_) => {
                let now = std::time::Instant::now();
                if now.duration_since(last_run) >= debounce_duration {
                    if config.clear_terminal {
                        clear_terminal();
                    }

                    println!("🔄 Change detected, rerunning...");
                    println!();

                    if let Err(e) = callback() {
                        eprintln!("❌ Error: {:#}", e);
                    }

                    last_run = now;
                    println!();
                    println!("👀 Watching for changes...");
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // No events, continue watching
                continue;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert_eq!(config.debounce_ms, 500);
        assert!(config.clear_terminal);
    }
}
