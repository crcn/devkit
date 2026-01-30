//! Build cache management extension
//!
//! Provides commands to clean, analyze, and manage build caches
//! across different build systems (cargo, npm, gradle, maven, etc.)

use anyhow::Result;
use devkit_core::{AppContext, Extension, MenuItem};
use humansize::{format_size, BINARY};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct CacheExtension;

impl Extension for CacheExtension {
    fn name(&self) -> &str {
        "cache"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        // Always available
        true
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🗑  Clean all build caches".to_string(),
                handler: Box::new(|ctx| clean_all(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "📊 Show cache statistics".to_string(),
                handler: Box::new(|ctx| show_stats(ctx).map_err(Into::into)),
            },
        ]
    }
}

#[derive(Debug)]
struct CacheInfo {
    name: String,
    path: PathBuf,
    size: u64,
    exists: bool,
}

/// Detect all cache locations in the project
fn detect_caches(ctx: &AppContext) -> Vec<CacheInfo> {
    let repo = &ctx.repo;
    let mut caches = Vec::new();

    // Rust/Cargo caches
    caches.push(CacheInfo {
        name: "Cargo target".to_string(),
        path: repo.join("target"),
        size: 0,
        exists: false,
    });

    // Node.js caches
    caches.push(CacheInfo {
        name: "node_modules".to_string(),
        path: repo.join("node_modules"),
        size: 0,
        exists: false,
    });

    // Find all package node_modules
    if let Ok(entries) = glob::glob(&format!("{}/**/node_modules", repo.display())) {
        for entry in entries.flatten() {
            if !entry.to_string_lossy().contains("/node_modules/") {
                caches.push(CacheInfo {
                    name: format!(
                        "node_modules ({})",
                        entry.parent().unwrap_or(&entry).display()
                    ),
                    path: entry,
                    size: 0,
                    exists: false,
                });
            }
        }
    }

    // Gradle cache
    caches.push(CacheInfo {
        name: "Gradle build".to_string(),
        path: repo.join("build"),
        size: 0,
        exists: false,
    });

    // Maven cache
    caches.push(CacheInfo {
        name: "Maven target".to_string(),
        path: repo.join("target"),
        size: 0,
        exists: false,
    });

    // Python caches
    caches.push(CacheInfo {
        name: "Python __pycache__".to_string(),
        path: repo.join("__pycache__"),
        size: 0,
        exists: false,
    });

    // Go cache
    caches.push(CacheInfo {
        name: "Go build".to_string(),
        path: dirs::cache_dir().unwrap_or_default().join("go-build"),
        size: 0,
        exists: false,
    });

    // Calculate sizes and check existence
    for cache in &mut caches {
        if cache.path.exists() {
            cache.exists = true;
            cache.size = calculate_dir_size(&cache.path);
        }
    }

    // Filter to only existing caches and deduplicate
    caches
        .into_iter()
        .filter(|c| c.exists)
        .fold(Vec::new(), |mut acc, cache| {
            if !acc.iter().any(|c: &CacheInfo| c.path == cache.path) {
                acc.push(cache);
            }
            acc
        })
}

/// Calculate total size of a directory
fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

/// Show cache statistics
pub fn show_stats(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Build Cache Statistics");
    println!();

    let caches = detect_caches(ctx);

    if caches.is_empty() {
        ctx.print_info("No build caches found");
        return Ok(());
    }

    let total_size: u64 = caches.iter().map(|c| c.size).sum();

    for cache in &caches {
        let size_str = format_size(cache.size, BINARY);
        println!("  {} - {}", cache.name, size_str);
    }

    println!();
    ctx.print_info(&format!(
        "Total cache size: {}",
        format_size(total_size, BINARY)
    ));

    Ok(())
}

/// Clean all detected caches
pub fn clean_all(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Cleaning Build Caches");
    println!();

    let caches = detect_caches(ctx);

    if caches.is_empty() {
        ctx.print_info("No build caches found");
        return Ok(());
    }

    let total_size: u64 = caches.iter().map(|c| c.size).sum();
    ctx.print_info(&format!(
        "Found {} caches ({} total)",
        caches.len(),
        format_size(total_size, BINARY)
    ));
    println!();

    for cache in &caches {
        let size_str = format_size(cache.size, BINARY);
        ctx.print_info(&format!("Removing {} ({})...", cache.name, size_str));

        if let Err(e) = fs::remove_dir_all(&cache.path) {
            ctx.print_warning(&format!("Failed to remove {}: {}", cache.name, e));
        } else {
            ctx.print_success(&format!("✓ Removed {}", cache.name));
        }
    }

    println!();
    ctx.print_success(&format!("✓ Freed {}", format_size(total_size, BINARY)));

    Ok(())
}

/// Clean specific cache by name
pub fn clean_cache(ctx: &AppContext, cache_name: &str) -> Result<()> {
    let caches = detect_caches(ctx);

    let cache = caches
        .iter()
        .find(|c| c.name.to_lowercase().contains(&cache_name.to_lowercase()))
        .ok_or_else(|| anyhow::anyhow!("Cache '{}' not found", cache_name))?;

    ctx.print_info(&format!(
        "Removing {} ({})...",
        cache.name,
        format_size(cache.size, BINARY)
    ));

    fs::remove_dir_all(&cache.path)?;
    ctx.print_success(&format!("✓ Freed {}", format_size(cache.size, BINARY)));

    Ok(())
}

/// Prune old cache entries (not implemented yet)
pub fn prune(_ctx: &AppContext, _max_age_days: u32) -> Result<()> {
    // TODO: Implement age-based pruning
    Ok(())
}
