use devkit_core::utils::*;

#[test]
fn test_cmd_exists() {
    // These commands should exist on most systems
    #[cfg(unix)]
    {
        assert!(cmd_exists("ls"));
        assert!(cmd_exists("echo"));
    }

    #[cfg(windows)]
    {
        assert!(cmd_exists("cmd"));
    }

    // This command almost certainly doesn't exist
    assert!(!cmd_exists("definitely-not-a-real-command-12345"));
}

#[test]
fn test_docker_available() {
    // We can't assume docker is installed, so just test that the function runs
    let _available = docker_available();
}

#[test]
fn test_ensure_docker() {
    // Should either succeed or return feature not available error
    let result = ensure_docker();
    if result.is_err() {
        let err = result.unwrap_err();
        assert!(err.to_string().contains("docker") || err.to_string().contains("not available"));
    }
}

#[test]
fn test_ensure_cargo() {
    // Cargo should be available since we're running tests with cargo
    assert!(ensure_cargo().is_ok());
}

#[test]
fn test_docker_compose_program() {
    let result = docker_compose_program();

    // Either succeeds and returns a valid program, or fails with helpful error
    if let Ok((prog, _args)) = result {
        assert!(prog == "docker" || prog == "docker-compose");
    }
}
