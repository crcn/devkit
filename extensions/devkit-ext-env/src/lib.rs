//! Environment variable management extension for devkit
//!
//! Provides environment variable loading, editing, and optional Pulumi ESC integration.

use anyhow::{anyhow, Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use devkit_tasks::CmdBuilder;
use std::fs;
use std::path::Path;

pub struct EnvExtension;

impl Extension for EnvExtension {
    fn name(&self) -> &str {
        "env"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        // Always available - env management is always useful
        true
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🔐 Env - Load .env".to_string(),
                handler: Box::new(|ctx| {
                    let env_file = ctx.repo.join(".env");
                    load_env(ctx, &env_file).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "🔐 Env - Load .env.local".to_string(),
                handler: Box::new(|ctx| {
                    let env_file = ctx.repo.join(".env.local");
                    load_env(ctx, &env_file).map_err(Into::into)
                }),
            },
        ]
    }
}

/// Load environment variables from a .env file
pub fn load_env(ctx: &AppContext, env_file: &Path) -> Result<()> {
    if !env_file.exists() {
        return Err(anyhow!(
            "Environment file not found: {}",
            env_file.display()
        ));
    }

    let content = fs::read_to_string(env_file)
        .with_context(|| format!("Failed to read {}", env_file.display()))?;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim_matches('"').trim_matches('\'');
            std::env::set_var(key, value);
        }
    }

    ctx.print_success(&format!("Loaded environment from {}", env_file.display()));
    Ok(())
}

/// Pull environment variables from Pulumi ESC (if available)
pub fn pull_env_from_esc(ctx: &AppContext, esc_path: &str, out_file: &Path) -> Result<()> {
    if !devkit_core::cmd_exists("esc") {
        return Err(anyhow!(
            "esc CLI not found. Install from: https://www.pulumi.com/docs/esc-cli/"
        ));
    }

    ctx.print_header(&format!("Pulling {} from ESC", esc_path));

    let out = CmdBuilder::new("esc")
        .args([
            "env",
            "get",
            esc_path,
            "--value",
            "dotenv",
            "--show-secrets",
        ])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()
        .with_context(|| format!("Failed to pull env from ESC: {}", esc_path))?;

    let target = ctx.repo.join(out_file);
    fs::write(&target, &out.stdout)
        .with_context(|| format!("Failed to write {}", target.display()))?;

    ctx.print_success(&format!("Wrote {}", target.display()));
    Ok(())
}

/// Set an environment variable in Pulumi ESC (if available)
pub fn set_env_in_esc(
    ctx: &AppContext,
    esc_path: &str,
    key: &str,
    value: &str,
    is_secret: bool,
) -> Result<()> {
    if !devkit_core::cmd_exists("esc") {
        return Err(anyhow!(
            "esc CLI not found. Install from: https://www.pulumi.com/docs/esc-cli/"
        ));
    }

    let path = format!("values.{}", key);
    let mut args = vec!["env", "set", esc_path, &path, value];

    if is_secret {
        args.push("--secret");
    }

    ctx.print_header(&format!(
        "Setting {} in {} (secret: {})",
        key, esc_path, is_secret
    ));

    let code = CmdBuilder::new("esc").args(args).cwd(&ctx.repo).run()?;

    if code != 0 {
        return Err(anyhow!("esc env set exited with code {}", code));
    }

    ctx.print_success(&format!("Successfully set {} in {}", key, esc_path));
    Ok(())
}

/// Check if this extension should be enabled
pub fn should_enable(_ctx: &devkit_core::AppContext) -> bool {
    // Always enable - env management is always useful
    true
}
