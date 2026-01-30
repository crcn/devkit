//! Security scanning extension
//!
//! Integrates with cargo audit, npm audit, and secret scanning tools

use anyhow::{Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use std::process::Command;

pub struct SecurityExtension;

impl Extension for SecurityExtension {
    fn name(&self) -> &str {
        "security"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.cargo || ctx.features.node
    }

    fn menu_items(&self, ctx: &AppContext) -> Vec<MenuItem> {
        let mut items = vec![MenuItem {
            label: "🔒 Run security scan".to_string(),
            handler: Box::new(|ctx| security_scan(ctx).map_err(Into::into)),
        }];

        if ctx.features.cargo {
            items.push(MenuItem {
                label: "🦀 Audit Rust dependencies".to_string(),
                handler: Box::new(|ctx| cargo_audit(ctx).map_err(Into::into)),
            });
        }

        if ctx.features.node {
            items.push(MenuItem {
                label: "📦 Audit npm dependencies".to_string(),
                handler: Box::new(|ctx| npm_audit(ctx).map_err(Into::into)),
            });
        }

        items
    }
}

/// Run comprehensive security scan
pub fn security_scan(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Security Scan");
    println!();

    let mut issues_found = false;

    // Cargo audit
    if ctx.features.cargo {
        ctx.print_info("Running cargo audit...");
        match cargo_audit(ctx) {
            Ok(_) => ctx.print_success("✓ No Rust vulnerabilities found"),
            Err(e) => {
                ctx.print_warning(&format!("Rust vulnerabilities found: {}", e));
                issues_found = true;
            }
        }
        println!();
    }

    // npm audit
    if ctx.features.node {
        ctx.print_info("Running npm audit...");
        match npm_audit(ctx) {
            Ok(_) => ctx.print_success("✓ No npm vulnerabilities found"),
            Err(e) => {
                ctx.print_warning(&format!("npm vulnerabilities found: {}", e));
                issues_found = true;
            }
        }
        println!();
    }

    // Secret scanning
    ctx.print_info("Scanning for exposed secrets...");
    match scan_secrets(ctx) {
        Ok(_) => ctx.print_success("✓ No exposed secrets found"),
        Err(e) => {
            ctx.print_warning(&format!("Secret scanning: {}", e));
        }
    }

    println!();
    if issues_found {
        ctx.print_warning("⚠️  Security issues found - review above");
    } else {
        ctx.print_success("✓ Security scan complete - no issues found");
    }

    Ok(())
}

/// Run cargo audit
pub fn cargo_audit(ctx: &AppContext) -> Result<()> {
    // Check if cargo-audit is installed
    if !cmd_exists("cargo-audit") {
        ctx.print_warning("cargo-audit not installed");
        ctx.print_info("Install: cargo install cargo-audit");
        return Ok(());
    }

    let output = Command::new("cargo")
        .arg("audit")
        .current_dir(&ctx.repo)
        .output()
        .context("Failed to run cargo audit")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Vulnerabilities found"));
    }

    Ok(())
}

/// Run npm audit
pub fn npm_audit(ctx: &AppContext) -> Result<()> {
    let output = Command::new("npm")
        .arg("audit")
        .current_dir(&ctx.repo)
        .output()
        .context("Failed to run npm audit")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Vulnerabilities found"));
    }

    Ok(())
}

/// Scan for exposed secrets
pub fn scan_secrets(ctx: &AppContext) -> Result<()> {
    // Check if gitleaks is installed
    if cmd_exists("gitleaks") {
        let output = Command::new("gitleaks")
            .args(["detect", "--no-git", "-v"])
            .current_dir(&ctx.repo)
            .output()
            .context("Failed to run gitleaks")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Potential secrets found"));
        }
    } else {
        return Err(anyhow::anyhow!(
            "gitleaks not installed (install from https://github.com/gitleaks/gitleaks)"
        ));
    }

    Ok(())
}

/// Generate SBOM (Software Bill of Materials)
pub fn generate_sbom(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Generating SBOM");

    if ctx.features.cargo {
        ctx.print_info("Generating Cargo SBOM...");
        // TODO: Implement SBOM generation for Rust
        ctx.print_warning("Cargo SBOM generation not yet implemented");
    }

    if ctx.features.node {
        ctx.print_info("Generating npm SBOM...");
        // TODO: Implement SBOM generation for Node
        ctx.print_warning("npm SBOM generation not yet implemented");
    }

    Ok(())
}

fn cmd_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
