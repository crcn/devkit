//! Pre-commit checks (fmt + lint + typecheck)

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::{run_cmd, CmdOptions};

/// Run pre-commit checks (fmt + lint + typecheck)
pub fn run_check(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Running pre-commit checks");

    let mut had_errors = false;

    // Step 1: Format check
    if !ctx.quiet {
        println!();
        println!("=== Format Check ===");
    }
    if let Err(e) = crate::run_fmt(ctx, false) {
        ctx.print_warning(&format!("Format check failed: {}", e));
        had_errors = true;
    }

    // Step 2: Lint check
    if !ctx.quiet {
        println!();
        println!("=== Lint Check ===");
    }
    if let Err(e) = crate::run_lint(ctx, false) {
        ctx.print_warning(&format!("Lint check failed: {}", e));
        had_errors = true;
    }

    // Step 3: Type check using cmd system (handles dependencies automatically)
    if !ctx.quiet {
        println!();
        println!("=== Type Check ===");
    }

    let packages_with_typecheck = ctx.config.packages_with_cmd("typecheck");

    if packages_with_typecheck.is_empty() {
        if !ctx.quiet {
            println!("[typecheck] No packages define typecheck command");
        }
    } else {
        let opts = CmdOptions {
            parallel: false,
            variant: None,
            packages: vec![],
            capture: false,
        };
        let results = run_cmd(ctx, "typecheck", &opts)?;
        if results.iter().any(|r| !r.success) {
            ctx.print_warning("typecheck failed");
            had_errors = true;
        }
    }

    println!();
    if had_errors {
        return Err(anyhow!("Pre-commit checks failed"));
    }

    ctx.print_success("All pre-commit checks passed!");
    Ok(())
}
