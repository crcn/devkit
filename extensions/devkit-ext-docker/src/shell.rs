//! Interactive shell access to containers

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, utils::cmd_exists};
use devkit_tasks::CmdBuilder;

/// Open an interactive shell in a container
pub fn open_shell(ctx: &AppContext, container_id: &str) -> Result<()> {
    if !cmd_exists("docker") {
        return Err(anyhow!("docker not found. Install Docker Desktop."));
    }

    ctx.print_header(&format!("Opening shell in: {}", container_id));

    // Try common shells in order
    let shells = ["bash", "sh", "ash"];
    for shell in shells {
        let code = CmdBuilder::new("docker")
            .args(["exec", "-it", container_id, shell])
            .cwd(&ctx.repo)
            .inherit_io()
            .run()?;

        // If shell worked (exit 0) or user exited (130 = Ctrl+C), we're done
        if code == 0 || code == 130 {
            return Ok(());
        }
        // If exit code is 126 or 127, shell not found - try next
        if code != 126 && code != 127 {
            return Err(anyhow!("Shell exited with code {}", code));
        }
    }

    Err(anyhow!("No shell found in container"))
}
