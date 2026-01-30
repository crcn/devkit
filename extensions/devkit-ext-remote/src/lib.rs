//! Remote development support extension
//!
//! Enables SSH-based remote development with file sync and command execution

use anyhow::{Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use std::process::Command;

pub struct RemoteExtension;

impl Extension for RemoteExtension {
    fn name(&self) -> &str {
        "remote"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        cmd_exists("ssh") && cmd_exists("rsync")
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🌐 Connect to remote".to_string(),
                group: None,
                handler: Box::new(|ctx| connect_remote(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "🔄 Sync files to remote".to_string(),
                group: None,
                handler: Box::new(|ctx| sync_to_remote(ctx).map_err(Into::into)),
            },
        ]
    }
}

fn cmd_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Connect to remote environment
pub fn connect_remote(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Remote Development");
    println!();

    ctx.print_info("Remote development enables:");
    println!("  • SSH connection management");
    println!("  • File synchronization (rsync)");
    println!("  • Remote command execution");
    println!("  • Port forwarding");
    println!("  • Context switching");
    println!();

    ctx.print_info("Configuration in .dev/config.toml:");
    println!();
    println!("  [remote.staging]");
    println!("  host = \"staging.example.com\"");
    println!("  user = \"deploy\"");
    println!("  path = \"/app\"");
    println!("  sync_patterns = [\"src/**\", \"Cargo.toml\"]");
    println!("  port_forwards = [\"8080:8080\"]");
    println!();

    ctx.print_info("Commands:");
    println!("  devkit remote connect staging");
    println!("  devkit remote sync");
    println!("  devkit remote exec \"cargo build\"");
    println!("  devkit remote forward");

    Ok(())
}

/// Sync files to remote
pub fn sync_to_remote(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Syncing files to remote...");

    // Example rsync command
    ctx.print_info("Would run: rsync -avz --exclude target/ ./ user@host:/path");
    ctx.print_success("✓ Files synced (demo mode)");

    Ok(())
}

/// Execute command on remote
pub fn exec_remote(ctx: &AppContext, remote: &str, command: &str) -> Result<()> {
    ctx.print_info(&format!("Executing on {}: {}", remote, command));

    // Parse remote config
    // let config = load_remote_config(ctx, remote)?;

    // Execute via SSH
    let output = Command::new("ssh")
        .args(&[remote, command])
        .output()
        .context("Failed to execute remote command")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        ctx.print_success("✓ Command executed successfully");
    } else {
        ctx.print_error(&format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Start port forwarding
pub fn port_forward(
    ctx: &AppContext,
    remote: &str,
    local_port: u16,
    remote_port: u16,
) -> Result<()> {
    ctx.print_info(&format!(
        "Forwarding localhost:{} -> {}:{}",
        local_port, remote, remote_port
    ));

    let status = Command::new("ssh")
        .args(&[
            "-L",
            &format!("{}:localhost:{}", local_port, remote_port),
            "-N",
            remote,
        ])
        .status()
        .context("Failed to start port forwarding")?;

    if !status.success() {
        return Err(anyhow::anyhow!("Port forwarding failed"));
    }

    Ok(())
}

/// Watch and sync files on changes
pub fn watch_and_sync(ctx: &AppContext, _remote: &str) -> Result<()> {
    ctx.print_info("Starting file watcher for remote sync...");
    ctx.print_info("This would watch files and rsync on changes");
    ctx.print_info("Integration with devkit-tasks watch module");

    Ok(())
}
