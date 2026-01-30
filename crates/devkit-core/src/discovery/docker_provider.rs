//! Docker command provider
//!
//! Discovers docker-compose services and provides rich interactive commands

use anyhow::Result;
use std::fs;

use crate::context::AppContext;
use crate::discovery::{Category, CommandProvider, CommandScope, DiscoveredCommand};
use crate::utils::docker_available;

pub struct DockerProvider;

impl DockerProvider {
    pub fn new() -> Self {
        Self
    }

    fn has_docker_compose(ctx: &AppContext) -> bool {
        ctx.repo.join("docker-compose.yml").exists()
            || ctx.repo.join("docker-compose.yaml").exists()
            || ctx.repo.join("compose.yml").exists()
            || ctx.repo.join("compose.yaml").exists()
    }

    fn find_compose_file(ctx: &AppContext) -> Option<String> {
        let candidates = [
            "docker-compose.yml",
            "docker-compose.yaml",
            "compose.yml",
            "compose.yaml",
        ];

        for candidate in &candidates {
            if ctx.repo.join(candidate).exists() {
                return Some(candidate.to_string());
            }
        }

        None
    }

    fn parse_services(ctx: &AppContext) -> Result<Vec<String>> {
        let compose_file = Self::find_compose_file(ctx).unwrap_or_default();
        let compose_path = ctx.repo.join(&compose_file);

        if !compose_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&compose_path)?;
        let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;

        let mut services = Vec::new();

        if let Some(services_map) = yaml.get("services").and_then(|s| s.as_mapping()) {
            for (key, _) in services_map {
                if let Some(service_name) = key.as_str() {
                    services.push(service_name.to_string());
                }
            }
        }

        Ok(services)
    }
}

impl CommandProvider for DockerProvider {
    fn name(&self) -> &'static str {
        "docker"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        docker_available() && Self::has_docker_compose(ctx)
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();
        let compose_file = Self::find_compose_file(ctx).unwrap_or_default();

        // Up (start all services)
        commands.push(
            DiscoveredCommand::new("docker.up", "🐳 Start services", Category::Services)
                .description("Start all Docker services")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.push("up".to_string());
                        args.push("-d".to_string());

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        // Down (stop all services)
        commands.push(
            DiscoveredCommand::new("docker.down", "🐳 Stop services", Category::Services)
                .description("Stop all Docker services")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.push("down".to_string());

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        // Logs (interactive multi-select)
        commands.push(
            DiscoveredCommand::new("docker.logs", "🐳 View logs", Category::Services)
                .description("Follow logs from containers (multi-select)")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // This will use the docker extension's interactive functionality
                        // For now, just show all logs
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.extend(["logs".to_string(), "-f".to_string(), "--tail".to_string(), "200".to_string()]);

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        // Restart
        commands.push(
            DiscoveredCommand::new("docker.restart", "🐳 Restart services", Category::Services)
                .description("Restart Docker services")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.push("restart".to_string());

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        // Build
        commands.push(
            DiscoveredCommand::new("docker.build", "🐳 Build images", Category::Build)
                .description("Build Docker images")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.push("build".to_string());

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        // PS (show running containers)
        commands.push(
            DiscoveredCommand::new("docker.ps", "🐳 Show containers", Category::Services)
                .description("Show running containers")
                .source(&compose_file)
                .scope(CommandScope::Global)
                .handler({
                    let repo = ctx.repo.clone();
                    move |_ctx| {
                        // Use simple command execution
                        use crate::utils::docker_compose_program;

                        let (prog, mut args) = docker_compose_program()?;
                        args.push("ps".to_string());

                        crate::command::run_command(&prog, &args, &repo)
                    }
                }),
        );

        Ok(commands)
    }
}

impl Default for DockerProvider {
    fn default() -> Self {
        Self::new()
    }
}
