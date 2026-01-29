//! ECS extension for devkit
//!
//! Provides AWS ECS container operations (exec, logs, status).

use anyhow::{anyhow, Result};
use console::style;
use devkit_core::{AppContext, Extension, MenuItem};
use devkit_tasks::CmdBuilder;

pub struct EcsExtension;

impl Extension for EcsExtension {
    fn name(&self) -> &str {
        "ecs"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        devkit_core::cmd_exists("aws")
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        // Note: ECS operations require cluster/task parameters
        // These are better used programmatically or via CLI args
        // For now, return empty menu items
        vec![]
    }
}

/// Execute a command in an ECS container
pub fn ecs_exec(
    ctx: &AppContext,
    cluster: &str,
    task: &str,
    container: Option<&str>,
) -> Result<()> {
    if !devkit_core::cmd_exists("aws") {
        return Err(anyhow!(
            "AWS CLI not found. Install from: https://aws.amazon.com/cli/"
        ));
    }

    // Check for Session Manager plugin
    let session_manager_check = std::process::Command::new("session-manager-plugin")
        .arg("--version")
        .output();

    if session_manager_check.is_err() {
        return Err(anyhow!(
            "Session Manager plugin not found.\n\
             Install from: https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html"
        ));
    }

    ctx.print_header(&format!("Connecting to ECS task {}", task));
    println!("Cluster: {}", style(cluster).cyan());

    let mut args = vec![
        "ecs",
        "execute-command",
        "--cluster",
        cluster,
        "--task",
        task,
        "--interactive",
        "--command",
        "/bin/bash",
    ];

    if let Some(c) = container {
        args.push("--container");
        args.push(c);
    }

    let code = CmdBuilder::new("aws")
        .args(args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    // 130 = SIGINT (Ctrl+D), 254 = normal exit from exec session
    if code != 0 && code != 130 && code != 254 {
        return Err(anyhow!("ECS exec exited with code {}", code));
    }

    Ok(())
}

/// List tasks in an ECS cluster
pub fn ecs_list_tasks(ctx: &AppContext, cluster: &str, service: Option<&str>) -> Result<()> {
    if !devkit_core::cmd_exists("aws") {
        return Err(anyhow!(
            "AWS CLI not found. Install from: https://aws.amazon.com/cli/"
        ));
    }

    ctx.print_header(&format!("Listing tasks in {}", cluster));

    let mut args = vec![
        "ecs".to_string(),
        "list-tasks".to_string(),
        "--cluster".to_string(),
        cluster.to_string(),
    ];

    if let Some(svc) = service {
        args.push("--service-name".to_string());
        args.push(svc.to_string());
    }

    let code = CmdBuilder::new("aws").args(&args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("Failed to list ECS tasks"));
    }

    Ok(())
}

/// View logs for an ECS task
pub fn ecs_logs(ctx: &AppContext, log_group: &str, task_id: &str) -> Result<()> {
    if !devkit_core::cmd_exists("aws") {
        return Err(anyhow!(
            "AWS CLI not found. Install from: https://aws.amazon.com/cli/"
        ));
    }

    ctx.print_header(&format!("Viewing logs for task {}", task_id));

    // Stream logs from CloudWatch
    let code = CmdBuilder::new("aws")
        .args([
            "logs",
            "tail",
            log_group,
            "--follow",
            "--filter-pattern",
            task_id,
        ])
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 && code != 130 {
        return Err(anyhow!("Failed to view logs"));
    }

    Ok(())
}

/// Check if this extension should be enabled
pub fn should_enable(_ctx: &devkit_core::AppContext) -> bool {
    // Enable if AWS CLI is available
    devkit_core::cmd_exists("aws")
}
