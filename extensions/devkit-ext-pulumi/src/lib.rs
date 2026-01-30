//! Pulumi extension for devkit
//!
//! Provides Pulumi infrastructure deployment operations.

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use devkit_tasks::CmdBuilder;

pub struct PulumiExtension;

impl Extension for PulumiExtension {
    fn name(&self) -> &str {
        "pulumi"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.pulumi
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "☁️  Pulumi - Preview".to_string(),
                group: None,
                handler: Box::new(|ctx| pulumi_preview(ctx, None).map_err(Into::into)),
            },
            MenuItem {
                label: "☁️  Pulumi - Deploy (Up)".to_string(),
                group: None,
                handler: Box::new(|ctx| pulumi_up(ctx, None, false).map_err(Into::into)),
            },
        ]
    }
}

/// Pulumi up (deploy infrastructure)
pub fn pulumi_up(ctx: &AppContext, stack: Option<&str>, yes: bool) -> Result<()> {
    if !devkit_core::cmd_exists("pulumi") {
        return Err(anyhow!(
            "Pulumi CLI not found. Install from: https://www.pulumi.com/docs/get-started/install/"
        ));
    }

    ctx.print_header("Deploying infrastructure with Pulumi");

    let mut args = vec!["up".to_string()];

    if let Some(s) = stack {
        args.push("--stack".to_string());
        args.push(s.to_string());
    }

    if yes {
        args.push("--yes".to_string());
    }

    let code = CmdBuilder::new("pulumi")
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("Pulumi up failed with code {}", code));
    }

    ctx.print_success("Infrastructure deployed");
    Ok(())
}

/// Pulumi preview (preview changes)
pub fn pulumi_preview(ctx: &AppContext, stack: Option<&str>) -> Result<()> {
    if !devkit_core::cmd_exists("pulumi") {
        return Err(anyhow!(
            "Pulumi CLI not found. Install from: https://www.pulumi.com/docs/get-started/install/"
        ));
    }

    ctx.print_header("Previewing infrastructure changes");

    let mut args = vec!["preview".to_string()];

    if let Some(s) = stack {
        args.push("--stack".to_string());
        args.push(s.to_string());
    }

    let code = CmdBuilder::new("pulumi").args(&args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("Pulumi preview failed with code {}", code));
    }

    Ok(())
}

/// Pulumi destroy (tear down infrastructure)
pub fn pulumi_destroy(ctx: &AppContext, stack: Option<&str>, yes: bool) -> Result<()> {
    if !devkit_core::cmd_exists("pulumi") {
        return Err(anyhow!(
            "Pulumi CLI not found. Install from: https://www.pulumi.com/docs/get-started/install/"
        ));
    }

    ctx.print_header("Destroying infrastructure with Pulumi");

    let mut args = vec!["destroy".to_string()];

    if let Some(s) = stack {
        args.push("--stack".to_string());
        args.push(s.to_string());
    }

    if yes {
        args.push("--yes".to_string());
    }

    let code = CmdBuilder::new("pulumi")
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 {
        return Err(anyhow!("Pulumi destroy failed with code {}", code));
    }

    ctx.print_success("Infrastructure destroyed");
    Ok(())
}

/// Pulumi stack select
pub fn pulumi_stack_select(ctx: &AppContext, stack: &str) -> Result<()> {
    if !devkit_core::cmd_exists("pulumi") {
        return Err(anyhow!(
            "Pulumi CLI not found. Install from: https://www.pulumi.com/docs/get-started/install/"
        ));
    }

    let code = CmdBuilder::new("pulumi")
        .args(["stack", "select", stack])
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to select stack"));
    }

    ctx.print_success(&format!("Selected stack: {}", stack));
    Ok(())
}

/// Check if this extension should be enabled
pub fn should_enable(_ctx: &devkit_core::AppContext) -> bool {
    // Enable if Pulumi CLI is available
    devkit_core::cmd_exists("pulumi")
}
