//! Git status checking

use anyhow::Result;
use console::style;
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Show comprehensive git status
pub fn git_status(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Git Status");

    // Current branch
    let branch_output = CmdBuilder::new("git")
        .args(["branch", "--show-current"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;
    let current_branch = branch_output.stdout_string().trim().to_string();
    println!("Branch: {}", style(&current_branch).cyan());
    println!();

    // Working tree status
    println!("{}", style("Working tree:").bold());
    let status_code = CmdBuilder::new("git")
        .args(["status", "--short"])
        .cwd(&ctx.repo)
        .run()?;

    if status_code == 0 {
        println!();
    }

    // Recent commits
    println!("{}", style("Recent commits:").bold());
    CmdBuilder::new("git")
        .args([
            "log",
            "--oneline",
            "--no-decorate",
            "-n",
            "5",
        ])
        .cwd(&ctx.repo)
        .run()?;

    println!();

    // Remote status
    if is_tracking_remote(ctx)? {
        println!("{}", style("Remote status:").bold());

        // Fetch to get latest
        let _ = CmdBuilder::new("git")
            .args(["fetch", "origin"])
            .cwd(&ctx.repo)
            .capture_stdout()
            .run_capture();

        // Check if ahead/behind
        let ahead = CmdBuilder::new("git")
            .args(["rev-list", "--count", "@{u}..HEAD"])
            .cwd(&ctx.repo)
            .capture_stdout()
            .run_capture()?;

        let behind = CmdBuilder::new("git")
            .args(["rev-list", "--count", "HEAD..@{u}"])
            .cwd(&ctx.repo)
            .capture_stdout()
            .run_capture()?;

        let ahead_count: i32 = ahead.stdout_string().trim().parse().unwrap_or(0);
        let behind_count: i32 = behind.stdout_string().trim().parse().unwrap_or(0);

        if ahead_count == 0 && behind_count == 0 {
            println!("  {} Up to date with remote", style("✓").green());
        } else {
            if ahead_count > 0 {
                println!("  {} Ahead by {} commit(s)", style("↑").cyan(), ahead_count);
            }
            if behind_count > 0 {
                println!("  {} Behind by {} commit(s)", style("↓").yellow(), behind_count);
            }
        }
    } else {
        println!("{}", style("No remote tracking branch").dim());
    }

    Ok(())
}

fn is_tracking_remote(ctx: &AppContext) -> Result<bool> {
    let result = CmdBuilder::new("git")
        .args(["rev-parse", "--abbrev-ref", "@{u}"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture();

    Ok(result.is_ok() && result.unwrap().code == 0)
}
