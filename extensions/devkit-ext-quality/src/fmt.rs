//! Code formatting (cargo fmt, prettier)

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, utils::cmd_exists};
use devkit_tasks::CmdBuilder;
use std::process::{Command, Stdio};

/// Run all formatters
pub fn run_fmt(ctx: &AppContext, fix: bool) -> Result<()> {
    run_fmt_with_capture(ctx, fix, false).map(|_| ())
}

/// Run formatters with optional error capture (for AI fixing)
pub fn run_fmt_with_capture(
    ctx: &AppContext,
    fix: bool,
    capture_errors: bool,
) -> Result<Option<String>> {
    ctx.print_header("Running formatters");

    let mut had_errors = false;
    let mut error_output = String::new();

    // Rust formatting
    if cmd_exists("cargo") {
        if !ctx.quiet {
            println!("[fmt] Running cargo fmt...");
        }
        let mut args = vec!["fmt", "--all"];
        if !fix {
            args.push("--check");
        }

        if capture_errors {
            let output = Command::new("cargo")
                .args(&args)
                .current_dir(&ctx.repo)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;

            if !output.status.success() {
                ctx.print_warning("cargo fmt had issues");
                error_output.push_str("=== cargo fmt ===\n");
                error_output.push_str(&String::from_utf8_lossy(&output.stderr));
                error_output.push_str(&String::from_utf8_lossy(&output.stdout));
                had_errors = true;
            }
        } else {
            let code = CmdBuilder::new("cargo").args(args).cwd(&ctx.repo).run()?;
            if code != 0 {
                if fix {
                    ctx.print_warning("cargo fmt had issues");
                } else {
                    ctx.print_warning("cargo fmt check failed (run with --fix to auto-fix)");
                }
                had_errors = true;
            }
        }
    }

    // TypeScript/JavaScript formatting with prettier
    if let Some(mobile_pkg) = find_mobile_package(ctx) {
        if mobile_pkg.exists() && cmd_exists("npx") {
            let app_rel = mobile_pkg.strip_prefix(&ctx.repo).unwrap_or(&mobile_pkg);
            if !ctx.quiet {
                println!("[fmt] Running prettier on {}...", app_rel.display());
            }
            let mut args = vec!["prettier"];
            if fix {
                args.push("--write");
            } else {
                args.push("--check");
            }
            args.push("src/**/*.{ts,tsx,js,jsx,json,css}");

            if capture_errors {
                let output = Command::new("npx")
                    .args(&args)
                    .current_dir(&mobile_pkg)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()?;

                if !output.status.success() {
                    ctx.print_warning("prettier had issues");
                    error_output.push_str("\n=== prettier ===\n");
                    error_output.push_str(&String::from_utf8_lossy(&output.stderr));
                    error_output.push_str(&String::from_utf8_lossy(&output.stdout));
                    had_errors = true;
                }
            } else {
                let code = CmdBuilder::new("npx").args(args).cwd(&mobile_pkg).run()?;
                if code != 0 {
                    if fix {
                        ctx.print_warning("prettier had issues");
                    } else {
                        ctx.print_warning("prettier check failed (run with --fix to auto-fix)");
                    }
                    had_errors = true;
                }
            }
        }
    }

    if had_errors {
        if capture_errors {
            return Ok(Some(error_output));
        }
        if fix {
            ctx.print_success("Formatting complete (with warnings).");
        } else {
            return Err(anyhow!("Formatting check failed"));
        }
    } else {
        ctx.print_success("Formatting complete.");
    }

    Ok(None)
}

/// Find mobile/app package from config (generic detection)
fn find_mobile_package(ctx: &AppContext) -> Option<std::path::PathBuf> {
    ctx.config
        .packages
        .values()
        .find(|pkg| pkg.mobile.is_some())
        .map(|pkg| pkg.path.clone())
}
