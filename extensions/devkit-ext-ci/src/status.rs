//! CI status checking

use anyhow::{anyhow, Result};
use console::style;
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Show current CI status for the repository
pub fn ci_status(ctx: &AppContext, protected_branches: Option<Vec<String>>) -> Result<()> {
    ctx.print_header("CI/CD Status");

    // Get current branch
    let branch_output = CmdBuilder::new("git")
        .args(["branch", "--show-current"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;
    let current_branch = branch_output.stdout_string().trim().to_string();

    println!("Branch: {}", style(&current_branch).cyan());
    println!();

    // Show recent runs for this branch
    println!("{}", style("Recent runs on this branch:").bold());
    let code = CmdBuilder::new("gh")
        .args(["run", "list", "--branch", &current_branch, "--limit", "5"])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to fetch CI status"));
    }

    println!();

    // Show PR checks if on a feature branch (not a protected branch)
    let protected: Vec<String> = protected_branches
        .unwrap_or_else(|| vec!["main".to_string(), "master".to_string(), "dev".to_string()]);

    if !protected.iter().any(|b| b == &current_branch) {
        println!("{}", style("PR checks:").bold());
        let _ = CmdBuilder::new("gh")
            .args(["pr", "checks"])
            .cwd(&ctx.repo)
            .run();
    }

    Ok(())
}

/// List recent workflow runs
pub fn ci_runs(ctx: &AppContext, limit: u32, workflow: Option<&str>) -> Result<()> {
    ctx.print_header("Recent Workflow Runs");

    let mut args = vec![
        "run".to_string(),
        "list".to_string(),
        "--limit".to_string(),
        limit.to_string(),
    ];

    if let Some(wf) = workflow {
        args.push("--workflow".to_string());
        args.push(wf.to_string());
    }

    let code = CmdBuilder::new("gh").args(&args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("Failed to list workflow runs"));
    }

    Ok(())
}
