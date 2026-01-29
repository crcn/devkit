//! Configuration system for devkit.
//!
//! Supports a distributed configuration model:
//! - `.dev/config.toml` - Global configuration shared across all packages
//! - `packages/*/dev.toml` - Package-specific configuration (optional)
//!
//! Package names are derived from existing configs:
//! - Rust packages: Cargo.toml `[package] name = "..."`
//! - JS packages: package.json `"name": "..."`
//!
//! Packages declare capabilities via TOML sections:
//! - `[database]` - Package has migrations/seeds
//! - `[mobile]` - Package is a mobile app
//! - `[cmd]` - Package commands

#![allow(dead_code)]

use crate::error::{DevkitError, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// Global Configuration (.dev/config.toml)
// =============================================================================

/// Global configuration shared across all packages
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct GlobalConfig {
    pub project: ProjectConfig,
    pub workspaces: WorkspacesConfig,
    pub git: GitConfig,
    pub environments: EnvironmentsConfig,
    pub services: ServicesConfig,
    pub urls: UrlsConfig,
    pub defaults: DefaultsConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "my-project".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct WorkspacesConfig {
    /// Glob patterns for package discovery
    #[serde(default = "default_packages_patterns")]
    pub packages: Vec<String>,
    /// Glob patterns for infra package discovery
    #[serde(default = "default_infra_patterns")]
    pub infra: Vec<String>,
    /// Packages to exclude
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Default for WorkspacesConfig {
    fn default() -> Self {
        Self {
            packages: default_packages_patterns(),
            infra: default_infra_patterns(),
            exclude: Vec::new(),
        }
    }
}

fn default_packages_patterns() -> Vec<String> {
    vec!["packages/*".to_string()]
}

fn default_infra_patterns() -> Vec<String> {
    vec!["infra/*".to_string()]
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GitConfig {
    /// Protected branches that require special handling
    #[serde(default = "default_protected_branches")]
    pub protected_branches: Vec<String>,
    /// Default base branch for PRs
    #[serde(default = "default_pr_base")]
    pub default_pr_base: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            protected_branches: default_protected_branches(),
            default_pr_base: default_pr_base(),
        }
    }
}

fn default_protected_branches() -> Vec<String> {
    vec!["main".to_string(), "master".to_string()]
}

fn default_pr_base() -> String {
    "main".to_string()
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct EnvironmentsConfig {
    /// Available environments
    #[serde(default = "default_environments")]
    pub available: Vec<String>,
    /// Default environment
    #[serde(default = "default_env")]
    pub default: String,
}

fn default_environments() -> Vec<String> {
    vec!["dev".to_string(), "prod".to_string()]
}

fn default_env() -> String {
    "dev".to_string()
}

/// Services configuration - maps service name to port
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ServicesConfig {
    /// Service ports keyed by service name
    #[serde(flatten)]
    pub ports: HashMap<String, u16>,
}

impl ServicesConfig {
    /// Get port for a service, with optional default
    pub fn get_port(&self, service: &str, default: u16) -> u16 {
        self.ports.get(service).copied().unwrap_or(default)
    }
}

/// Quick access URLs configuration
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct UrlsConfig {
    /// URL entries keyed by identifier
    #[serde(flatten)]
    pub entries: HashMap<String, UrlEntry>,
}

/// A quick access URL entry
#[derive(Debug, Deserialize, Clone)]
pub struct UrlEntry {
    /// Display label for the URL
    pub label: String,
    /// The URL to open
    pub url: String,
}

impl UrlsConfig {
    /// Get a URL entry by key
    pub fn get(&self, key: &str) -> Option<&UrlEntry> {
        self.entries.get(key)
    }

    /// Get all URL entries as (key, entry) pairs
    pub fn all(&self) -> impl Iterator<Item = (&String, &UrlEntry)> {
        self.entries.iter()
    }

    /// Check if any URLs are defined
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct DefaultsConfig {
    /// Default number of releases to list
    #[serde(default = "default_release_list_count")]
    pub release_list_count: u32,
}

fn default_release_list_count() -> u32 {
    5
}

/// Feature flags for kitchen sink CLI
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FeaturesConfig {
    pub docker: bool,
    pub database: bool,
    pub quality: bool,
    pub ci: bool,
    pub env: bool,
    pub deploy: bool,
    pub tunnel: bool,
    pub mobile: bool,
    pub benchmark: bool,
    pub git_workflows: bool,
    pub monitoring: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            docker: true,
            database: true,
            quality: true,
            ci: false,
            env: false,
            deploy: false,
            tunnel: false,
            mobile: false,
            benchmark: false,
            git_workflows: false,
            monitoring: false,
        }
    }
}

// =============================================================================
// Package Configuration (packages/*/dev.toml)
// =============================================================================

/// Package-specific configuration from dev.toml
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PackageToml {
    /// Database capability
    pub database: Option<DatabaseConfig>,
    /// Mobile capability
    pub mobile: Option<MobileConfig>,
    /// Package commands
    #[serde(default)]
    pub cmd: HashMap<String, CmdEntry>,
}

/// Database capability configuration
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Path to migrations directory (relative to package)
    pub migrations: String,
    /// Path to seed file (relative to package)
    pub seeds: Option<String>,
}

/// Mobile capability configuration
#[derive(Debug, Deserialize, Clone)]
pub struct MobileConfig {
    /// Pre-run scripts (relative to package)
    #[serde(default)]
    pub pre_run_scripts: Vec<String>,
    /// Startup timeout in seconds
    #[serde(default = "default_startup_timeout")]
    pub startup_timeout_secs: u32,
}

fn default_startup_timeout() -> u32 {
    300
}

// =============================================================================
// Command Configuration
// =============================================================================

/// Command entry - either a simple string or full config
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum CmdEntry {
    /// Simple command string
    Simple(String),
    /// Full command config with options
    Full(CmdConfig),
}

impl CmdEntry {
    /// Get the default command
    pub fn default_cmd(&self) -> &str {
        match self {
            CmdEntry::Simple(s) => s,
            CmdEntry::Full(c) => &c.default,
        }
    }

    /// Get a variant command by name (e.g., "fix", "watch")
    pub fn variant(&self, name: &str) -> &str {
        match self {
            CmdEntry::Simple(s) => s,
            CmdEntry::Full(c) => c
                .variants
                .get(name)
                .map(|s| s.as_str())
                .unwrap_or(&c.default),
        }
    }

    /// Get dependencies
    pub fn deps(&self) -> &[String] {
        match self {
            CmdEntry::Simple(_) => &[],
            CmdEntry::Full(c) => &c.deps,
        }
    }
}

/// Full command configuration
#[derive(Debug, Clone)]
pub struct CmdConfig {
    /// The default command to run
    pub default: String,
    /// Dependencies to run first (format: "package:cmd" or "package" for same cmd)
    pub deps: Vec<String>,
    /// Command variants (any other key becomes a variant)
    pub variants: HashMap<String, String>,
}

impl<'de> Deserialize<'de> for CmdConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map: HashMap<String, toml::Value> = HashMap::deserialize(deserializer)?;

        let default = map
            .remove("default")
            .and_then(|v| v.as_str().map(String::from))
            .ok_or_else(|| serde::de::Error::missing_field("default"))?;

        let deps = map
            .remove("deps")
            .map(|v| {
                v.as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let variants: HashMap<String, String> = map
            .into_iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
            .collect();

        Ok(CmdConfig {
            default,
            deps,
            variants,
        })
    }
}

/// Resolved package configuration with inferred values
#[derive(Debug, Default)]
pub struct PackageConfig {
    /// Package directory path
    pub path: PathBuf,
    /// Package directory name
    pub dir_name: String,
    /// Package name from Cargo.toml or package.json
    pub name: String,
    /// Database capability
    pub database: Option<DatabaseConfig>,
    /// Mobile capability
    pub mobile: Option<MobileConfig>,
    /// Package commands
    pub cmd: HashMap<String, CmdEntry>,
}

// =============================================================================
// Package Name Inference
// =============================================================================

/// Infer package name from Cargo.toml
fn infer_name_from_cargo_toml(package_path: &Path) -> Option<String> {
    let cargo_path = package_path.join("Cargo.toml");
    if !cargo_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&cargo_path).ok()?;
    let parsed: toml::Value = toml::from_str(&content).ok()?;

    parsed
        .get("package")?
        .get("name")?
        .as_str()
        .map(|s| s.to_string())
}

/// Infer package name from package.json
fn infer_name_from_package_json(package_path: &Path) -> Option<String> {
    let json_path = package_path.join("package.json");
    if !json_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&json_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;

    parsed.get("name")?.as_str().map(|s| {
        // Strip org prefix like "@org/app" -> "app"
        if let Some(stripped) = s.strip_prefix('@') {
            stripped.split('/').nth(1).unwrap_or(s).to_string()
        } else {
            s.to_string()
        }
    })
}

/// Infer package name from existing config files
fn infer_package_name(package_path: &Path, dir_name: &str) -> String {
    infer_name_from_cargo_toml(package_path)
        .or_else(|| infer_name_from_package_json(package_path))
        .unwrap_or_else(|| dir_name.to_string())
}

// =============================================================================
// Combined Configuration
// =============================================================================

/// Combined configuration from global and package configs
#[derive(Debug, Default)]
pub struct Config {
    /// Repository root path
    pub repo_root: PathBuf,
    /// Global configuration
    pub global: GlobalConfig,
    /// Package configurations keyed by package name
    pub packages: HashMap<String, PackageConfig>,
}

impl Config {
    /// Load configuration from the repository root
    pub fn load(repo_root: &Path) -> Result<Self> {
        let global = Self::load_global_config(repo_root)?;
        let packages = Self::discover_packages(repo_root, &global)?;

        Ok(Config {
            repo_root: repo_root.to_path_buf(),
            global,
            packages,
        })
    }

    /// Load global configuration from .dev/config.toml
    fn load_global_config(repo_root: &Path) -> Result<GlobalConfig> {
        let config_path = repo_root.join(".dev/config.toml");

        if !config_path.exists() {
            return Ok(GlobalConfig::default());
        }

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| DevkitError::config_load(config_path.clone(), e.into()))?;

        toml::from_str(&content)
            .map_err(|e| DevkitError::config_parse(config_path, e))
    }

    /// Discover packages and load their configurations
    fn discover_packages(
        repo_root: &Path,
        global: &GlobalConfig,
    ) -> Result<HashMap<String, PackageConfig>> {
        let mut packages = HashMap::new();

        for pattern in &global.workspaces.packages {
            let full_pattern = repo_root.join(pattern);
            let pattern_str = full_pattern.to_string_lossy();

            let entries = glob::glob(&pattern_str)
                .map_err(|e| DevkitError::InvalidGlob {
                    pattern: pattern.clone(),
                    source: e,
                })?;

            for entry in entries {
                let path = entry?;
                if !path.is_dir() {
                    continue;
                }

                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
                    .to_string();

                if global.workspaces.exclude.contains(&name) {
                    continue;
                }

                let config = Self::load_package_config(&path, &name)?;
                packages.insert(config.name.clone(), config);
            }
        }

        Ok(packages)
    }

    /// Load package configuration
    fn load_package_config(package_path: &Path, dir_name: &str) -> Result<PackageConfig> {
        let name = infer_package_name(package_path, dir_name);

        let config_path = package_path.join("dev.toml");
        let toml_config: PackageToml = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| DevkitError::config_load(config_path.clone(), e.into()))?;
            toml::from_str(&content)
                .map_err(|e| DevkitError::config_parse(config_path, e))?
        } else {
            PackageToml::default()
        };

        Ok(PackageConfig {
            path: package_path.to_path_buf(),
            dir_name: dir_name.to_string(),
            name,
            database: toml_config.database,
            mobile: toml_config.mobile,
            cmd: toml_config.cmd,
        })
    }

    /// Find all packages with database capability
    pub fn database_packages(&self) -> Vec<(&str, &DatabaseConfig)> {
        self.packages
            .iter()
            .filter_map(|(name, pkg)| pkg.database.as_ref().map(|db| (name.as_str(), db)))
            .collect()
    }

    /// Find all packages that have a specific command
    pub fn packages_with_cmd(&self, cmd_name: &str) -> Vec<(&str, &PackageConfig, &CmdEntry)> {
        self.packages
            .iter()
            .filter_map(|(name, pkg)| pkg.cmd.get(cmd_name).map(|cmd| (name.as_str(), pkg, cmd)))
            .collect()
    }

    /// Get a specific command from a package
    pub fn get_cmd(&self, pkg_name: &str, cmd_name: &str) -> Option<&CmdEntry> {
        self.packages
            .get(pkg_name)
            .and_then(|pkg| pkg.cmd.get(cmd_name))
    }

    /// Get a specific package configuration
    pub fn get_package(&self, name: &str) -> Option<&PackageConfig> {
        self.packages.get(name)
    }
}
