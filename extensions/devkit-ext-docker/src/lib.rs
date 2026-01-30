//! Docker compose operations

mod compose;
mod logs;
mod shell;

pub use compose::*;
pub use logs::*;
pub use shell::*;

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

pub struct DockerExtension;

impl Extension for DockerExtension {
    fn name(&self) -> &str {
        "docker"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.docker
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🐳 Docker - Up".to_string(),
                handler: Box::new(|ctx| Ok(docker_up_interactive(ctx)?)),
            },
            MenuItem {
                label: "🐳 Docker - Down".to_string(),
                handler: Box::new(|ctx| Ok(compose_down(ctx)?)),
            },
            MenuItem {
                label: "🐳 Docker - Restart".to_string(),
                handler: Box::new(|ctx| Ok(docker_restart_interactive(ctx)?)),
            },
            MenuItem {
                label: "🐳 Docker - Logs".to_string(),
                handler: Box::new(|ctx| Ok(docker_logs_interactive(ctx)?)),
            },
            MenuItem {
                label: "🐳 Docker - Shell".to_string(),
                handler: Box::new(|ctx| Ok(docker_shell_interactive(ctx)?)),
            },
            MenuItem {
                label: "🐳 Docker - Build".to_string(),
                handler: Box::new(|ctx| Ok(docker_build_interactive(ctx)?)),
            },
        ]
    }
}

// =============================================================================
// Interactive Container Selection
// =============================================================================

/// Select containers interactively with [All] option
#[allow(dead_code)]
fn select_containers_multi(
    ctx: &AppContext,
    prompt: &str,
    include_all: bool,
) -> Result<Vec<String>> {
    let running = list_running_containers(ctx)?;

    if running.is_empty() {
        return Err(anyhow!("No running containers found"));
    }

    let mut items = Vec::new();
    if include_all {
        items.push("[All]".to_string());
    }
    items.extend(running.iter().map(|c| c.label.clone()));

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .interact()?;

    if selection.is_empty() {
        return Err(anyhow!("No containers selected"));
    }

    // If [All] is selected, return all container IDs
    if include_all && selection.contains(&0) {
        return Ok(running.iter().map(|c| c.id.clone()).collect());
    }

    // Otherwise, map selected indices to container IDs
    let offset = if include_all { 1 } else { 0 };
    Ok(selection
        .iter()
        .map(|&i| running[i - offset].id.clone())
        .collect())
}

/// Select a single container interactively
fn select_container_single(ctx: &AppContext, prompt: &str) -> Result<String> {
    let running = list_running_containers(ctx)?;

    if running.is_empty() {
        return Err(anyhow!("No running containers found"));
    }

    let items: Vec<String> = running.iter().map(|c| c.label.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()?;

    Ok(running[selection].id.clone())
}

/// Select services (not containers) for operations that work with service names
fn select_services_multi(
    ctx: &AppContext,
    prompt: &str,
    include_all: bool,
) -> Result<Vec<String>> {
    let services = list_services(ctx)?;

    if services.is_empty() {
        return Err(anyhow!("No services found in docker-compose.yml"));
    }

    let mut items = Vec::new();
    if include_all {
        items.push("[All]".to_string());
    }
    items.extend(services.clone());

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .interact()?;

    if selection.is_empty() {
        return Err(anyhow!("No services selected"));
    }

    // If [All] is selected or nothing specific selected, return empty (means all)
    if include_all && selection.contains(&0) {
        return Ok(Vec::new());
    }

    // Otherwise, map selected indices to service names
    let offset = if include_all { 1 } else { 0 };
    Ok(selection
        .iter()
        .map(|&i| services[i - offset].clone())
        .collect())
}

// =============================================================================
// Interactive Menu Handlers
// =============================================================================

/// Interactive handler for docker up
fn docker_up_interactive(ctx: &AppContext) -> Result<()> {
    let services = select_services_multi(
        ctx,
        "Select services to start (space to select, enter to confirm)",
        true,
    )?;

    compose_up(ctx, &services, false)
}

/// Interactive handler for docker restart
fn docker_restart_interactive(ctx: &AppContext) -> Result<()> {
    let services = select_services_multi(
        ctx,
        "Select services to restart (space to select, enter to confirm)",
        true,
    )?;

    compose_restart(ctx, &services)
}

/// Interactive handler for docker build
fn docker_build_interactive(ctx: &AppContext) -> Result<()> {
    let services = select_services_multi(
        ctx,
        "Select services to build (space to select, enter to confirm)",
        true,
    )?;

    compose_build(ctx, &services, false, false)
}

/// Interactive handler for docker logs with live following
fn docker_logs_interactive(ctx: &AppContext) -> Result<()> {
    let container_id = select_container_single(
        ctx,
        "Select container to follow logs",
    )?;

    follow_logs(ctx, &container_id)
}

/// Interactive handler for docker shell
fn docker_shell_interactive(ctx: &AppContext) -> Result<()> {
    let container_id = select_container_single(
        ctx,
        "Select container to open shell",
    )?;

    open_shell(ctx, &container_id)
}

// =============================================================================
// CLI Compatibility Wrappers
// =============================================================================

/// Follow logs for a service (CLI compatibility wrapper)
pub fn logs(ctx: &AppContext, service: Option<&str>) -> Result<()> {
    use devkit_core::utils::docker_compose_program;
    use devkit_tasks::CmdBuilder;

    let (prog, mut args) = docker_compose_program()?;
    args.push("logs".to_string());
    args.push("-f".to_string());
    args.push("--tail".to_string());
    args.push("100".to_string());

    if let Some(svc) = service {
        args.push(svc.to_string());
    }

    ctx.print_info("Following logs...");

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker compose logs exited with code {}", code));
    }

    Ok(())
}

/// Open shell in a service (CLI compatibility wrapper)
pub fn shell(ctx: &AppContext, service: Option<&str>) -> Result<()> {
    use devkit_core::utils::docker_compose_program;
    use devkit_tasks::CmdBuilder;

    let service = match service {
        Some(s) => s.to_string(),
        None => {
            // Get first running service
            let (prog, mut args) = docker_compose_program()?;
            args.push("ps".to_string());
            args.push("--services".to_string());
            args.push("--filter".to_string());
            args.push("status=running".to_string());

            let out = CmdBuilder::new(&prog)
                .args(&args)
                .cwd(&ctx.repo)
                .capture_stdout()
                .run_capture()?;

            let services = out.stdout_lines();
            services
                .first()
                .ok_or_else(|| anyhow!("No running containers found"))?
                .to_string()
        }
    };

    let (prog, mut args) = docker_compose_program()?;
    args.push("exec".to_string());
    args.push(service.clone());
    args.push("sh".to_string());

    ctx.print_info(&format!("Opening shell in {}...", service));

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker exec exited with code {}", code));
    }

    Ok(())
}
