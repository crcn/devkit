# devkit Custom CLI - AI Assistant Guide

This project uses **devkit**, a Rust-based development environment orchestration toolkit. This guide helps AI assistants understand the project structure and how to work with it effectively.

---

## What is devkit?

devkit is a zero-friction CLI tool that unifies development workflows. It provides:
- **Smart feature detection** - Automatically discovers available tools
- **Package command system** - Define commands once in `dev.toml`, run anywhere
- **Extension system** - Modular functionality you can mix and match
- **Template support** - Variable substitution in commands
- **File watching** - Auto-rerun commands on file changes

---

## Project Structure

```
my-cli/
├── .dev/
│   └── config.toml          # Global project configuration
├── crates/
│   └── my-cli/
│       ├── Cargo.toml
│       ├── dev.toml         # Package-specific commands
│       └── src/
│           └── main.rs      # CLI implementation
├── extensions/              # Custom extensions (optional)
│   └── my-ext/
│       ├── Cargo.toml
│       ├── dev.toml
│       └── src/lib.rs
└── claude.md               # This file
```

---

## Configuration Files

### `.dev/config.toml` (Global)
```toml
[project]
name = "my-cli"

[workspaces]
packages = ["crates/*", "extensions/*"]

[environments]
available = ["dev", "staging", "prod"]
default = "dev"

[aliases]
t = "test"
b = "build"

[features]
docker = true
database = true
```

### `dev.toml` (Per Package)
```toml
[cmd.build]
default = "cargo build"
release = "cargo build --release"
watch = "cargo watch -x build"
deps = ["common:build"]  # Dependency on other packages

[cmd.test]
default = "cargo test"

[cmd.lint]
default = "cargo clippy"
fix = "cargo clippy --fix"
```

---

## Common Development Commands

```bash
# Using the devkit binary
devkit                      # Interactive menu
devkit status              # Show project status
devkit cmd build           # Run build command
devkit cmd test            # Run tests
devkit cmd build --watch   # Watch and rebuild

# Using the generated dev.sh wrapper
./dev.sh                   # Interactive menu
./dev.sh cmd build         # Build all packages
./dev.sh cmd test          # Test all packages
```

---

## Creating Extensions

Extensions extend devkit functionality. Here's the pattern:

```rust
use devkit_core::{AppContext, Extension, MenuItem};

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my-extension"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Check if this extension should be enabled
        ctx.features.docker
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![MenuItem {
            label: "🚀 My Command".to_string(),
            handler: Box::new(|ctx| {
                // Command implementation
                ctx.print_success("Done!");
                Ok(())
            }),
        }]
    }
}
```

---

## Command Templates

Commands support variable substitution:

```toml
[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"

[cmd.run]
default = "{runtime} {entrypoint} --port {port}"
vars = { runtime = "node", entrypoint = "index.js", port = "3000" }
```

Usage:
```bash
export env=prod
devkit cmd deploy  # Resolves to: kubectl apply -f k8s/prod.yaml
```

---

## Available Extensions

### Built-in Extensions

1. **devkit-ext-cache** - Build cache management
   - `devkit cache clean` - Clean all caches
   - `devkit cache stats` - Show cache sizes

2. **devkit-ext-secrets** - Secrets management
   - Supports: AWS, 1Password, Doppler
   - `devkit secrets pull` - Pull secrets to .env

3. **devkit-ext-security** - Security scanning
   - cargo audit, npm audit, gitleaks
   - `devkit security scan` - Full scan

4. **devkit-ext-k8s** - Kubernetes operations
   - `devkit k8s status` - Cluster status
   - `devkit k8s pods` - List pods

5. **devkit-ext-monitoring** - Prometheus/Grafana stack
   - `devkit monitoring up` - Start stack

6. **devkit-ext-watch** - Advanced file watching
7. **devkit-ext-remote** - Remote development
8. **devkit-ext-dashboard** - Terminal UI

### Installing Extensions from crcn/devkit

You can use any of the official devkit extensions by adding them to your `Cargo.toml`:

```toml
# In your CLI's Cargo.toml

[dependencies]
# Core dependencies
devkit-core = { git = "https://github.com/crcn/devkit" }
devkit-tasks = { git = "https://github.com/crcn/devkit" }

# Add official extensions as needed
devkit-ext-cache = { git = "https://github.com/crcn/devkit" }
devkit-ext-secrets = { git = "https://github.com/crcn/devkit" }
devkit-ext-security = { git = "https://github.com/crcn/devkit" }
devkit-ext-k8s = { git = "https://github.com/crcn/devkit" }
devkit-ext-monitoring = { git = "https://github.com/crcn/devkit" }
devkit-ext-watch = { git = "https://github.com/crcn/devkit" }
devkit-ext-remote = { git = "https://github.com/crcn/devkit" }
devkit-ext-dashboard = { git = "https://github.com/crcn/devkit" }
```

Then register them in your CLI:

```rust
// In main.rs or your CLI setup

use devkit_core::ExtensionRegistry;

fn main() {
    let mut registry = ExtensionRegistry::new();

    // Register official extensions
    #[cfg(feature = "cache")]
    registry.register(Box::new(devkit_ext_cache::CacheExtension));

    #[cfg(feature = "secrets")]
    registry.register(Box::new(devkit_ext_secrets::SecretsExtension));

    #[cfg(feature = "security")]
    registry.register(Box::new(devkit_ext_security::SecurityExtension));

    #[cfg(feature = "k8s")]
    registry.register(Box::new(devkit_ext_k8s::K8sExtension));

    // ... register other extensions

    // Use the registry
    let menu_items = registry.menu_items(&ctx);
}
```

**Feature flags** (recommended pattern):
```toml
[features]
default = ["cache", "secrets"]
all = ["cache", "secrets", "security", "k8s", "monitoring"]

cache = ["devkit-ext-cache"]
secrets = ["devkit-ext-secrets"]
security = ["devkit-ext-security"]
k8s = ["devkit-ext-k8s"]
monitoring = ["devkit-ext-monitoring"]
```

This allows users to enable/disable extensions:
```bash
# Build with default features
cargo build

# Build with all features
cargo build --all-features

# Build with specific features
cargo build --features k8s,monitoring
```

---

## Common Patterns

### Adding a New Command

1. Edit the package's `dev.toml`:
```toml
[cmd.my-command]
default = "cargo run --bin my-tool"
```

2. Run it:
```bash
devkit cmd my-command
```

### Adding Dependencies Between Commands

```toml
[cmd.integration-test]
default = "cargo test --test integration"
deps = ["api:build", "db:migrate"]  # Runs these first
```

### Using the Extension System

1. Create extension directory:
```
extensions/my-ext/
├── Cargo.toml
├── dev.toml
└── src/lib.rs
```

2. Implement `Extension` trait
3. Register in main CLI:
```rust
registry.register(Box::new(MyExtension));
```

---

## Working with devkit-core

### Key Types

**AppContext** - Shared state across all commands
```rust
pub struct AppContext {
    pub repo: PathBuf,           // Project root
    pub config: Config,          // Parsed configuration
    pub features: Features,      // Detected features
    pub quiet: bool,            // Quiet mode
}
```

**Features** - Auto-detected capabilities
```rust
pub struct Features {
    pub docker: bool,
    pub database: bool,
    pub cargo: bool,
    pub node: bool,
    // ... more
}
```

### Utility Functions
```rust
// Print helpers
ctx.print_success("✓ Done");
ctx.print_error("Error occurred");
ctx.print_warning("Warning");
ctx.print_info("Info message");
ctx.print_header("Section Header");

// Feature detection
if ctx.features.docker {
    // Docker available
}

// Command execution
run_cmd(ctx, "build", &opts)?;
```

---

## Testing

Run tests:
```bash
devkit cmd test                  # All tests
devkit cmd test -p my-crate     # Specific crate
cargo test --workspace          # Direct cargo
```

---

## Building & Releasing

```bash
# Development build
devkit cmd build

# Release build
devkit cmd build --variant release

# Or directly
cargo build --release -p my-cli

# Binary location
target/release/my-cli
```

---

## Debugging

Enable debug logging:
```bash
RUST_LOG=devkit=debug devkit cmd build
RUST_LOG=trace devkit cmd test
```

---

## AI Assistant Instructions

When working with this project:

1. **Use devkit commands** - Don't bypass devkit, use its commands
2. **Check dev.toml** - Look at package dev.toml for available commands
3. **Respect features** - Check ctx.features before using functionality
4. **Follow patterns** - Use existing extension patterns
5. **Add tests** - Test new functionality
6. **Update documentation** - Keep this file current

### Common Tasks

**Add new command:**
1. Edit `dev.toml`
2. Add command definition
3. Test with `devkit cmd <name>`

**Create extension:**
1. Create directory in `extensions/`
2. Implement `Extension` trait
3. Register in CLI
4. Add to workspace in `Cargo.toml`

**Modify configuration:**
1. Edit `.dev/config.toml` for global changes
2. Edit package `dev.toml` for package-specific changes

---

## Troubleshooting

**Command not found:**
- Check if defined in `dev.toml`
- Run `devkit cmd --list` to see available commands

**Feature not available:**
- Check `devkit status` to see detected features
- Ensure required tools are installed
- Run `devkit doctor` for system check

**Build errors:**
- Run `cargo clean`
- Check dependencies in `Cargo.toml`
- Ensure workspace members are listed

---

## Resources

- [devkit GitHub](https://github.com/crcn/devkit)
- [Architecture docs](../ARCHITECTURE.md)
- [Roadmap](../ROADMAP.md)
- [Examples](../examples/)

---

## Quick Reference

```bash
# Core commands
devkit init                 # Initialize project
devkit status              # Show status
devkit doctor              # Health check
devkit update              # Check for updates
devkit history             # Command history

# Package commands
devkit cmd build           # Build
devkit cmd test            # Test
devkit cmd lint            # Lint
devkit cmd <name>          # Custom command

# Flags
--watch                    # Watch mode
--parallel                 # Parallel execution
-p <pkg>                   # Specific package
--format json              # JSON output

# Extensions (via menu)
devkit                     # Opens interactive menu
# Select from: Cache, Secrets, Security, K8s, Monitoring, etc.
```

---

*This file should be kept up-to-date as the project evolves.*
