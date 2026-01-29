//! Workflow management (logs, watch, trigger, rerun, cancel)

use anyhow::{anyhow, Result};
use console::style;
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// View logs for a specific workflow run
pub fn ci_logs(ctx: &AppContext, run_id: &str) -> Result<()> {
    ctx.print_header(&format!("Logs for run {}", run_id));

    let code = CmdBuilder::new("gh")
        .args(["run", "view", run_id, "--log"])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to fetch logs"));
    }

    Ok(())
}

/// Watch a workflow run in progress
pub fn ci_watch(ctx: &AppContext, run_id: Option<&str>) -> Result<()> {
    let run = match run_id {
        Some(id) => id.to_string(),
        None => {
            // Find the most recent in-progress run
            let output = CmdBuilder::new("gh")
                .args([
                    "run",
                    "list",
                    "--status",
                    "in_progress",
                    "--limit",
                    "1",
                    "--json",
                    "databaseId",
                ])
                .cwd(&ctx.repo)
                .capture_stdout()
                .run_capture()?;

            let runs: Vec<serde_json::Value> =
                serde_json::from_str(&output.stdout_string()).unwrap_or_default();

            if runs.is_empty() {
                // No in-progress runs, watch the most recent
                let output = CmdBuilder::new("gh")
                    .args(["run", "list", "--limit", "1", "--json", "databaseId"])
                    .cwd(&ctx.repo)
                    .capture_stdout()
                    .run_capture()?;

                let runs: Vec<serde_json::Value> =
                    serde_json::from_str(&output.stdout_string()).unwrap_or_default();

                runs.first()
                    .and_then(|r| r["databaseId"].as_i64())
                    .map(|id| id.to_string())
                    .ok_or_else(|| anyhow!("No workflow runs found"))?
            } else {
                runs[0]["databaseId"]
                    .as_i64()
                    .map(|id| id.to_string())
                    .ok_or_else(|| anyhow!("Failed to get run ID"))?
            }
        }
    };

    ctx.print_header(&format!("Watching run {}", run));
    println!("Press Ctrl+C to stop watching");
    println!();

    let code = CmdBuilder::new("gh")
        .args(["run", "watch", &run])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Watch failed"));
    }

    Ok(())
}

/// Trigger a workflow manually
pub fn ci_trigger(ctx: &AppContext, workflow: &str, branch: Option<&str>) -> Result<()> {
    // Get branch
    let target_branch = match branch {
        Some(b) => b.to_string(),
        None => {
            let output = CmdBuilder::new("git")
                .args(["branch", "--show-current"])
                .cwd(&ctx.repo)
                .capture_stdout()
                .run_capture()?;
            output.stdout_string().trim().to_string()
        }
    };

    ctx.print_header(&format!("Triggering workflow: {}", workflow));
    println!("Branch: {}", style(&target_branch).cyan());

    let code = CmdBuilder::new("gh")
        .args(["workflow", "run", workflow, "--ref", &target_branch])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to trigger workflow"));
    }

    ctx.print_success(&format!(
        "Workflow '{}' triggered on branch '{}'",
        workflow, target_branch
    ));

    Ok(())
}

/// Re-run a failed workflow
pub fn ci_rerun(ctx: &AppContext, run_id: &str, failed_only: bool) -> Result<()> {
    ctx.print_header(&format!("Re-running workflow {}", run_id));

    let mut args = vec!["run".to_string(), "rerun".to_string(), run_id.to_string()];
    if failed_only {
        args.push("--failed".to_string());
    }

    let code = CmdBuilder::new("gh").args(&args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("Failed to re-run workflow"));
    }

    ctx.print_success("Workflow re-run triggered");
    Ok(())
}

/// Cancel a running workflow
pub fn ci_cancel(ctx: &AppContext, run_id: &str) -> Result<()> {
    let code = CmdBuilder::new("gh")
        .args(["run", "cancel", run_id])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to cancel workflow"));
    }

    ctx.print_success("Workflow cancelled");
    Ok(())
}
