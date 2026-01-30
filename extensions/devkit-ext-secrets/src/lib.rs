//! Secrets management extension
//!
//! Supports multiple providers: AWS Secrets Manager, 1Password, Doppler, environment files

use anyhow::{Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub struct SecretsExtension;

impl Extension for SecretsExtension {
    fn name(&self) -> &str {
        "secrets"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        // Available if any secrets tool is installed
        cmd_exists("aws") || cmd_exists("op") || cmd_exists("doppler")
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🔐 Pull secrets to .env".to_string(),
                handler: Box::new(|ctx| pull_secrets(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "📋 List available secrets".to_string(),
                handler: Box::new(|ctx| list_secrets(ctx).map_err(Into::into)),
            },
        ]
    }
}

fn cmd_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Pull secrets from configured provider
pub fn pull_secrets(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Pulling Secrets");

    // Check for 1Password CLI
    if cmd_exists("op") {
        return pull_from_1password(ctx);
    }

    // Check for Doppler
    if cmd_exists("doppler") {
        return pull_from_doppler(ctx);
    }

    // Check for AWS CLI
    if cmd_exists("aws") {
        return pull_from_aws(ctx);
    }

    ctx.print_warning("No secrets provider found");
    ctx.print_info("Install: aws-cli, 1password-cli, or doppler");

    Ok(())
}

fn pull_from_1password(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Pulling from 1Password...");

    // Example: op run --env-file=.env -- env | grep -v "^#" > .env.local
    let output = Command::new("op")
        .args(["run", "--", "env"])
        .output()
        .context("Failed to run 1Password CLI")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "1Password CLI failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let env_path = ctx.repo.join(".env.local");
    fs::write(&env_path, &output.stdout).context("Failed to write .env.local")?;

    ctx.print_success(&format!("✓ Secrets saved to {}", env_path.display()));

    Ok(())
}

fn pull_from_doppler(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Pulling from Doppler...");

    let output = Command::new("doppler")
        .args(["secrets", "download", "--no-file", "--format", "env"])
        .output()
        .context("Failed to run Doppler CLI")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Doppler CLI failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let env_path = ctx.repo.join(".env.local");
    fs::write(&env_path, &output.stdout).context("Failed to write .env.local")?;

    ctx.print_success(&format!("✓ Secrets saved to {}", env_path.display()));

    Ok(())
}

fn pull_from_aws(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Pulling from AWS Secrets Manager...");
    ctx.print_warning("AWS integration requires additional configuration");
    ctx.print_info("Set AWS_SECRET_NAME environment variable");

    let secret_name = std::env::var("AWS_SECRET_NAME").context("AWS_SECRET_NAME not set")?;

    let output = Command::new("aws")
        .args([
            "secretsmanager",
            "get-secret-value",
            "--secret-id",
            &secret_name,
            "--query",
            "SecretString",
            "--output",
            "text",
        ])
        .output()
        .context("Failed to run AWS CLI")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "AWS CLI failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let secrets_json = String::from_utf8_lossy(&output.stdout);
    let secrets: HashMap<String, String> =
        serde_json::from_str(&secrets_json).context("Failed to parse secrets JSON")?;

    let mut env_content = String::new();
    for (key, value) in secrets {
        env_content.push_str(&format!("{}={}\n", key, value));
    }

    let env_path = ctx.repo.join(".env.local");
    fs::write(&env_path, env_content).context("Failed to write .env.local")?;

    ctx.print_success(&format!("✓ Secrets saved to {}", env_path.display()));

    Ok(())
}

/// List available secrets
pub fn list_secrets(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Available Secrets");

    if cmd_exists("op") {
        list_1password_secrets(ctx)?;
    } else if cmd_exists("doppler") {
        list_doppler_secrets(ctx)?;
    } else if cmd_exists("aws") {
        list_aws_secrets(ctx)?;
    } else {
        ctx.print_warning("No secrets provider found");
    }

    Ok(())
}

fn list_1password_secrets(ctx: &AppContext) -> Result<()> {
    ctx.print_info("1Password vaults:");

    let output = Command::new("op")
        .args(["vault", "list", "--format", "json"])
        .output()
        .context("Failed to list 1Password vaults")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

fn list_doppler_secrets(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Doppler secrets:");

    let output = Command::new("doppler")
        .args(["secrets"])
        .output()
        .context("Failed to list Doppler secrets")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

fn list_aws_secrets(ctx: &AppContext) -> Result<()> {
    ctx.print_info("AWS secrets:");

    let output = Command::new("aws")
        .args(["secretsmanager", "list-secrets"])
        .output()
        .context("Failed to list AWS secrets")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}
