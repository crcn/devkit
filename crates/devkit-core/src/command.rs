//! Simple command execution utilities

use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

/// Run a command and wait for it to complete
pub fn run_command(program: &str, args: &[String], cwd: &Path) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context(format!("Failed to execute command: {}", program))?;

    if !status.success() {
        anyhow::bail!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
