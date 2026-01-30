//! Project initialization and setup wizard

use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, MultiSelect};
use glob;
use std::fs;
use std::path::Path;

/// Initialize a new devkit project
pub fn init_project(path: &Path, interactive: bool) -> Result<()> {
    println!("🚀 Initializing devkit project");
    println!();

    let project_name = if interactive {
        Input::<String>::new()
            .with_prompt("Project name")
            .default(
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("my-project")
                    .to_string(),
            )
            .interact_text()?
    } else {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string()
    };

    // Detect existing tools
    let has_docker = crate::docker_available();
    let has_cargo = crate::cmd_exists("cargo");
    let has_node = crate::cmd_exists("node");
    let has_python = crate::cmd_exists("python") || crate::cmd_exists("python3");

    println!("Detected:");
    if has_docker {
        println!("  ✓ Docker");
    }
    if has_cargo {
        println!("  ✓ Rust/Cargo");
    }
    if has_node {
        println!("  ✓ Node.js");
    }
    if has_python {
        println!("  ✓ Python");
    }
    println!();

    // Ask which features to enable
    let features = if interactive {
        let feature_options = vec![
            "Docker operations",
            "Database management",
            "Code quality tools",
            "CI integration",
            "Environment management",
            "Tunneling (ngrok/cloudflared)",
        ];

        let defaults = vec![true, true, true, false, false, false]; // Docker, Database, Quality enabled by default

        MultiSelect::new()
            .with_prompt("Select features to enable")
            .items(&feature_options)
            .defaults(&defaults)
            .interact()?
    } else {
        vec![0, 1, 2] // Default features
    };

    // Create .dev directory
    let dev_dir = path.join(".dev");
    fs::create_dir_all(&dev_dir).context("Failed to create .dev directory")?;

    // Generate config.toml
    let config_content = generate_config(&project_name, has_docker, &features);

    fs::write(dev_dir.join("config.toml"), config_content)
        .context("Failed to write config.toml")?;

    println!("✓ Created .dev/config.toml");

    // Generate .gitignore entry if needed
    let gitignore_path = path.join(".gitignore");
    if !gitignore_path.exists()
        || !fs::read_to_string(&gitignore_path)
            .unwrap_or_default()
            .contains(".dev/")
    {
        if interactive {
            let add_gitignore = Confirm::new()
                .with_prompt("Add .dev/ to .gitignore?")
                .default(true)
                .interact()?;

            if add_gitignore {
                let mut content = fs::read_to_string(&gitignore_path).unwrap_or_default();
                if !content.ends_with('\n') && !content.is_empty() {
                    content.push('\n');
                }
                content.push_str("\n# devkit\n.dev/\n");
                fs::write(&gitignore_path, content)?;
                println!("✓ Updated .gitignore");
            }
        }
    }

    // Scan for packages and generate dev.toml files
    println!();
    println!("Scanning for packages...");
    let packages_generated = scan_and_generate_package_configs(path)?;

    if packages_generated > 0 {
        println!(
            "✓ Generated {} package dev.toml file(s)",
            packages_generated
        );
    }

    println!();
    println!("✓ devkit project initialized!");
    println!();
    println!("Next steps:");
    println!("  devkit          - Open interactive menu");
    println!("  devkit status   - Check project status");
    println!("  devkit doctor   - Verify system prerequisites");

    Ok(())
}

/// Scan project for packages and generate dev.toml files with detected capabilities
fn scan_and_generate_package_configs(project_root: &Path) -> Result<usize> {
    let mut count = 0;

    // Scan for Rust packages (Cargo.toml)
    for entry in glob::glob(&format!("{}/**/Cargo.toml", project_root.display()))
        .context("Failed to glob for Cargo.toml files")?
    {
        if let Ok(cargo_path) = entry {
            // Skip workspace root Cargo.toml if it doesn't have a [package] section
            let content = fs::read_to_string(&cargo_path)?;
            if !content.contains("[package]") {
                continue;
            }

            let package_dir = cargo_path.parent().unwrap();
            let dev_toml_path = package_dir.join("dev.toml");

            // Skip if dev.toml already exists
            if dev_toml_path.exists() {
                continue;
            }

            // Generate Rust dev.toml
            let dev_config = generate_rust_dev_toml(&cargo_path)?;
            fs::write(&dev_toml_path, dev_config)?;
            println!("  ✓ Created {}", dev_toml_path.display());
            count += 1;
        }
    }

    // Scan for Node packages (package.json)
    for entry in glob::glob(&format!("{}/**/package.json", project_root.display()))
        .context("Failed to glob for package.json files")?
    {
        if let Ok(package_path) = entry {
            let package_dir = package_path.parent().unwrap();

            // Skip node_modules
            if package_dir.to_string_lossy().contains("node_modules") {
                continue;
            }

            let dev_toml_path = package_dir.join("dev.toml");

            // Skip if dev.toml already exists
            if dev_toml_path.exists() {
                continue;
            }

            // Generate Node dev.toml
            let dev_config = generate_node_dev_toml(&package_path)?;
            fs::write(&dev_toml_path, dev_config)?;
            println!("  ✓ Created {}", dev_toml_path.display());
            count += 1;
        }
    }

    Ok(count)
}

/// Generate dev.toml for a Rust package
fn generate_rust_dev_toml(cargo_path: &Path) -> Result<String> {
    let content = fs::read_to_string(cargo_path)?;
    let package_dir = cargo_path.parent().unwrap();

    // Parse package name
    let name = content
        .lines()
        .find(|line| line.starts_with("name = "))
        .and_then(|line| line.split('"').nth(1))
        .unwrap_or("package");

    let mut config = format!(
        r#"# =============================================================================
# {} Dev Configuration
# =============================================================================

"#,
        name
    );

    // Detect if it's a binary or library
    let is_bin = package_dir.join("src/main.rs").exists();
    if is_bin {
        config.push_str("releasable = true\n\n");
    }

    // Add standard Rust commands
    config.push_str(
        r#"[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
release = "cargo build --release"

[cmd.lint]
default = "cargo clippy --all-targets --all-features -- -D warnings"
fix = "cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features"

[cmd.fmt]
default = "cargo fmt --all --check"
fix = "cargo fmt --all"

[cmd.typecheck]
default = "cargo check --all-targets --all-features"

[cmd]
test = "cargo test"
"#,
    );

    Ok(config)
}

/// Generate dev.toml for a Node package
fn generate_node_dev_toml(package_path: &Path) -> Result<String> {
    let content = fs::read_to_string(package_path)?;
    let package_json: serde_json::Value = serde_json::from_str(&content)?;

    let name = package_json["name"]
        .as_str()
        .unwrap_or("package")
        .trim_start_matches('@')
        .replace('/', "-");

    let mut config = format!(
        r#"# =============================================================================
# {} Dev Configuration
# =============================================================================

"#,
        name
    );

    // Detect package manager
    let package_dir = package_path.parent().unwrap();
    let pm = if package_dir.join("yarn.lock").exists() {
        "yarn"
    } else if package_dir.join("pnpm-lock.yaml").exists() {
        "pnpm"
    } else {
        "npm"
    };

    // Check scripts in package.json
    let scripts = package_json["scripts"].as_object();
    let has_test = scripts.map_or(false, |s| s.contains_key("test"));
    let has_lint = scripts.map_or(false, |s| s.contains_key("lint"));
    let has_fmt = scripts.map_or(false, |s| {
        s.contains_key("format") || s.contains_key("prettier")
    });
    let has_typecheck = scripts.map_or(false, |s| {
        s.contains_key("typecheck") || s.contains_key("type-check")
    });
    let has_build = scripts.map_or(false, |s| s.contains_key("build"));
    let has_dev = scripts.map_or(false, |s| s.contains_key("dev"));

    // Generate commands based on detected scripts
    if has_build {
        config.push_str(&format!(
            r#"[cmd.build]
default = "{} run build"

"#,
            pm
        ));
    }

    if has_lint {
        config.push_str(&format!(
            r#"[cmd.lint]
default = "{} run lint"
fix = "{} run lint -- --fix"

"#,
            pm, pm
        ));
    }

    if has_fmt {
        config.push_str(&format!(
            r#"[cmd.fmt]
default = "{} run format -- --check"
fix = "{} run format"

"#,
            pm, pm
        ));
    }

    if has_typecheck {
        config.push_str(&format!(
            r#"[cmd.typecheck]
default = "{} run typecheck"

"#,
            pm
        ));
    }

    if has_dev {
        config.push_str(&format!(
            r#"[cmd.dev]
default = "{} run dev"

"#,
            pm
        ));
    }

    if has_test {
        config.push_str(&format!(
            r#"[cmd]
test = "{} run test"
"#,
            pm
        ));
    }

    Ok(config)
}

fn generate_config(project_name: &str, has_docker: bool, features: &[usize]) -> String {
    let mut config = format!(
        r#"[project]
name = "{}"

[workspaces]
packages = ["packages/*", "apps/*"]

[environments]
available = ["dev", "staging", "prod"]
default = "dev"
"#,
        project_name
    );

    if has_docker {
        config.push_str(
            r#"
[services]
# Add your services here
# api = 8080
# postgres = 5432
"#,
        );
    }

    // Add features configuration
    config.push_str("\n[features]\n");
    config.push_str(&format!("docker = {}\n", features.contains(&0)));
    config.push_str(&format!("database = {}\n", features.contains(&1)));
    config.push_str(&format!("quality = {}\n", features.contains(&2)));
    config.push_str(&format!("ci = {}\n", features.contains(&3)));
    config.push_str(&format!("env = {}\n", features.contains(&4)));
    config.push_str(&format!("tunnel = {}\n", features.contains(&5)));

    config
}
