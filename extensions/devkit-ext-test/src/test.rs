//! Test running functionality

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;
use std::process::{Command, Stdio};

/// Check if the test command is cargo-based
fn is_cargo(command: &str) -> bool {
    command.starts_with("cargo ")
}

/// Check if the test command is nextest-based
fn is_nextest(command: &str) -> bool {
    command.contains("nextest")
}

/// Parse command into executable and arguments
fn parse_command(command: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let (exe, args) = parts.split_first().unwrap_or((&"echo", &[]));
    (*exe, args.to_vec())
}

/// Options for running tests
pub struct TestOptions {
    /// Specific package to test (cargo only)
    pub package: Option<String>,
    /// Test name filter
    pub filter: Option<String>,
    /// Capture errors instead of failing
    pub capture_errors: bool,
    /// Custom test command (overrides config)
    pub command: Option<String>,
}

impl Default for TestOptions {
    fn default() -> Self {
        Self {
            package: None,
            filter: None,
            capture_errors: false,
            command: None,
        }
    }
}

/// Run tests
///
/// If `capture_errors` is true, returns captured error output instead of failing.
/// This is useful for AI-assisted error fixing.
pub fn run_tests(ctx: &AppContext, opts: &TestOptions) -> Result<Option<String>> {
    // Determine test command
    let test_command = if let Some(cmd) = &opts.command {
        cmd.clone()
    } else if ctx.features.cargo {
        // Default to cargo nextest if available, otherwise cargo test
        if devkit_core::cmd_exists("cargo-nextest") {
            "cargo nextest run".to_string()
        } else {
            "cargo test".to_string()
        }
    } else if ctx.features.node {
        // Try common JS test runners
        if devkit_core::cmd_exists("npm") {
            "npm test".to_string()
        } else if devkit_core::cmd_exists("yarn") {
            "yarn test".to_string()
        } else {
            return Err(anyhow!("No test command found. Configure [test.command] in config"));
        }
    } else {
        return Err(anyhow!(
            "No test framework detected. Configure [test.command] in config"
        ));
    };

    let (exe, base_args) = parse_command(&test_command);

    // Build args: start with base command args
    let mut args: Vec<String> = base_args.iter().map(|s| s.to_string()).collect();

    // Add package/filter args only for cargo-based commands
    if is_cargo(&test_command) {
        if let Some(pkg) = &opts.package {
            args.push("-p".to_string());
            args.push(pkg.to_string());
        }

        if let Some(filter) = &opts.filter {
            if is_nextest(&test_command) {
                args.push("-E".to_string());
                args.push(format!("test({})", filter));
            } else {
                args.push("--".to_string());
                args.push(filter.to_string());
            }
        }
    }

    ctx.print_header(&format!("Running tests: {} {}", exe, args.join(" ")));

    if opts.capture_errors {
        // Capture output while displaying it to the user
        let output = Command::new(exe)
            .args(&args)
            .current_dir(&ctx.repo)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Print output so user sees it
        if !stdout.is_empty() {
            print!("{}", stdout);
        }
        if !stderr.is_empty() {
            eprint!("{}", stderr);
        }

        if !output.status.success() {
            let mut error_output = format!("=== {} ===\n", test_command);
            error_output.push_str(&stderr);
            error_output.push_str(&stdout);
            return Ok(Some(error_output));
        }
    } else {
        let code = CmdBuilder::new(exe)
            .args(&args)
            .cwd(&ctx.repo)
            .inherit_io()
            .run()?;

        if code != 0 {
            return Err(anyhow!("{} exited with code {}", test_command, code));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cargo() {
        assert!(is_cargo("cargo test"));
        assert!(is_cargo("cargo nextest run"));
        assert!(!is_cargo("npm test"));
    }

    #[test]
    fn test_is_nextest() {
        assert!(is_nextest("cargo nextest run"));
        assert!(!is_nextest("cargo test"));
    }

    #[test]
    fn test_parse_command() {
        let (exe, args) = parse_command("cargo nextest run");
        assert_eq!(exe, "cargo");
        assert_eq!(args, vec!["nextest", "run"]);

        let (exe, args) = parse_command("npm test");
        assert_eq!(exe, "npm");
        assert_eq!(args, vec!["test"]);
    }
}
