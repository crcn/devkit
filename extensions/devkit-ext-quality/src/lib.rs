//! Code quality tools (fmt, lint, test)

use anyhow::Result;
use devkit_core::{AppContext, Extension, MenuItem};
use devkit_tasks::{run_cmd, print_results, CmdOptions};

pub struct QualityExtension;

impl Extension for QualityExtension {
    fn name(&self) -> &str {
        "quality"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.commands
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        use devkit_core::DevkitError;
        vec![
            MenuItem {
                label: "✨ Format (check)".to_string(),
                handler: Box::new(|ctx| fmt(ctx, false).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "✨ Format (fix)".to_string(),
                handler: Box::new(|ctx| fmt(ctx, true).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "✨ Lint (check)".to_string(),
                handler: Box::new(|ctx| lint(ctx, false).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "✨ Lint (fix)".to_string(),
                handler: Box::new(|ctx| lint(ctx, true).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "✨ Test".to_string(),
                handler: Box::new(|ctx| test(ctx).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "✨ Test (watch)".to_string(),
                handler: Box::new(|ctx| test_watch(ctx).map_err(DevkitError::from)),
            },
        ]
    }
}

pub fn fmt(ctx: &AppContext, fix: bool) -> Result<()> {
    let variant = if fix {
        Some("fix".to_string())
    } else {
        None
    };

    let opts = CmdOptions {
        parallel: false,
        variant,
        packages: vec![],
        capture: false,
    };

    ctx.print_header("Format");
    let results = run_cmd(ctx, "fmt", &opts)?;
    print_results(ctx, &results);

    if results.iter().any(|r| !r.success) {
        return Err(anyhow::anyhow!("Formatting failed"));
    }

    Ok(())
}

pub fn lint(ctx: &AppContext, fix: bool) -> Result<()> {
    let variant = if fix {
        Some("fix".to_string())
    } else {
        None
    };

    let opts = CmdOptions {
        parallel: false,
        variant,
        packages: vec![],
        capture: false,
    };

    ctx.print_header("Lint");
    let results = run_cmd(ctx, "lint", &opts)?;
    print_results(ctx, &results);

    if results.iter().any(|r| !r.success) {
        return Err(anyhow::anyhow!("Linting failed"));
    }

    Ok(())
}

pub fn test(ctx: &AppContext) -> Result<()> {
    let opts = CmdOptions {
        parallel: false,
        variant: None,
        packages: vec![],
        capture: false,
    };

    ctx.print_header("Test");
    let results = run_cmd(ctx, "test", &opts)?;
    print_results(ctx, &results);

    if results.iter().any(|r| !r.success) {
        return Err(anyhow::anyhow!("Tests failed"));
    }

    Ok(())
}

pub fn test_watch(ctx: &AppContext) -> Result<()> {
    let opts = CmdOptions {
        parallel: false,
        variant: Some("watch".to_string()),
        packages: vec![],
        capture: false,
    };

    ctx.print_header("Test (watch mode)");
    let results = run_cmd(ctx, "test", &opts)?;
    print_results(ctx, &results);

    Ok(())
}
