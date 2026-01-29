//! Code linting (clippy, eslint)

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, utils::cmd_exists};
use devkit_tasks::CmdBuilder;
use std::process::{Command, Stdio};

/// Run all linters
pub fn run_lint(ctx: &AppContext, fix: bool) -> Result<()> {
    run_lint_with_capture(ctx, fix, false).map(|_| ())
}

/// Run linters with optional error capture (for AI fixing)
pub fn run_lint_with_capture(
    ctx: &AppContext,
    fix: bool,
    capture_errors: bool,
) -> Result<Option<String>> {
    ctx.print_header("Running linters");

    let mut had_errors = false;
    let mut error_output = String::new();

    // Rust linting with clippy
    if cmd_exists("cargo") {
        if !ctx.quiet {
            println!("[lint] Running cargo clippy...");
        }
        let mut args = vec!["clippy", "--all-targets", "--all-features"];
        if fix {
            args.extend(["--fix", "--allow-dirty", "--allow-staged"]);
        }
        args.extend(["--", "-D", "warnings"]);

        if capture_errors {
            let output = Command::new("cargo")
                .args(&args)
                .current_dir(&ctx.repo)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !output.status.success() {
                ctx.print_warning("clippy found issues");
                error_output.push_str("=== clippy ===\n");
                error_output.push_str(&String::from_utf8_lossy(&output.stderr));
                error_output.push_str(&String::from_utf8_lossy(&output.stdout));
                had_errors = true;
            }
        } else {
            let code = CmdBuilder::new("cargo").args(args).cwd(&ctx.repo).run()?;
            if code != 0 {
                ctx.print_warning("clippy found issues");
                had_errors = true;
            }
        }
    }

    // TypeScript/JavaScript linting with eslint
    if let Some(mobile_pkg) = find_mobile_package(ctx) {
        if mobile_pkg.exists() && cmd_exists("npx") {
            let app_rel = mobile_pkg.strip_prefix(&ctx.repo).unwrap_or(&mobile_pkg);
            if !ctx.quiet {
                println!("[lint] Running eslint on {}...", app_rel.display());
            }
            let mut args = vec!["eslint", "src"];
            if fix {
                args.push("--fix");
            }

            if capture_errors {
                let output = Command::new("npx")
                    .args(&args)
                    .current_dir(&mobile_pkg)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()?;

                if !output.status.success() {
                    ctx.print_warning("eslint found issues");
                    error_output.push_str("\n=== eslint ===\n");
                    error_output.push_str(&String::from_utf8_lossy(&output.stderr));
                    error_output.push_str(&String::from_utf8_lossy(&output.stdout));
                    had_errors = true;
                }
            } else {
                let code = CmdBuilder::new("npx").args(args).cwd(&mobile_pkg).run()?;
                if code != 0 {
                    ctx.print_warning("eslint found issues");
                    had_errors = true;
                }
            }
        }
    }

    if had_errors {
        if capture_errors {
            return Ok(Some(error_output));
        }
        return Err(anyhow!("Linting found issues"));
    }

    ctx.print_success("Linting complete.");
    Ok(None)
}

/// Find mobile/app package from config
fn find_mobile_package(ctx: &AppContext) -> Option<std::path::PathBuf> {
    ctx.config
        .packages
        .values()
        .find(|pkg| pkg.mobile.is_some())
        .map(|pkg| pkg.path.clone())
}
