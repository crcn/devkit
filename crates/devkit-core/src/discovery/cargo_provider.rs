//! Cargo command provider
//!
//! Discovers Rust cargo commands in workspaces and individual packages

use anyhow::Result;

use crate::context::AppContext;
use crate::discovery::{Category, CommandProvider, CommandScope, DiscoveredCommand};
use crate::utils::cmd_exists;

pub struct CargoProvider;

impl CargoProvider {
    pub fn new() -> Self {
        Self
    }

    fn has_cargo_workspace(ctx: &AppContext) -> bool {
        ctx.repo.join("Cargo.toml").exists()
    }

    fn discover_workspace_commands(ctx: &AppContext) -> Vec<DiscoveredCommand> {
        let mut commands = Vec::new();

        // Build all
        commands.push(
            DiscoveredCommand::new("cargo.build.all", "📦 Build all packages", Category::Build)
                .description("Build all packages in workspace")
                .source("Cargo.toml")
                .scope(CommandScope::Workspace)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command(
                            "cargo",
                            &vec!["build".to_string()],
                            &repo,
                        )
                    }
                }),
        );

        // Build release
        commands.push(
            DiscoveredCommand::new(
                "cargo.build.release.all",
                "📦 Build all (release)",
                Category::Build,
            )
            .description("Build all packages in release mode")
            .source("Cargo.toml")
            .scope(CommandScope::Workspace)
            .handler({
                let repo = ctx.repo.clone();
                move |_ctx| {
                    crate::command::run_command(
                        "cargo",
                        &vec!["build".to_string(), "--release".to_string()],
                        &repo,
                    )
                }
            }),
        );

        // Test all
        commands.push(
            DiscoveredCommand::new("cargo.test.all", "🧪 Test all packages", Category::Test)
                .description("Run tests for all packages")
                .source("Cargo.toml")
                .scope(CommandScope::Workspace)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command("cargo", &vec!["test".to_string()], &repo)
                    }
                }),
        );

        // Clippy all
        commands.push(
            DiscoveredCommand::new("cargo.clippy.all", "✨ Lint all packages", Category::Quality)
                .description("Run clippy on all packages")
                .source("Cargo.toml")
                .scope(CommandScope::Workspace)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command(
                            "cargo",
                            &vec![
                                "clippy".to_string(),
                                "--all-targets".to_string(),
                                "--all-features".to_string(),
                                "--".to_string(),
                                "-D".to_string(),
                                "warnings".to_string(),
                            ],
                            &repo,
                        )
                    }
                }),
        );

        // Format all
        commands.push(
            DiscoveredCommand::new("cargo.fmt.all", "✨ Format all packages", Category::Quality)
                .description("Format all packages with rustfmt")
                .source("Cargo.toml")
                .scope(CommandScope::Workspace)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command(
                            "cargo",
                            &vec!["fmt".to_string(), "--all".to_string()],
                            &repo,
                        )
                    }
                }),
        );

        // Check all
        commands.push(
            DiscoveredCommand::new("cargo.check.all", "✨ Check all packages", Category::Quality)
                .description("Run cargo check on all packages")
                .source("Cargo.toml")
                .scope(CommandScope::Workspace)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        crate::command::run_command(
                            "cargo",
                            &vec![
                                "check".to_string(),
                                "--all-targets".to_string(),
                                "--all-features".to_string(),
                            ],
                            &repo,
                        )
                    }
                }),
        );

        commands
    }
}

impl CommandProvider for CargoProvider {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        cmd_exists("cargo") && Self::has_cargo_workspace(ctx)
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();

        // Add workspace-level commands
        commands.extend(Self::discover_workspace_commands(ctx));

        Ok(commands)
    }
}

impl Default for CargoProvider {
    fn default() -> Self {
        Self::new()
    }
}
