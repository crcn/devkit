//! Shared utility functions

use crate::error::{DevkitError, Result};
use anyhow::Context;
use std::env;
use std::path::PathBuf;
use which::which;

/// Get the repository root path from REPO_ROOT env var or infer from CARGO_MANIFEST_DIR
pub fn repo_root() -> Result<PathBuf> {
    if let Ok(v) = env::var("REPO_ROOT") {
        let p = PathBuf::from(v);
        if p.exists() {
            return Ok(p);
        }
    }

    // Try to find .git directory walking up from current dir
    let current = env::current_dir()?;
    let mut path = current.as_path();

    loop {
        if path.join(".git").exists() || path.join(".dev").exists() {
            return Ok(path.to_path_buf());
        }

        match path.parent() {
            Some(parent) => path = parent,
            None => break,
        }
    }

    // No repo root found
    Err(DevkitError::RepoRootNotFound)
}

/// Check if a command exists in PATH
pub fn cmd_exists(name: &str) -> bool {
    which(name).is_ok()
}

/// Check if docker or docker-compose is available
pub fn docker_available() -> bool {
    cmd_exists("docker") || cmd_exists("docker-compose")
}

/// Ensure docker is available, returning an error if not
pub fn ensure_docker() -> Result<()> {
    if !docker_available() {
        return Err(DevkitError::feature_not_available(
            "docker".to_string(),
            "Install Docker from https://docker.com".to_string(),
        ));
    }
    Ok(())
}

/// Ensure cargo is available, returning an error if not
pub fn ensure_cargo() -> Result<()> {
    if !cmd_exists("cargo") {
        return Err(DevkitError::feature_not_available(
            "cargo".to_string(),
            "Install Rust from https://rustup.rs".to_string(),
        ));
    }
    Ok(())
}

/// Open a URL in the default browser
pub fn open_in_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .with_context(|| format!("failed to open {url} in browser"))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .with_context(|| format!("failed to open {url} in browser"))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()
            .with_context(|| format!("failed to open {url} in browser"))?;
    }
    Ok(())
}

/// Get docker compose program and base args
pub fn docker_compose_program() -> Result<(String, Vec<String>)> {
    if cmd_exists("docker") {
        return Ok(("docker".to_string(), vec!["compose".to_string()]));
    }
    if cmd_exists("docker-compose") {
        return Ok(("docker-compose".to_string(), vec![]));
    }
    Err(DevkitError::feature_not_available(
        "docker-compose".to_string(),
        "Install Docker Compose from https://docs.docker.com/compose/install/".to_string(),
    ))
}
