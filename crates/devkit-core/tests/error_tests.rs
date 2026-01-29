use devkit_core::error::DevkitError;
use std::path::PathBuf;

#[test]
fn test_command_not_found_error() {
    let err = DevkitError::command_not_found(
        "test".to_string(),
        "api".to_string(),
        vec!["build".to_string(), "lint".to_string()],
    );

    let msg = err.to_string();
    assert!(msg.contains("test"));
    assert!(msg.contains("api"));
    assert!(msg.contains("build, lint"));
}

#[test]
fn test_package_not_found_error() {
    let err = DevkitError::package_not_found(
        "missing".to_string(),
        vec!["api".to_string(), "web".to_string()],
    );

    let msg = err.to_string();
    assert!(msg.contains("missing"));
    assert!(msg.contains("api, web"));
}

#[test]
fn test_docker_compose_failed_error_suggestions() {
    let err1 = DevkitError::docker_compose_failed("file not found".to_string());
    assert!(err1.to_string().contains("docker-compose.yml"));

    let err2 = DevkitError::docker_compose_failed("Cannot connect to daemon".to_string());
    assert!(err2.to_string().contains("Docker is running"));
}

#[test]
fn test_repo_root_not_found_error() {
    let err = DevkitError::RepoRootNotFound;
    let msg = err.to_string();
    assert!(msg.contains("Repository root not found"));
    assert!(msg.contains("git repository") || msg.contains(".dev/config.toml"));
}

#[test]
fn test_feature_not_available_error() {
    let err = DevkitError::feature_not_available(
        "docker".to_string(),
        "Install Docker from https://docker.com".to_string(),
    );

    let msg = err.to_string();
    assert!(msg.contains("docker"));
    assert!(msg.contains("https://docker.com"));
}

#[test]
fn test_circular_dependency_error() {
    let err = DevkitError::CircularDependency {
        cycle: "a:build -> b:build -> a:build".to_string(),
    };

    let msg = err.to_string();
    assert!(msg.contains("Circular dependency"));
    assert!(msg.contains("a:build -> b:build -> a:build"));
}
