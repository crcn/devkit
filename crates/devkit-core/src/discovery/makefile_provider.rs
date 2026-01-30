//! Makefile command provider
//!
//! Discovers Make targets from Makefiles

use anyhow::Result;
use std::fs;

use crate::context::AppContext;
use crate::discovery::{Category, CommandProvider, CommandScope, DiscoveredCommand};
use crate::utils::cmd_exists;

pub struct MakefileProvider;

impl MakefileProvider {
    pub fn new() -> Self {
        Self
    }

    fn parse_makefile_targets(content: &str) -> Vec<(String, Option<String>)> {
        let mut targets = Vec::new();
        let mut current_comment: Option<String> = None;

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                current_comment = None;
                continue;
            }

            // Skip recipe lines (indented lines - these are commands, not targets)
            // Recipe lines start with whitespace (tab or spaces)
            if !line.is_empty() && (line.starts_with('\t') || line.starts_with(' ')) {
                continue;
            }

            // Capture comments
            if trimmed.starts_with('#') {
                let comment = trimmed.trim_start_matches('#').trim();
                if !comment.is_empty() {
                    current_comment = Some(comment.to_string());
                }
                continue;
            }

            // Check if this is a target line
            if let Some(colon_pos) = trimmed.find(':') {
                let target_part = &trimmed[..colon_pos];

                // Skip internal targets (starting with . or _)
                if target_part.starts_with('.') || target_part.starts_with('_') {
                    current_comment = None;
                    continue;
                }

                // Skip if it contains $( or ${ (variable references)
                if target_part.contains("$(") || target_part.contains("${") {
                    current_comment = None;
                    continue;
                }

                // Extract target name (first word before colon)
                if let Some(target_name) = target_part.split_whitespace().next() {
                    if !target_name.is_empty() {
                        targets.push((target_name.to_string(), current_comment.take()));
                    }
                }
            } else {
                // Not a target line, reset comment
                current_comment = None;
            }
        }

        targets
    }

    fn categorize_target(name: &str) -> Category {
        match name {
            n if n.contains("build") || n.contains("compile") => Category::Build,
            n if n.contains("test") => Category::Test,
            n if n.contains("lint") || n.contains("check") => Category::Quality,
            n if n.contains("deploy") || n.contains("release") || n.contains("publish") => {
                Category::Deploy
            }
            n if n.contains("dev") || n.contains("serve") || n.contains("watch") => Category::Dev,
            n if n.contains("clean") || n.contains("install") || n.contains("setup") => {
                Category::Other
            }
            _ => Category::Scripts,
        }
    }
}

impl CommandProvider for MakefileProvider {
    fn name(&self) -> &'static str {
        "makefile"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        cmd_exists("make")
            && (ctx.repo.join("Makefile").exists()
                || ctx.repo.join("makefile").exists()
                || ctx.repo.join("GNUmakefile").exists())
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();

        // Find Makefile
        let makefile_path = if ctx.repo.join("Makefile").exists() {
            ctx.repo.join("Makefile")
        } else if ctx.repo.join("makefile").exists() {
            ctx.repo.join("makefile")
        } else if ctx.repo.join("GNUmakefile").exists() {
            ctx.repo.join("GNUmakefile")
        } else {
            return Ok(commands);
        };

        let content = fs::read_to_string(&makefile_path)?;
        let targets = Self::parse_makefile_targets(&content);

        for (target_name, comment) in targets {
            let category = Self::categorize_target(&target_name);
            let emoji = category.emoji();

            let description = comment
                .clone()
                .unwrap_or_else(|| format!("Run make target: {}", target_name));

            commands.push(
                DiscoveredCommand::new(
                    format!("make.{}", target_name),
                    format!("{} {}", emoji, target_name),
                    category,
                )
                .description(description)
                .source("Makefile")
                .scope(CommandScope::Global)
                .handler({
                    let target = target_name.clone();
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command("make", &vec![target.clone()], &repo)
                    }
                }),
            );
        }

        Ok(commands)
    }
}

impl Default for MakefileProvider {
    fn default() -> Self {
        Self::new()
    }
}
