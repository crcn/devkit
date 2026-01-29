//! Release management

use anyhow::{anyhow, Result};
use console::style;
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

use crate::version::{get_current_version, Version};

pub struct ReleaseOptions {
    /// Bump type: patch, minor, or major
    pub bump: BumpType,
    /// Tag message
    pub message: Option<String>,
    /// Skip pre-flight checks
    pub skip_checks: bool,
}

pub enum BumpType {
    Patch,
    Minor,
    Major,
}

impl Default for ReleaseOptions {
    fn default() -> Self {
        Self {
            bump: BumpType::Patch,
            message: None,
            skip_checks: false,
        }
    }
}

/// Create a new release
pub fn create_release(ctx: &AppContext, opts: &ReleaseOptions) -> Result<()> {
    let current = get_current_version(ctx)?;

    // Calculate new version
    let new_version = if let Some(ref curr) = current {
        match opts.bump {
            BumpType::Patch => curr.bump_patch(),
            BumpType::Minor => curr.bump_minor(),
            BumpType::Major => curr.bump_major(),
        }
    } else {
        Version {
            major: 0,
            minor: 1,
            patch: 0,
            prerelease: None,
        }
    };

    let tag = new_version.to_tag();

    ctx.print_header("Release Summary");
    println!();

    let current_str = current
        .as_ref()
        .map(|v| v.to_tag())
        .unwrap_or_else(|| "none".to_string());

    println!(
        "  {} → {}",
        style(&current_str).dim(),
        style(&tag).green().bold()
    );
    println!();

    // Run pre-flight checks unless skipped
    if !opts.skip_checks {
        run_preflight_checks(ctx)?;
    }

    // Create annotated tag
    let tag_message = opts
        .message
        .clone()
        .unwrap_or_else(|| format!("Release {}", tag));

    println!();
    println!("Creating release {}...", style(&tag).green());

    CmdBuilder::new("git")
        .args(["tag", "-a", &tag, "-m", &tag_message])
        .cwd(&ctx.repo)
        .run()?;

    // Push tag
    CmdBuilder::new("git")
        .args(["push", "origin", &tag])
        .cwd(&ctx.repo)
        .run()?;

    ctx.print_success(&format!("Released {}!", tag));

    Ok(())
}

/// Rollback to a previous version
pub fn rollback(ctx: &AppContext, version: &str) -> Result<()> {
    // Ensure gh CLI is available
    if !devkit_core::cmd_exists("gh") {
        return Err(anyhow!(
            "GitHub CLI (gh) is required for rollback. Install from: https://cli.github.com/"
        ));
    }

    ctx.print_header("Rollback");
    println!("Rolling back to {}...", style(version).cyan());

    // This would typically trigger a deployment workflow
    // For now, just show how to do it manually
    println!();
    println!("To rollback:");
    println!("  1. Trigger deployment workflow: gh workflow run deploy --ref {}", version);
    println!("  2. Or manually: git checkout {} && ./deploy.sh", version);

    Ok(())
}

fn run_preflight_checks(ctx: &AppContext) -> Result<()> {
    println!("Pre-release checks:");
    println!();

    let mut all_passed = true;

    // Check 1: On main/master branch
    let branch = CmdBuilder::new("git")
        .args(["branch", "--show-current"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;

    let current_branch = branch.stdout_string().trim().to_string();
    let on_release_branch = current_branch == "main" || current_branch == "master";

    if on_release_branch {
        println!("  {} On {} branch", style("✓").green(), current_branch);
    } else {
        println!(
            "  {} On {} branch (expected main/master)",
            style("✗").red(),
            current_branch
        );
        all_passed = false;
    }

    // Check 2: Working tree clean
    let status = CmdBuilder::new("git")
        .args(["status", "--porcelain"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;

    if status.stdout_string().trim().is_empty() {
        println!("  {} Working tree clean", style("✓").green());
    } else {
        println!("  {} Uncommitted changes", style("✗").red());
        all_passed = false;
    }

    // Check 3: Up to date with remote
    let _ = CmdBuilder::new("git")
        .args(["fetch", "origin"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture();

    let behind = CmdBuilder::new("git")
        .args(["rev-list", "--count", "HEAD..@{u}"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture();

    match behind {
        Ok(output) if output.code == 0 => {
            let count: i32 = output.stdout_string().trim().parse().unwrap_or(0);
            if count == 0 {
                println!("  {} Up to date with remote", style("✓").green());
            } else {
                println!(
                    "  {} Behind remote by {} commit(s)",
                    style("✗").red(),
                    count
                );
                all_passed = false;
            }
        }
        _ => println!("  {} Could not check remote status", style("?").yellow()),
    }

    println!();

    if !all_passed {
        return Err(anyhow!("Pre-release checks failed"));
    }

    Ok(())
}
