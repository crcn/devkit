//! Docker compose operations

use devkit_core::{AppContext, DevkitError, Extension, MenuItem, Result, utils::docker_compose_program};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;

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
                handler: Box::new(|ctx| compose_up(ctx, &[], false)),
            },
            MenuItem {
                label: "🐳 Docker - Down".to_string(),
                handler: Box::new(|ctx| compose_down(ctx)),
            },
            MenuItem {
                label: "🐳 Docker - Restart".to_string(),
                handler: Box::new(|ctx| compose_restart(ctx, &[])),
            },
            MenuItem {
                label: "🐳 Docker - Logs".to_string(),
                handler: Box::new(|ctx| logs(ctx, None)),
            },
            MenuItem {
                label: "🐳 Docker - Shell".to_string(),
                handler: Box::new(|ctx| shell(ctx, None)),
            },
        ]
    }
}

pub fn compose_up(ctx: &AppContext, services: &[String], build: bool) -> Result<()> {
    let (prog, mut args) = docker_compose_program()?;
    args.push("up".to_string());
    args.push("-d".to_string());

    if build {
        args.push("--build".to_string());
    }

    args.extend(services.iter().cloned());

    let pb = if !ctx.quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Starting containers...");
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let output = Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevkitError::docker_compose_failed(stderr.to_string()));
    }

    ctx.print_success("✓ Containers started");
    Ok(())
}

pub fn compose_down(ctx: &AppContext) -> Result<()> {
    let (prog, mut args) = docker_compose_program()?;
    args.push("down".to_string());

    let pb = if !ctx.quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.yellow} {msg}")
                .unwrap(),
        );
        pb.set_message("Stopping containers...");
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let output = Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevkitError::docker_compose_failed(stderr.to_string()));
    }

    ctx.print_success("✓ Containers stopped");
    Ok(())
}

pub fn compose_restart(ctx: &AppContext, services: &[String]) -> Result<()> {
    let (prog, mut args) = docker_compose_program()?;
    args.push("restart".to_string());
    args.extend(services.iter().cloned());

    let pb = if !ctx.quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message("Restarting containers...");
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let output = Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevkitError::docker_compose_failed(stderr.to_string()));
    }

    ctx.print_success("✓ Containers restarted");
    Ok(())
}

pub fn logs(ctx: &AppContext, service: Option<&str>) -> Result<()> {
    let (prog, mut args) = docker_compose_program()?;
    args.push("logs".to_string());
    args.push("-f".to_string());
    args.push("--tail".to_string());
    args.push("100".to_string());

    if let Some(svc) = service {
        args.push(svc.to_string());
    }

    ctx.print_info("Following logs...");

    let output = Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevkitError::docker_compose_failed(stderr.to_string()));
    }

    Ok(())
}

pub fn shell(ctx: &AppContext, service: Option<&str>) -> Result<()> {
    let service = match service {
        Some(s) => s.to_string(),
        None => {
            // Get first running service
            let (prog, mut args) = docker_compose_program()?;
            args.push("ps".to_string());
            args.push("--services".to_string());
            args.push("--filter".to_string());
            args.push("status=running".to_string());

            let output = Command::new(&prog)
                .args(&args)
                .current_dir(&ctx.repo)
                .output()?;

            let services = String::from_utf8_lossy(&output.stdout);
            services
                .lines()
                .next()
                .ok_or_else(|| DevkitError::docker_compose_failed(
                    "No running containers found".to_string()
                ))?
                .to_string()
        }
    };

    let (prog, mut args) = docker_compose_program()?;
    args.push("exec".to_string());
    args.push(service.clone());
    args.push("sh".to_string());

    ctx.print_info(&format!("Opening shell in {}...", service));

    let output = Command::new(&prog)
        .args(&args)
        .current_dir(&ctx.repo)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevkitError::docker_compose_failed(stderr.to_string()));
    }

    Ok(())
}
