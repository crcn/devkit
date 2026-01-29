//! Code coverage generation

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Options for coverage generation
pub struct CoverageOptions {
    /// Output format (html, lcov, json, etc.)
    pub format: Option<String>,
    /// Open HTML report in browser
    pub open: bool,
    /// Custom coverage command (overrides config)
    pub command: Option<String>,
}

impl Default for CoverageOptions {
    fn default() -> Self {
        Self {
            format: None,
            open: false,
            command: None,
        }
    }
}

/// Run code coverage
pub fn run_coverage(ctx: &AppContext, opts: &CoverageOptions) -> Result<()> {
    // Determine coverage command
    let coverage_command = if let Some(cmd) = &opts.command {
        cmd.clone()
    } else if ctx.features.cargo {
        // Default to cargo-tarpaulin if available, otherwise cargo-llvm-cov
        if devkit_core::cmd_exists("cargo-tarpaulin") {
            let mut cmd = "cargo tarpaulin".to_string();
            if let Some(fmt) = &opts.format {
                cmd.push_str(&format!(" --out {}", fmt));
            } else if opts.open {
                cmd.push_str(" --out Html");
            } else {
                cmd.push_str(" --out Lcov");
            }
            cmd
        } else if devkit_core::cmd_exists("cargo-llvm-cov") {
            let mut cmd = "cargo llvm-cov".to_string();
            if let Some(fmt) = &opts.format {
                cmd.push_str(&format!(" --{}", fmt));
            } else if opts.open {
                cmd.push_str(" --html --open");
            } else {
                cmd.push_str(" --lcov");
            }
            cmd
        } else {
            return Err(anyhow!(
                "No coverage tool found. Install cargo-tarpaulin or cargo-llvm-cov:\n\
                 cargo install cargo-tarpaulin\n\
                 cargo install cargo-llvm-cov"
            ));
        }
    } else if ctx.features.node {
        // Try common JS coverage tools
        if devkit_core::cmd_exists("npm") {
            "npm test -- --coverage".to_string()
        } else if devkit_core::cmd_exists("yarn") {
            "yarn test --coverage".to_string()
        } else {
            return Err(anyhow!(
                "No coverage command found. Configure [test.coverage_command] in config"
            ));
        }
    } else {
        return Err(anyhow!(
            "No test framework detected. Configure [test.coverage_command] in config"
        ));
    };

    // Parse command
    let parts: Vec<&str> = coverage_command.split_whitespace().collect();
    let (exe, args) = parts.split_first().unwrap_or((&"echo", &[]));

    ctx.print_header(&format!("Generating coverage: {}", coverage_command));

    let code = CmdBuilder::new(*exe)
        .args(args.iter().copied())
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("{} exited with code {}", coverage_command, code));
    }

    // Open HTML report if requested
    if opts.open && ctx.features.cargo {
        let html_path = ctx.repo.join("target/llvm-cov/html/index.html");
        if html_path.exists() {
            ctx.print_success("Opening coverage report...");
            #[cfg(target_os = "macos")]
            {
                CmdBuilder::new("open")
                    .arg(html_path.to_str().unwrap())
                    .run()?;
            }
            #[cfg(target_os = "linux")]
            {
                CmdBuilder::new("xdg-open")
                    .arg(html_path.to_str().unwrap())
                    .run()?;
            }
            #[cfg(target_os = "windows")]
            {
                CmdBuilder::new("cmd")
                    .args(["/c", "start", html_path.to_str().unwrap()])
                    .run()?;
            }
        }
    }

    Ok(())
}
