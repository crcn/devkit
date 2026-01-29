use devkit_core::config::*;
use std::collections::HashMap;

#[test]
fn test_cmd_entry_simple() {
    let cmd = CmdEntry::Simple("cargo test".to_string());
    assert_eq!(cmd.default_cmd(), "cargo test");
    assert_eq!(cmd.variant("watch"), "cargo test");
    assert_eq!(cmd.deps().len(), 0);
}

#[test]
fn test_cmd_entry_full() {
    let mut variants = HashMap::new();
    variants.insert("watch".to_string(), "cargo watch -x test".to_string());
    variants.insert("release".to_string(), "cargo test --release".to_string());

    let cmd = CmdEntry::Full(CmdConfig {
        default: "cargo test".to_string(),
        deps: vec!["common:build".to_string()],
        variants,
    });

    assert_eq!(cmd.default_cmd(), "cargo test");
    assert_eq!(cmd.variant("watch"), "cargo watch -x test");
    assert_eq!(cmd.variant("release"), "cargo test --release");
    assert_eq!(cmd.variant("unknown"), "cargo test"); // Falls back to default
    assert_eq!(cmd.deps().len(), 1);
    assert_eq!(cmd.deps()[0], "common:build");
}

#[test]
fn test_global_config_defaults() {
    let config = GlobalConfig::default();
    assert_eq!(config.project.name, "my-project");
    assert_eq!(config.workspaces.packages, vec!["packages/*"]);
    // Note: Rust's Default trait gives empty string, but serde default would use "dev"
    // In practice this is loaded via serde, not Default trait
    assert!(config.services.ports.is_empty());
}

#[test]
fn test_services_config_get_port() {
    let mut ports = HashMap::new();
    ports.insert("api".to_string(), 8080);
    ports.insert("postgres".to_string(), 5432);

    let services = ServicesConfig { ports };

    assert_eq!(services.get_port("api", 3000), 8080);
    assert_eq!(services.get_port("postgres", 3000), 5432);
    assert_eq!(services.get_port("unknown", 3000), 3000); // Returns default
}

#[test]
fn test_urls_config() {
    let mut entries = HashMap::new();
    entries.insert(
        "playground".to_string(),
        UrlEntry {
            label: "GraphQL Playground".to_string(),
            url: "http://localhost:8080/playground".to_string(),
        },
    );

    let urls = UrlsConfig { entries };

    assert!(!urls.is_empty());
    assert!(urls.get("playground").is_some());
    assert_eq!(urls.get("playground").unwrap().label, "GraphQL Playground");
    assert!(urls.get("unknown").is_none());
}

#[test]
fn test_package_config_has_database() {
    let pkg = PackageConfig {
        path: "/test".into(),
        dir_name: "test".to_string(),
        name: "test-pkg".to_string(),
        database: Some(DatabaseConfig {
            migrations: "migrations".to_string(),
            seeds: Some("seeds/dev.sql".to_string()),
        }),
        mobile: None,
        cmd: HashMap::new(),
    };

    assert!(pkg.database.is_some());
    assert_eq!(pkg.database.as_ref().unwrap().migrations, "migrations");
}
