//! Container log following with auto-reconnect

use anyhow::{anyhow, Result};
use console::style;
use devkit_core::{utils::cmd_exists, AppContext};
use devkit_tasks::CmdBuilder;

/// Check if a container is running
fn is_container_running(container: &str) -> bool {
    let output = std::process::Command::new("docker")
        .args(["inspect", "-f", "{{.State.Running}}", container])
        .output();

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim() == "true",
        Err(_) => false,
    }
}

/// Follow container logs with auto-reconnect
pub fn follow_logs(ctx: &AppContext, container: &str) -> Result<()> {
    if !cmd_exists("docker") {
        return Err(anyhow!("docker not found. Install Docker Desktop."));
    }

    ctx.print_header(&format!("Following logs for: {}", container));

    if !ctx.quiet {
        println!(
            "{}",
            style("Auto-reconnect enabled. Press Ctrl+C to exit.").yellow()
        );
        println!();
    }

    loop {
        // Check if container is running
        if !is_container_running(container) {
            if !ctx.quiet {
                println!(
                    "{}",
                    style("Container not running. Waiting for it to start...").yellow()
                );
            }
            // Wait before retrying
            std::thread::sleep(std::time::Duration::from_secs(2));
            continue;
        }

        // Follow logs with tail
        let code = CmdBuilder::new("docker")
            .args(["logs", "-f", "--tail", "200", container])
            .cwd(&ctx.repo)
            .inherit_io()
            .run()?;

        // Exit code 130 = Ctrl+C, exit normally
        if code == 130 {
            break;
        }

        // Container likely stopped/restarted, show message and retry
        if !ctx.quiet {
            println!();
            println!(
                "{}",
                style("Container disconnected. Reconnecting in 2 seconds...").yellow()
            );
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    Ok(())
}
