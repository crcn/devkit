//! Feature detection - automatically discover what's available in the project

use crate::utils::{cmd_exists, docker_available};
use crate::Config;
use std::path::Path;

/// Detected features available in the current project
#[derive(Debug, Default)]
pub struct Features {
    pub docker: bool,
    pub database: bool,
    pub git: bool,
    pub cargo: bool,
    pub node: bool,
    pub github_actions: bool,
    pub mobile: bool,
    pub commands: bool, // Has packages with [cmd] sections
    pub pulumi: bool,
    pub test: bool,
}

impl Features {
    /// Detect features based on the repository and config
    pub fn detect(repo_root: &Path, config: &Config) -> Self {
        Features {
            docker: Self::has_docker(repo_root),
            database: Self::has_database(config),
            git: Self::has_git(repo_root),
            cargo: cmd_exists("cargo"),
            node: Self::has_node(repo_root, config),
            github_actions: Self::has_github_actions(repo_root),
            mobile: Self::has_mobile(config),
            commands: Self::has_commands(config),
            pulumi: Self::has_pulumi(repo_root),
            test: Self::has_tests(repo_root, config),
        }
    }

    fn has_docker(repo_root: &Path) -> bool {
        // Check if docker is installed and if docker-compose.yml exists
        docker_available()
            && (repo_root.join("docker-compose.yml").exists()
                || repo_root.join("docker-compose.yaml").exists()
                || repo_root.join("compose.yml").exists()
                || repo_root.join("compose.yaml").exists())
    }

    fn has_database(config: &Config) -> bool {
        // Check if any package has database capability
        !config.database_packages().is_empty()
    }

    fn has_git(repo_root: &Path) -> bool {
        repo_root.join(".git").exists()
    }

    fn has_node(repo_root: &Path, config: &Config) -> bool {
        // Check for any package.json files
        repo_root.join("package.json").exists()
            || config
                .packages
                .values()
                .any(|pkg| pkg.path.join("package.json").exists())
    }

    fn has_github_actions(repo_root: &Path) -> bool {
        repo_root.join(".github/workflows").exists()
    }

    fn has_mobile(config: &Config) -> bool {
        // Check if any package has mobile capability
        config.packages.values().any(|pkg| pkg.mobile.is_some())
    }

    fn has_commands(config: &Config) -> bool {
        // Check if any package defines commands
        config.packages.values().any(|pkg| !pkg.cmd.is_empty())
    }

    fn has_pulumi(repo_root: &Path) -> bool {
        // Check if pulumi CLI is installed AND Pulumi project files exist
        cmd_exists("pulumi")
            && (repo_root.join("Pulumi.yaml").exists() ||
            repo_root.join("Pulumi.yml").exists() ||
            // Check for any Pulumi stack files
            repo_root.read_dir()
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .any(|entry| {
                            entry.file_name()
                                .to_str()
                                .map(|name| name.starts_with("Pulumi.") &&
                                           (name.ends_with(".yaml") || name.ends_with(".yml")))
                                .unwrap_or(false)
                        })
                })
                .unwrap_or(false))
    }

    fn has_tests(repo_root: &Path, config: &Config) -> bool {
        // Check if any package has [cmd.test] defined
        let has_test_cmd = config
            .packages
            .values()
            .any(|pkg| pkg.cmd.contains_key("test"));

        if has_test_cmd {
            return true;
        }

        // Check for common test directories/files
        let common_test_paths = [
            "tests",         // Rust/Python tests
            "test",          // General tests
            "__tests__",     // Jest tests
            "src/__tests__", // Inline tests
        ];

        for test_path in &common_test_paths {
            if repo_root.join(test_path).exists() {
                return true;
            }
        }

        // Check if any package has test directories
        config.packages.values().any(|pkg| {
            pkg.path.join("tests").exists()
                || pkg.path.join("test").exists()
                || pkg.path.join("__tests__").exists()
                || pkg.path.join("src/__tests__").exists()
        })
    }
}

/// Tool detection - check if specific tools are installed
pub struct Tools;

impl Tools {
    pub fn docker() -> bool {
        docker_available()
    }

    pub fn git() -> bool {
        cmd_exists("git")
    }

    pub fn gh() -> bool {
        cmd_exists("gh")
    }

    pub fn cargo() -> bool {
        cmd_exists("cargo")
    }

    pub fn sqlx() -> bool {
        cmd_exists("sqlx")
    }

    pub fn npm() -> bool {
        cmd_exists("npm")
    }

    pub fn yarn() -> bool {
        cmd_exists("yarn")
    }

    pub fn pnpm() -> bool {
        cmd_exists("pnpm")
    }
}
