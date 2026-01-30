//! NPM/Node.js command provider
//!
//! Discovers package.json scripts in Node.js projects and workspaces

use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::context::AppContext;
use crate::discovery::{Category, CommandProvider, CommandScope, DiscoveredCommand};
use crate::utils::cmd_exists;

pub struct NpmProvider;

impl NpmProvider {
    pub fn new() -> Self {
        Self
    }

    fn detect_package_manager(repo_root: &Path) -> &'static str {
        if repo_root.join("pnpm-lock.yaml").exists() || repo_root.join("pnpm-workspace.yaml").exists() {
            "pnpm"
        } else if repo_root.join("yarn.lock").exists() {
            "yarn"
        } else {
            "npm"
        }
    }

    fn categorize_script(name: &str) -> Category {
        match name {
            n if n.contains("build") => Category::Build,
            n if n.contains("test") => Category::Test,
            n if n.contains("lint") || n.contains("eslint") => Category::Quality,
            n if n.contains("format") || n.contains("prettier") => Category::Quality,
            n if n.contains("typecheck") || n.contains("tsc") => Category::Quality,
            n if n.contains("dev") || n.contains("start") || n.contains("serve") => Category::Dev,
            n if n.contains("deploy") || n.contains("release") || n.contains("publish") => {
                Category::Deploy
            }
            _ => Category::Scripts,
        }
    }

    fn discover_package_scripts(
        package_path: &Path,
        package_manager: &str,
        scope: CommandScope,
    ) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();

        let package_json_path = package_path.join("package.json");
        if !package_json_path.exists() {
            return Ok(commands);
        }

        let content = fs::read_to_string(&package_json_path)?;
        let package_json: Value = serde_json::from_str(&content)?;

        let package_name = package_json["name"]
            .as_str()
            .unwrap_or("package")
            .trim_start_matches('@')
            .replace('/', "-");

        if let Some(scripts) = package_json["scripts"].as_object() {
            for (script_name, _script_value) in scripts {
                let category = Self::categorize_script(script_name);
                let emoji = category.emoji();

                let label = match &scope {
                    CommandScope::Workspace => format!("{} {} (all)", emoji, script_name),
                    CommandScope::Package(pkg) => format!("{} {} ({})", emoji, script_name, pkg),
                    CommandScope::Global => format!("{} {}", emoji, script_name),
                };

                let description = match &scope {
                    CommandScope::Workspace => {
                        format!("Run {} script in all packages", script_name)
                    }
                    CommandScope::Package(pkg) => {
                        format!("Run {} script in {}", script_name, pkg)
                    }
                    CommandScope::Global => format!("Run {} script", script_name),
                };

                let id = format!("npm.{}.{}", package_name, script_name);

                commands.push(
                    DiscoveredCommand::new(id, label, category)
                        .description(description)
                        .source("package.json")
                        .scope(scope.clone())
                        .handler({
                            let pm = package_manager.to_string();
                            let script = script_name.clone();
                            let path = package_path.to_path_buf();
                            move |_ctx| {
                                crate::command::run_command(
                                    &pm,
                                    &vec!["run".to_string(), script.clone()],
                                    &path,
                                )
                            }
                        }),
                );
            }
        }

        Ok(commands)
    }
}

impl CommandProvider for NpmProvider {
    fn name(&self) -> &'static str {
        "npm"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        let has_node = cmd_exists("node");
        let has_package_json = ctx.repo.join("package.json").exists();
        has_node && has_package_json
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();
        let package_manager = Self::detect_package_manager(&ctx.repo);

        // Check if it's a workspace root
        let root_package_json_path = ctx.repo.join("package.json");
        if root_package_json_path.exists() {
            let content = fs::read_to_string(&root_package_json_path)?;
            let package_json: Value = serde_json::from_str(&content)?;

            // Check for workspaces
            let has_workspaces = package_json.get("workspaces").is_some()
                || ctx.repo.join("pnpm-workspace.yaml").exists();

            if has_workspaces {
                // Discover workspace root scripts
                commands.extend(Self::discover_package_scripts(
                    &ctx.repo,
                    package_manager,
                    CommandScope::Workspace,
                )?);
            } else {
                // Single package project
                commands.extend(Self::discover_package_scripts(
                    &ctx.repo,
                    package_manager,
                    CommandScope::Global,
                )?);
            }
        }

        // Discover scripts in workspace packages
        for (package_name, package_config) in &ctx.config.packages {
            let package_path = ctx.repo.join(&package_config.path);
            if package_path.join("package.json").exists() {
                commands.extend(Self::discover_package_scripts(
                    &package_path,
                    package_manager,
                    CommandScope::Package(package_name.clone()),
                )?);
            }
        }

        Ok(commands)
    }
}

impl Default for NpmProvider {
    fn default() -> Self {
        Self::new()
    }
}
