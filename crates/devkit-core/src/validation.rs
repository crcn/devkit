//! Configuration validation

use crate::config::Config;
use crate::error::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate configuration
pub fn validate_config(config: &Config) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Validate glob patterns
    validate_glob_patterns(config, &mut report);

    // Validate command dependencies
    validate_command_dependencies(config, &mut report)?;

    // Validate port conflicts
    validate_port_conflicts(config, &mut report);

    // Check for empty packages
    if config.packages.is_empty() {
        report.add_warning(
            "No packages found. Check your workspace patterns in .dev/config.toml".to_string(),
        );
    }

    Ok(report)
}

fn validate_glob_patterns(config: &Config, report: &mut ValidationReport) {
    for pattern in &config.global.workspaces.packages {
        if let Err(e) = glob::Pattern::new(pattern) {
            report.add_error(format!("Invalid glob pattern '{}': {}", pattern, e));
        }
    }

    for pattern in &config.global.workspaces.infra {
        if let Err(e) = glob::Pattern::new(pattern) {
            report.add_error(format!("Invalid infra pattern '{}': {}", pattern, e));
        }
    }
}

fn validate_command_dependencies(config: &Config, report: &mut ValidationReport) -> Result<()> {
    // Build dependency graph
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (pkg_name, pkg_config) in &config.packages {
        for (cmd_name, cmd_entry) in &pkg_config.cmd {
            let node = format!("{}:{}", pkg_name, cmd_name);
            let mut deps = Vec::new();

            for dep in cmd_entry.deps() {
                let dep_node = if dep.contains(':') {
                    dep.to_string()
                } else {
                    // "package" shorthand means "package:same_command"
                    format!("{}:{}", dep, cmd_name)
                };

                // Validate that dependency exists
                if !dependency_exists(config, &dep_node) {
                    report.add_error(format!(
                        "Invalid dependency '{}' in {}:{} - dependency not found",
                        dep, pkg_name, cmd_name
                    ));
                }

                deps.push(dep_node);
            }

            graph.insert(node.clone(), deps);
        }
    }

    // Check for circular dependencies
    for node in graph.keys() {
        if let Some(cycle) = detect_cycle(&graph, node) {
            report.add_error(format!("Circular dependency detected: {}", cycle));
        }
    }

    Ok(())
}

fn dependency_exists(config: &Config, dep_ref: &str) -> bool {
    let parts: Vec<&str> = dep_ref.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let (pkg_name, cmd_name) = (parts[0], parts[1]);
    config
        .packages
        .get(pkg_name)
        .and_then(|pkg| pkg.cmd.get(cmd_name))
        .is_some()
}

fn detect_cycle(graph: &HashMap<String, Vec<String>>, start: &str) -> Option<String> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<String> {
        if path.contains(&node.to_string()) {
            // Found a cycle
            let cycle_start = path.iter().position(|n| n == node).unwrap();
            let cycle: Vec<String> = path[cycle_start..]
                .iter()
                .chain(std::iter::once(&node.to_string()))
                .cloned()
                .collect();
            return Some(cycle.join(" -> "));
        }

        if visited.contains(node) {
            return None;
        }

        visited.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = graph.get(node) {
            for dep in deps {
                if let Some(cycle) = dfs(graph, dep, visited, path) {
                    return Some(cycle);
                }
            }
        }

        path.pop();
        None
    }

    dfs(graph, start, &mut visited, &mut path)
}

fn validate_port_conflicts(config: &Config, report: &mut ValidationReport) {
    let mut port_map: HashMap<u16, Vec<String>> = HashMap::new();

    for (service, port) in &config.global.services.ports {
        port_map
            .entry(*port)
            .or_insert_with(Vec::new)
            .push(service.clone());
    }

    for (port, services) in port_map {
        if services.len() > 1 {
            report.add_warning(format!(
                "Port {} is used by multiple services: {}",
                port,
                services.join(", ")
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CmdConfig, CmdEntry, GlobalConfig, PackageConfig};
    use std::collections::HashMap;

    #[test]
    fn test_circular_dependency_detection() {
        let mut packages = HashMap::new();

        let mut cmd_a = HashMap::new();
        cmd_a.insert(
            "build".to_string(),
            CmdEntry::Full(CmdConfig {
                default: "cargo build".to_string(),
                deps: vec!["b:build".to_string()],
                variants: HashMap::new(),
            }),
        );

        packages.insert(
            "a".to_string(),
            PackageConfig {
                path: "/a".into(),
                dir_name: "a".to_string(),
                name: "a".to_string(),
                database: None,
                mobile: None,
                cmd: cmd_a,
            },
        );

        let mut cmd_b = HashMap::new();
        cmd_b.insert(
            "build".to_string(),
            CmdEntry::Full(CmdConfig {
                default: "cargo build".to_string(),
                deps: vec!["a:build".to_string()], // Circular!
                variants: HashMap::new(),
            }),
        );

        packages.insert(
            "b".to_string(),
            PackageConfig {
                path: "/b".into(),
                dir_name: "b".to_string(),
                name: "b".to_string(),
                database: None,
                mobile: None,
                cmd: cmd_b,
            },
        );

        let config = Config {
            repo_root: "/".into(),
            global: GlobalConfig::default(),
            packages,
        };

        let report = validate_config(&config).unwrap();
        assert!(!report.is_valid());
        assert!(!report.errors.is_empty());
        assert!(report.errors[0].contains("Circular dependency"));
    }

    #[test]
    fn test_invalid_dependency() {
        let mut packages = HashMap::new();

        let mut cmd = HashMap::new();
        cmd.insert(
            "build".to_string(),
            CmdEntry::Full(CmdConfig {
                default: "cargo build".to_string(),
                deps: vec!["nonexistent:build".to_string()],
                variants: HashMap::new(),
            }),
        );

        packages.insert(
            "a".to_string(),
            PackageConfig {
                path: "/a".into(),
                dir_name: "a".to_string(),
                name: "a".to_string(),
                database: None,
                mobile: None,
                cmd,
            },
        );

        let config = Config {
            repo_root: "/".into(),
            global: GlobalConfig::default(),
            packages,
        };

        let report = validate_config(&config).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors[0].contains("dependency not found"));
    }

    #[test]
    fn test_valid_config() {
        let mut packages = HashMap::new();

        let mut cmd = HashMap::new();
        cmd.insert(
            "build".to_string(),
            CmdEntry::Simple("cargo build".to_string()),
        );

        packages.insert(
            "a".to_string(),
            PackageConfig {
                path: "/a".into(),
                dir_name: "a".to_string(),
                name: "a".to_string(),
                database: None,
                mobile: None,
                cmd,
            },
        );

        let config = Config {
            repo_root: "/".into(),
            global: GlobalConfig::default(),
            packages,
        };

        let report = validate_config(&config).unwrap();
        assert!(report.is_valid());
    }
}
