//! Error types for devkit

use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DevkitError>;

#[derive(Debug, Error)]
pub enum DevkitError {
    #[error("Failed to load config from {path}: {source}")]
    ConfigLoad {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },

    #[error("Failed to parse config file {path}: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Command '{cmd}' not found in package '{package}'\nAvailable commands: {available}")]
    CommandNotFound {
        cmd: String,
        package: String,
        available: String,
    },

    #[error("Package '{package}' not found\nAvailable packages: {available}")]
    PackageNotFound { package: String, available: String },

    #[error("Circular dependency detected: {cycle}\nPlease remove the circular dependency from your dev.toml files")]
    CircularDependency { cycle: String },

    #[error(
        "Invalid dependency reference: {dep}\nFormat should be 'package:command' or 'package'"
    )]
    InvalidDependency { dep: String },

    #[error("Invalid glob pattern: {pattern}\n{source}")]
    InvalidGlob {
        pattern: String,
        #[source]
        source: glob::PatternError,
    },

    #[error("Docker compose failed: {message}\nTry: {suggestion}")]
    DockerComposeFailed { message: String, suggestion: String },

    #[error("Command execution failed: {command}\n{output}")]
    CommandFailed { command: String, output: String },

    #[error("Repository root not found\nMake sure you're in a git repository or create a .dev/config.toml file")]
    RepoRootNotFound,

    #[error("Feature '{feature}' is not available in this project\n{hint}")]
    FeatureNotAvailable { feature: String, hint: String },

    #[error("{0}")]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Glob(#[from] glob::GlobError),
}

impl DevkitError {
    /// Create a ConfigLoad error with context
    pub fn config_load(path: PathBuf, source: anyhow::Error) -> Self {
        Self::ConfigLoad { path, source }
    }

    /// Create a ConfigParse error
    pub fn config_parse(path: PathBuf, source: toml::de::Error) -> Self {
        Self::ConfigParse { path, source }
    }

    /// Create a CommandNotFound error with suggestions
    pub fn command_not_found(cmd: String, package: String, available: Vec<String>) -> Self {
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available.join(", ")
        };
        Self::CommandNotFound {
            cmd,
            package,
            available,
        }
    }

    /// Create a PackageNotFound error with suggestions
    pub fn package_not_found(package: String, available: Vec<String>) -> Self {
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available.join(", ")
        };
        Self::PackageNotFound { package, available }
    }

    /// Create a DockerComposeFailed error with helpful suggestion
    pub fn docker_compose_failed(message: String) -> Self {
        let suggestion = if message.contains("not found") || message.contains("No such file") {
            "Make sure docker-compose.yml exists in your repository root".to_string()
        } else if message.contains("Cannot connect") {
            "Make sure Docker is running: docker info".to_string()
        } else {
            "Check docker-compose logs for more details".to_string()
        };
        Self::DockerComposeFailed {
            message,
            suggestion,
        }
    }

    /// Create a FeatureNotAvailable error with hint
    pub fn feature_not_available(feature: String, hint: String) -> Self {
        Self::FeatureNotAvailable { feature, hint }
    }
}
