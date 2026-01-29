//! Docker Compose operations

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, utils::{docker_compose_program, ensure_docker}};
use devkit_tasks::CmdBuilder;
use std::cell::RefCell;

// =============================================================================
// Service Cache
// =============================================================================

thread_local! {
    static SERVICE_CACHE: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

/// Get compose services with caching
pub fn list_services(ctx: &AppContext) -> Result<Vec<String>> {
    SERVICE_CACHE.with(|cache| {
        if let Some(ref services) = *cache.borrow() {
            return Ok(services.clone());
        }
        let services = list_services_uncached(ctx)?;
        *cache.borrow_mut() = Some(services.clone());
        Ok(services)
    })
}

/// Invalidate the service cache (call after starting/stopping containers)
fn invalidate_cache() {
    SERVICE_CACHE.with(|cache| {
        *cache.borrow_mut() = None;
    });
}

/// List all services defined in docker-compose.yml (uncached)
fn list_services_uncached(ctx: &AppContext) -> Result<Vec<String>> {
    let (prog, base_args) = docker_compose_program()?;

    let mut args = base_args;
    args.extend(["config", "--services"].map(String::from));

    let out = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;

    let mut svcs = out.stdout_lines();
    svcs.sort();
    Ok(svcs)
}

// =============================================================================
// Container Info
// =============================================================================

#[derive(Debug, Clone)]
pub struct Container {
    pub label: String,
    pub id: String,
}

/// List running containers from docker compose
pub fn list_running_containers(ctx: &AppContext) -> Result<Vec<Container>> {
    let (prog, base_args) = docker_compose_program()?;

    let mut args = base_args.clone();
    args.extend(["ps", "--services", "--filter", "status=running"].map(String::from));

    let out = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;

    let services = out.stdout_lines();
    let mut containers: Vec<Container> = Vec::new();

    for svc in services {
        let mut args2 = base_args.clone();
        args2.extend(["ps", "-q"].map(String::from));
        args2.push(svc.clone());

        let out2 = CmdBuilder::new(&prog)
            .args(&args2)
            .cwd(&ctx.repo)
            .capture_stdout()
            .run_capture()?;

        for id in out2.stdout_lines() {
            let short = id.chars().take(12).collect::<String>();
            containers.push(Container {
                label: format!("{svc} ({short})"),
                id,
            });
        }
    }

    containers.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(containers)
}

// =============================================================================
// Compose Operations
// =============================================================================

/// Start docker containers (docker compose up -d)
pub fn compose_up(ctx: &AppContext, services: &[String], build: bool) -> Result<()> {
    ensure_docker()?;

    let (prog, base_args) = docker_compose_program()?;
    let mut args = base_args;
    args.push("up".to_string());
    args.push("-d".to_string());

    if build {
        args.push("--build".to_string());
    }
    args.extend(services.iter().cloned());

    ctx.print_header("Starting docker containers");
    if !ctx.quiet {
        println!("[docker] {} {}", prog, args.join(" "));
    }

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker compose up exited with code {code}"));
    }

    invalidate_cache();
    ctx.print_success("Docker containers started!");
    Ok(())
}

/// Stop docker containers (docker compose down)
pub fn compose_down(ctx: &AppContext) -> Result<()> {
    ensure_docker()?;

    ctx.print_header("Stopping docker containers");

    let (prog, base_args) = docker_compose_program()?;
    let mut args = base_args;
    args.push("down".to_string());

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker compose down exited with code {code}"));
    }

    invalidate_cache();
    ctx.print_success("Docker containers stopped!");
    Ok(())
}

/// Restart docker containers
pub fn compose_restart(ctx: &AppContext, services: &[String]) -> Result<()> {
    ensure_docker()?;

    let (prog, base_args) = docker_compose_program()?;
    let mut args = base_args;
    args.push("restart".to_string());
    args.extend(services.iter().cloned());

    ctx.print_header("Restarting docker containers");
    if !ctx.quiet {
        println!("[docker] {} {}", prog, args.join(" "));
    }

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker compose restart exited with code {code}"));
    }

    ctx.print_success("Docker containers restarted!");
    Ok(())
}

/// Build docker images
pub fn compose_build(
    ctx: &AppContext,
    services: &[String],
    pull: bool,
    no_cache: bool,
) -> Result<()> {
    ensure_docker()?;

    let (prog, base_args) = docker_compose_program()?;
    let mut args = base_args;
    args.push("build".to_string());

    if pull {
        args.push("--pull".to_string());
    }
    if no_cache {
        args.push("--no-cache".to_string());
    }
    args.extend(services.iter().cloned());

    ctx.print_header("Building docker images");
    if !ctx.quiet {
        println!("[docker] {} {}", prog, args.join(" "));
    }

    let code = CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("docker compose build exited with code {code}"));
    }

    invalidate_cache();
    ctx.print_success("Docker images built!");
    Ok(())
}

/// Nuke and rebuild docker images (stop, remove containers, remove images, rebuild)
pub fn nuke_rebuild(ctx: &AppContext, services: &[String]) -> Result<()> {
    ensure_docker()?;

    let (prog, base_args) = docker_compose_program()?;

    ctx.print_header("Nuke and rebuild docker images");
    ctx.print_warning("This will stop containers, remove images, and rebuild from scratch");

    // Get image names before removing
    let images = get_service_images(ctx, services)?;
    if !ctx.quiet && !images.is_empty() {
        println!("[docker] Images to remove: {}", images.join(", "));
    }

    // Step 1: Stop and remove containers
    if !ctx.quiet {
        println!("[docker] Stopping and removing containers...");
    }
    let mut args = base_args.clone();
    args.extend(["rm", "-sf"].map(String::from));
    args.extend(services.iter().cloned());

    CmdBuilder::new(&prog)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    // Step 2: Remove images
    if !images.is_empty() {
        if !ctx.quiet {
            println!("[docker] Removing images...");
        }
        CmdBuilder::new("docker")
            .args(["rmi", "-f"])
            .args(&images)
            .cwd(&ctx.repo)
            .inherit_io()
            .run()?;
    }

    // Step 3: Rebuild
    if !ctx.quiet {
        println!("[docker] Rebuilding...");
    }
    compose_build(ctx, services, true, true)?;

    invalidate_cache();
    ctx.print_success("Nuke and rebuild complete!");
    Ok(())
}

/// Get image names for compose services
fn get_service_images(ctx: &AppContext, services: &[String]) -> Result<Vec<String>> {
    let (prog, base_args) = docker_compose_program()?;
    let mut args = base_args;
    args.extend(["images", "-q"].map(String::from));
    args.extend(services.iter().cloned());

    let output = std::process::Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let images: Vec<String> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(images)
}
