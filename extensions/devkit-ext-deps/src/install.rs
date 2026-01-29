//! Dependency installation logic

use anyhow::{Context, Result};
use std::process::Command;

use crate::detection::PackageInfo;

/// Install dependencies for a single package
pub fn install_package(package: &PackageInfo, quiet: bool) -> Result<()> {
    if !package.package_manager.is_available() {
        anyhow::bail!(
            "{} is not installed. Please install it first.",
            package.package_manager.name()
        );
    }

    let cmd_parts = package.package_manager.install_cmd();
    let mut cmd = Command::new(cmd_parts[0]);

    for arg in &cmd_parts[1..] {
        cmd.arg(arg);
    }

    cmd.current_dir(&package.path);

    if !quiet {
        println!(
            "  Installing {} dependencies for {}...",
            package.language.name(),
            package.name
        );
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to run {}", package.package_manager.name()))?;

    if !status.success() {
        anyhow::bail!(
            "Failed to install dependencies for {} (exit code: {:?})",
            package.name,
            status.code()
        );
    }

    Ok(())
}

/// Install dependencies for all packages that need them
pub fn install_all(packages: &[PackageInfo], quiet: bool) -> Result<()> {
    let needs_install: Vec<_> = packages.iter().filter(|p| p.needs_install).collect();

    if needs_install.is_empty() {
        if !quiet {
            println!("✓ All dependencies up to date");
        }
        return Ok(());
    }

    if !quiet {
        println!(
            "Installing dependencies for {} package(s)...",
            needs_install.len()
        );
    }

    for package in needs_install {
        install_package(package, quiet)?;
    }

    if !quiet {
        println!("✓ All dependencies installed");
    }

    Ok(())
}
