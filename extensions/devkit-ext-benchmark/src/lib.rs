//! Benchmark extension for devkit
//!
//! Provides performance benchmarking for Rust and JavaScript projects.

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use devkit_tasks::CmdBuilder;

pub struct BenchmarkExtension;

impl Extension for BenchmarkExtension {
    fn name(&self) -> &str {
        "benchmark"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.cargo || ctx.features.node
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "⚡ Benchmark - Run All".to_string(),
                handler: Box::new(|ctx| {
                    run_benchmarks(ctx, &BenchmarkOptions::default()).map_err(Into::into)
                }),
            },
        ]
    }
}

pub struct BenchmarkOptions {
    /// Specific benchmark filter
    pub filter: Option<String>,
    /// Save baseline for comparison
    pub baseline: Option<String>,
    /// Compare against baseline
    pub compare: Option<String>,
}

impl Default for BenchmarkOptions {
    fn default() -> Self {
        Self {
            filter: None,
            baseline: None,
            compare: None,
        }
    }
}

/// Run benchmarks
pub fn run_benchmarks(ctx: &AppContext, opts: &BenchmarkOptions) -> Result<()> {
    if ctx.features.cargo {
        run_cargo_benchmarks(ctx, opts)
    } else if ctx.features.node {
        run_node_benchmarks(ctx, opts)
    } else {
        Err(anyhow!(
            "No benchmark framework detected. Configure benchmark command in config."
        ))
    }
}

fn run_cargo_benchmarks(ctx: &AppContext, opts: &BenchmarkOptions) -> Result<()> {
    ctx.print_header("Running Rust benchmarks");

    let mut args = vec!["bench".to_string()];

    if let Some(ref filter) = opts.filter {
        args.push(filter.clone());
    }

    // Check for criterion support
    if opts.baseline.is_some() || opts.compare.is_some() {
        if let Some(ref baseline) = opts.baseline {
            args.push("--save-baseline".to_string());
            args.push(baseline.clone());
        }
        if let Some(ref compare) = opts.compare {
            args.push("--baseline".to_string());
            args.push(compare.clone());
        }
    }

    let code = CmdBuilder::new("cargo")
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("Benchmarks failed with code {}", code));
    }

    Ok(())
}

fn run_node_benchmarks(ctx: &AppContext, opts: &BenchmarkOptions) -> Result<()> {
    ctx.print_header("Running JavaScript benchmarks");

    // Try to find benchmark script in package.json
    let mut args = vec!["run".to_string(), "bench".to_string()];

    if let Some(ref filter) = opts.filter {
        args.push("--".to_string());
        args.push(filter.clone());
    }

    let npm = if devkit_core::cmd_exists("npm") {
        "npm"
    } else if devkit_core::cmd_exists("yarn") {
        "yarn"
    } else {
        return Err(anyhow!("No package manager found (npm/yarn)"));
    };

    let code = CmdBuilder::new(npm).args(&args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("Benchmarks failed with code {}", code));
    }

    Ok(())
}

/// Check if this extension should be enabled
pub fn should_enable(ctx: &devkit_core::AppContext) -> bool {
    // Enable if we have Rust or Node projects
    ctx.features.cargo || ctx.features.node
}
