//! Version update checking and notification
//!
//! Checks GitHub releases for new versions and notifies users.
//! Respects cache to avoid excessive API calls.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const GITHUB_REPO: &str = "crcn/devkit";
const CACHE_FILE: &str = "update_check.json";
const CHECK_INTERVAL_HOURS: u64 = 24;

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCache {
    last_check: u64,
    latest_version: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    prerelease: bool,
}

/// Check for updates and return latest version info if available
pub fn check_for_updates(force: bool) -> Result<Option<UpdateInfo>> {
    let cache_path = get_cache_path()?;

    // Check cache unless forced
    if !force {
        if let Some(cached) = read_cache(&cache_path)? {
            let now = current_timestamp();
            let elapsed_hours = (now - cached.last_check) / 3600;

            if elapsed_hours < CHECK_INTERVAL_HOURS {
                // Cache is still fresh
                let current = current_version();
                if version_is_newer(&cached.latest_version, current) {
                    return Ok(Some(UpdateInfo {
                        current_version: current.to_string(),
                        latest_version: cached.latest_version,
                        download_url: format!("https://github.com/{}/releases/latest", GITHUB_REPO),
                    }));
                }
                return Ok(None);
            }
        }
    }

    // Fetch latest release from GitHub
    let latest = fetch_latest_release()?;

    // Update cache
    let cache = UpdateCache {
        last_check: current_timestamp(),
        latest_version: latest.tag_name.clone(),
    };
    write_cache(&cache_path, &cache)?;

    // Compare versions
    let current = current_version();
    if version_is_newer(&latest.tag_name, current) {
        Ok(Some(UpdateInfo {
            current_version: current.to_string(),
            latest_version: latest.tag_name,
            download_url: latest.html_url,
        }))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub download_url: String,
}

fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

fn get_cache_path() -> Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Failed to get cache directory"))?;

    let devkit_cache = cache_dir.join("devkit");
    fs::create_dir_all(&devkit_cache)?;

    Ok(devkit_cache.join(CACHE_FILE))
}

fn read_cache(path: &PathBuf) -> Result<Option<UpdateCache>> {
    if !path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(path)?;
    let cache: UpdateCache = serde_json::from_str(&contents)?;
    Ok(Some(cache))
}

fn write_cache(path: &PathBuf, cache: &UpdateCache) -> Result<()> {
    let contents = serde_json::to_string_pretty(cache)?;
    fs::write(path, contents)?;
    Ok(())
}

fn fetch_latest_release() -> Result<GitHubRelease> {
    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    );

    let client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build();

    let response = client
        .get(&url)
        .set("User-Agent", &format!("devkit/{}", current_version()))
        .call()
        .map_err(|e| anyhow::anyhow!("Failed to fetch releases: {}", e))?;

    let release: GitHubRelease = response.into_json()?;
    Ok(release)
}

fn version_is_newer(latest: &str, current: &str) -> bool {
    // Remove 'v' prefix if present
    let latest = latest.trim_start_matches('v');
    let current = current.trim_start_matches('v');

    // Simple version comparison
    match (parse_version(latest), parse_version(current)) {
        (Some(l), Some(c)) => l > c,
        _ => false,
    }
}

fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let major = parts[0].parse().ok()?;
    let minor = parts[1].parse().ok()?;
    let patch = parts[2].parse().ok()?;

    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(version_is_newer("0.2.0", "0.1.0"));
        assert!(version_is_newer("1.0.0", "0.9.9"));
        assert!(version_is_newer("v0.2.0", "0.1.0"));
        assert!(!version_is_newer("0.1.0", "0.2.0"));
        assert!(!version_is_newer("0.1.0", "0.1.0"));
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_version("0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_version("invalid"), None);
    }
}
