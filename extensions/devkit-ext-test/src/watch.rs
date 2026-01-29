//! Test watch mode

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Parse command into executable and arguments
fn parse_command(command: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let (exe, args) = parts.split_first().unwrap_or((&"echo", &[]));
    (*exe, args.to_vec())
}

/// Watch tests for changes and re-run automatically
pub fn watch_tests(ctx: &AppContext, command: Option<&str>) -> Result<()> {
    // Determine watch command
    let watch_command = if let Some(cmd) = command {
        cmd.to_string()
    } else if ctx.features.cargo {
        // Default to cargo watch if available
        if devkit_core::cmd_exists("cargo-watch") {
            "cargo watch -x test".to_string()
        } else if devkit_core::cmd_exists("cargo-nextest") {
            return Err(anyhow!(
                "cargo-watch not found. Install with: cargo install cargo-watch"
            ));
        } else {
            return Err(anyhow!(
                "No watch command configured. Install cargo-watch or configure [test.watch_command]"
            ));
        }
    } else if ctx.features.node {
        // Try common JS test watchers
        if devkit_core::cmd_exists("npm") {
            "npm test -- --watch".to_string()
        } else if devkit_core::cmd_exists("yarn") {
            "yarn test --watch".to_string()
        } else {
            return Err(anyhow!(
                "No watch command found. Configure [test.watch_command] in config"
            ));
        }
    } else {
        return Err(anyhow!(
            "No test framework detected. Configure [test.watch_command] in config"
        ));
    };

    let (exe, base_args) = parse_command(&watch_command);
    let args: Vec<String> = base_args.iter().map(|s| s.to_string()).collect();

    ctx.print_header(&format!("Watching tests: {}", watch_command));
    ctx.print_warning("Press Ctrl+C to stop watching.");

    let code = CmdBuilder::new(exe)
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    // 130 = SIGINT (Ctrl+C), which is expected
    if code != 0 && code != 130 {
        return Err(anyhow!("{} exited with code {}", watch_command, code));
    }
    Ok(())
}
