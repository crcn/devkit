//! Shell script provider
//!
//! Discovers executable shell scripts in common directories

use anyhow::Result;
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::context::AppContext;
use crate::discovery::{Category, CommandProvider, CommandScope, DiscoveredCommand};

pub struct ScriptProvider;

impl ScriptProvider {
    pub fn new() -> Self {
        Self
    }

    /// Common directories to search for scripts
    fn script_directories() -> Vec<&'static str> {
        vec!["bin", "scripts", ".dev/scripts", "tools"]
    }

    /// Check if a file is executable
    fn is_executable(path: &Path) -> bool {
        #[cfg(unix)]
        {
            // On Unix, check permission bits
            if let Ok(metadata) = fs::metadata(path) {
                let permissions = metadata.permissions();
                // Check if any execute bit is set (owner, group, or other)
                return permissions.mode() & 0o111 != 0;
            }
            false
        }

        #[cfg(windows)]
        {
            // On Windows, check file extension
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(
                    ext.as_str(),
                    "exe" | "bat" | "cmd" | "ps1" | "sh" | "bash"
                )
            } else {
                false
            }
        }
    }

    /// Parse shebang to determine script type
    fn parse_shebang(path: &Path) -> Option<String> {
        if let Ok(content) = fs::read_to_string(path) {
            if let Some(first_line) = content.lines().next() {
                if first_line.starts_with("#!") {
                    return Some(first_line[2..].trim().to_string());
                }
            }
        }
        None
    }

    /// Extract description from script comments
    fn extract_description(path: &Path) -> Option<String> {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines().take(20) {
                let trimmed = line.trim();
                // Look for comment lines that might be descriptions
                if trimmed.starts_with("# Description:") {
                    return Some(trimmed.trim_start_matches("# Description:").trim().to_string());
                } else if trimmed.starts_with("# ") && !trimmed.starts_with("#!") {
                    let comment = trimmed.trim_start_matches("# ").trim();
                    // Skip common boilerplate comments
                    if !comment.is_empty()
                        && !comment.starts_with("!")
                        && !comment.contains("bin/")
                        && !comment.contains("usr/")
                        && comment.len() > 10
                        && comment.len() < 100
                    {
                        return Some(comment.to_string());
                    }
                }
            }
        }
        None
    }

    /// Categorize script based on name
    fn categorize_script(name: &str) -> Category {
        match name {
            n if n.contains("build") || n.contains("compile") => Category::Build,
            n if n.contains("test") => Category::Test,
            n if n.contains("lint") || n.contains("check") || n.contains("format") => {
                Category::Quality
            }
            n if n.contains("deploy") || n.contains("release") || n.contains("publish") => {
                Category::Deploy
            }
            n if n.contains("dev") || n.contains("serve") || n.contains("watch") => Category::Dev,
            n if n.contains("setup") || n.contains("install") => Category::Other,
            _ => Category::Scripts,
        }
    }

    /// Discover scripts in a directory
    fn discover_in_directory(
        repo_root: &Path,
        dir: &str,
        gitignore: Option<&ignore::gitignore::Gitignore>,
    ) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();
        let dir_path = repo_root.join(dir);

        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(commands);
        }

        // Check if the directory itself is ignored
        if let Some(gi) = gitignore {
            let relative_path = dir_path.strip_prefix(repo_root).unwrap_or(&dir_path);
            if gi.matched(relative_path, true).is_ignore() {
                return Ok(commands);
            }
        }

        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Check if file is ignored by .gitignore
            if let Some(gi) = gitignore {
                let relative_path = path.strip_prefix(repo_root).unwrap_or(&path);
                if gi.matched(relative_path, false).is_ignore() {
                    continue;
                }
            }

            // Check if executable
            if !Self::is_executable(&path) {
                continue;
            }

            // Get script name
            let script_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("script")
                .to_string();

            // Skip hidden files
            if script_name.starts_with('.') {
                continue;
            }

            // Categorize
            let category = Self::categorize_script(&script_name);
            let emoji = category.emoji();

            // Extract description
            let description = Self::extract_description(&path)
                .unwrap_or_else(|| format!("Run {} script", script_name));

            // Determine interpreter from shebang
            let _shebang = Self::parse_shebang(&path);

            let id = format!("script.{}.{}", dir.replace('/', "_"), script_name);
            let label = format!("{} {}", emoji, script_name);
            let source = format!("{}/{}", dir, script_name);

            commands.push(
                DiscoveredCommand::new(id, label, category)
                    .description(description)
                    .source(source)
                    .scope(CommandScope::Global)
                    .handler({
                        let script_path = path.clone();
                        move |_ctx| {
                            crate::command::run_command(
                                &script_path.to_string_lossy(),
                                &vec![],
                                &script_path.parent().unwrap(),
                            )
                        }
                    }),
            );
        }

        Ok(commands)
    }
}

impl CommandProvider for ScriptProvider {
    fn name(&self) -> &'static str {
        "scripts"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Check if any script directories exist
        Self::script_directories()
            .iter()
            .any(|dir| ctx.repo.join(dir).exists())
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();

        // Load .gitignore if it exists
        let gitignore_path = ctx.repo.join(".gitignore");
        let gitignore = if gitignore_path.exists() {
            let mut builder = ignore::gitignore::GitignoreBuilder::new(&ctx.repo);
            let _ = builder.add(gitignore_path);
            builder.build().ok()
        } else {
            None
        };

        for dir in Self::script_directories() {
            commands.extend(Self::discover_in_directory(&ctx.repo, dir, gitignore.as_ref())?);
        }

        Ok(commands)
    }
}

impl Default for ScriptProvider {
    fn default() -> Self {
        Self::new()
    }
}
