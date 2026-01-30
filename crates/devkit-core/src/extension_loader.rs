use std::path::Path;
use anyhow::Result;
use crate::{Extension, external_extension::ExternalExtension};

/// Discover and load all external extensions from directories
pub fn load_external_extensions(repo_root: &Path) -> Result<Vec<Box<dyn Extension>>> {
    let mut extensions = Vec::new();

    let ext_dir = repo_root.join(".dev/extensions");
    if !ext_dir.exists() {
        return Ok(extensions);
    }

    tracing::info!("Scanning for extensions in {}", ext_dir.display());

    // Find all subdirectories in the extensions directory
    for entry in std::fs::read_dir(&ext_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process directories
        if !path.is_dir() {
            continue;
        }

        // Try to load config.toml from this directory
        match ExternalExtension::load(&path) {
            Ok(ext) => {
                tracing::info!("✓ Loaded extension: {} from {}",
                    ext.name(), path.display());
                extensions.push(Box::new(ext) as Box<dyn Extension>);
            }
            Err(e) => {
                tracing::warn!("✗ Failed to load extension from {}: {}",
                    path.display(), e);
            }
        }
    }

    Ok(extensions)
}
