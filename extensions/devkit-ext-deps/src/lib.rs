//! Dependency detection and installation extension
//!
//! Automatically detects what dependencies each package needs and installs them.

use anyhow::Result;
use devkit_core::AppContext;

mod detection;
mod extension_impl;
mod install;

pub use detection::{Language, PackageInfo, PackageManager};
pub use extension_impl::DepsExtension;
pub use install::install_all;

/// Discover and analyze all packages in the workspace using glob patterns
pub fn discover_packages(ctx: &AppContext) -> Vec<PackageInfo> {
    let mut packages = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    // Use workspace patterns from config to find all packages
    for pattern in &ctx.config.global.workspaces.packages {
        let full_pattern = ctx.repo.join(pattern);
        let pattern_str = full_pattern.to_string_lossy();

        if let Ok(entries) = glob::glob(&pattern_str) {
            for entry in entries.flatten() {
                if !entry.is_dir() {
                    continue;
                }

                // Skip if we've already seen this path
                if !seen_paths.insert(entry.clone()) {
                    continue;
                }

                // Skip excluded packages
                let dir_name = entry
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if ctx.config.global.workspaces.exclude.contains(&dir_name.to_string()) {
                    continue;
                }

                // Try to detect package info
                if let Some(info) = PackageInfo::detect(&entry) {
                    packages.push(info);
                }
            }
        }
    }

    packages
}

/// Check and install dependencies for all packages
pub fn check_and_install(ctx: &AppContext) -> Result<()> {
    let packages = discover_packages(ctx);

    if packages.is_empty() {
        if !ctx.quiet {
            ctx.print_info("No packages with dependencies found");
        }
        return Ok(());
    }

    install_all(&packages, ctx.quiet)
}

/// Print a summary of discovered packages
pub fn print_summary(ctx: &AppContext) {
    let packages = discover_packages(ctx);

    if packages.is_empty() {
        println!("No packages found");
        return;
    }

    ctx.print_header("Discovered Packages");
    println!();

    for pkg in &packages {
        let status = if pkg.needs_install {
            "needs install"
        } else {
            "up to date"
        };

        println!(
            "  {} [{}] via {} - {}",
            pkg.name,
            pkg.language.name(),
            pkg.package_manager.name(),
            status
        );
    }

    println!();
}
