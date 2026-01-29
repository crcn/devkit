# devkit

**A comprehensive development environment orchestration toolkit written in Rust.**

> ✨ **Status**: Active development - Core features complete, extensions in progress

---

## 🚀 Installation

### Quick Install (Recommended)

Install devkit binary + project setup:

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash
```

This installs the binary to `~/.local/bin` and optionally sets up your project with configs and wrapper scripts.

### Binary Only

Just want the CLI tool globally? Download and install:

```bash
# Detect platform and install
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m | sed 's/arm64/aarch64/')
curl -fsSL "https://github.com/crcn/devkit/releases/latest/download/devkit-${PLATFORM}" -o devkit
chmod +x devkit
sudo mv devkit /usr/local/bin/
```

Or manually for your platform:

```bash
# Linux x86_64
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-linux-x86_64 -o devkit

# Linux ARM64
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-linux-aarch64 -o devkit

# macOS Intel
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-macos-x86_64 -o devkit

# macOS Apple Silicon
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-macos-aarch64 -o devkit

# Then install
chmod +x devkit && sudo mv devkit /usr/local/bin/
```

<details>
<summary>Windows Installation</summary>

Download from [releases page](https://github.com/crcn/devkit/releases/latest):
- `devkit-windows-x86_64.exe`

Add to PATH or rename to `devkit.exe` and place in a directory in your PATH.
</details>

### Build from Source

```bash
git clone https://github.com/crcn/devkit
cd devkit
cargo build --release -p devkit-cli
# Binary at target/release/devkit
```

---

## What is devkit?

`devkit` is a zero-friction CLI tool that unifies your development workflows. It provides a single interface for Docker, databases, testing, CI/CD, and custom project commands - all while automatically detecting what's available in your project.

Born from the need to eliminate duplicated dev tooling across multiple projects, devkit extracts the 70% of dev-cli patterns that are identical across projects while letting you keep the 30% that's unique to each codebase.

## Key Features

### 🎯 Smart Feature Detection
Commands automatically hide if features aren't detected:
- No `docker-compose.yml`? No docker commands
- No `[database]` sections? No database commands
- No `.github/workflows`? No CI commands

### 📦 Package Command System
Define commands once in `dev.toml`, run anywhere:
```toml
[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
deps = ["common:build"]  # Automatic dependency resolution

[cmd.test]
default = "cargo test"
```

Then run:
```bash
./dev.sh cmd build --watch  # Automatic variant selection
./dev.sh cmd test           # Respects dependencies
```

### ✨ Production-Ready Features

**Better Error Messages**
```
Error: Docker compose failed: Cannot connect to Docker daemon
Try: Make sure Docker is running: docker info
```

**Early Validation**
```
Configuration validation failed:
  ✗ Circular dependency detected: api:build -> common:build -> api:build
  ✗ Invalid dependency 'nonexistent:build' in api:test - dependency not found
```

**Progress Indicators**
```
⠋ Starting containers...
✓ Containers started
```

**Shell Completions**
```bash
devkit completions bash > /etc/bash_completion.d/devkit
devkit completions zsh > /usr/local/share/zsh/site-functions/_devkit
```

**Structured Logging**
```bash
RUST_LOG=devkit=debug ./dev.sh status
```

### 🔧 Extend or Use Standalone
- **Library**: Import `devkit-core` and `devkit-tasks` for custom CLIs
- **Standalone**: Use the default `devkit` binary
- **Both**: Mix and match as needed

## Architecture

```
devkit/
├── crates/
│   ├── devkit-core/       ✅ Config, context, detection, errors, validation
│   ├── devkit-tasks/      ✅ Command discovery, execution, dependencies
│   └── devkit-cli/        ✅ Kitchen sink CLI binary
│
└── extensions/
    ├── devkit-ext-docker/    ✅ Docker compose operations
    ├── devkit-ext-deps/      ✅ Dependency detection & installation
    ├── devkit-ext-database/  ✅ Database migrations & seeds
    ├── devkit-ext-quality/   ✅ Format, lint, test
    ├── devkit-ext-git/       🚧 Git workflows
    ├── devkit-ext-ci/        🚧 CI integration
    ├── devkit-ext-env/       🚧 Environment management
    ├── devkit-ext-tunnel/    🚧 Tunneling services
    ├── devkit-ext-test/      🚧 Test orchestration
    ├── devkit-ext-benchmark/ 🚧 Benchmarking
    ├── devkit-ext-ecs/       🚧 ECS deployment
    └── devkit-ext-pulumi/    🚧 Pulumi infrastructure
```

### devkit-core
Core abstractions for all devkit tools:
- **Config system**: Discovers packages, loads `.dev/config.toml` and `dev.toml`
- **Feature detection**: Auto-detect Docker, Git, databases, CI, mobile, etc.
- **Error handling**: Structured errors with helpful suggestions
- **Validation**: Early detection of circular dependencies, invalid configs
- **AppContext**: Shared state, theming, quiet mode
- **Utilities**: Repo root detection, command checks, browser opening

### devkit-tasks
Task discovery and execution engine:
- **Command discovery**: Find commands in package `dev.toml` files
- **Dependency resolution**: Topological sort with circular dependency detection
- **Parallel execution**: Run independent commands concurrently
- **Variant support**: `build:watch`, `lint:fix`, etc.

### devkit-cli
Kitchen sink CLI with all features:
- Interactive menu system
- Docker operations (up, down, restart, logs, shell)
- Database management (migrate, seed, reset, shell)
- Code quality tools (fmt, lint, test)
- Dependency management (auto-install)
- Shell completions (bash, zsh, fish, powershell)
- Structured logging with tracing

### Extensions
Modular functionality you can include:
- **docker**: Docker Compose operations with progress indicators
- **deps**: Smart dependency detection and installation
- **database**: Database migrations, seeds, and shell access
- **quality**: Format, lint, and test orchestration
- More coming soon...

## Quick Start

### 🚀 Install from Source

```bash
# Clone and build
git clone https://github.com/crcn/devkit
cd devkit
cargo build --release

# Binary at target/release/devkit
```

### Run Commands
```bash
devkit                    # Interactive menu
devkit start             # Start development environment
devkit docker up         # Start Docker containers
devkit cmd build         # Run package build commands
devkit cmd test          # Run package tests
devkit doctor            # Check system health
devkit completions bash  # Generate shell completions
```

### Enable Debug Logging
```bash
RUST_LOG=devkit=debug devkit status
RUST_LOG=devkit=trace devkit cmd build
```

## Configuration

### Global Config (`.dev/config.toml`)

```toml
[project]
name = "my-project"

[workspaces]
packages = ["packages/*", "apps/*"]
exclude = ["packages/legacy"]

[environments]
available = ["dev", "staging", "prod"]
default = "dev"

[services]
api = 8080
postgres = 5432

[urls.playground]
label = "GraphQL Playground"
url = "http://localhost:8080/playground"
```

### Package Config (`packages/*/dev.toml`)

```toml
# Database migrations
[database]
migrations = "migrations"
seeds = "seeds/dev.sql"

# Commands with variants and dependencies
[cmd]
test = "cargo test"

[cmd.build]
default = "cargo build"
release = "cargo build --release"
watch = "cargo watch -x build"
deps = ["common:build", "utils:build"]

[cmd.lint]
default = "cargo clippy"
fix = "cargo clippy --fix"
```

## Use as a Library

### Basic Usage
```rust
use devkit_core::{AppContext, Result};
use devkit_tasks::{run_cmd, CmdOptions};

fn main() -> Result<()> {
    // Create context (loads config, validates, detects features)
    let ctx = AppContext::new(false)?;

    // Feature detection
    if ctx.features.docker {
        println!("Docker available!");
    }

    // Run package commands
    let opts = CmdOptions::default();
    let results = run_cmd(&ctx, "test", &opts)?;

    Ok(())
}
```

### Error Handling
```rust
use devkit_core::{DevkitError, Result};

fn example() -> Result<()> {
    // Errors include helpful context
    Err(DevkitError::feature_not_available(
        "docker".to_string(),
        "Install from https://docker.com".to_string(),
    ))
}
```

### Custom Extension
```rust
use devkit_core::{AppContext, Extension, MenuItem, Result};

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my-extension"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Check if this extension should be enabled
        ctx.features.docker
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![MenuItem {
            label: "🚀 My Command".to_string(),
            handler: Box::new(|ctx| {
                println!("Running my command!");
                Ok(())
            }),
        }]
    }
}
```

## Development Roadmap

### Core (Complete ✅)
- [x] Config system with validation
- [x] Feature detection
- [x] Task execution engine
- [x] Dependency resolution with cycle detection
- [x] Error handling with structured errors
- [x] Extension system
- [x] Progress indicators
- [x] Shell completions
- [x] Structured logging
- [x] Test suite (20+ tests)

### Extensions (In Progress 🚧)
- [x] Docker operations
- [x] Dependency management
- [x] Database operations
- [x] Code quality tools
- [ ] Git workflows
- [ ] CI integration
- [ ] Environment management
- [ ] Tunnel services
- [ ] Test orchestration
- [ ] Benchmarking
- [ ] ECS deployment
- [ ] Pulumi infrastructure

### Future
- [ ] Init command for new projects
- [ ] One-line installer script
- [ ] Publish to crates.io
- [ ] Additional language support (Python, Go, TypeScript)
- [ ] Plugin marketplace

Current progress: **~60%** (core complete, extensions in progress)

## Testing

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package devkit-core --test validation_tests
```

Test coverage:
- Configuration loading and parsing
- Error message formatting
- Circular dependency detection
- Command validation
- Utility functions
- Invalid config handling

## Why devkit?

**Problem**: Multiple projects with similar dev workflows, lots of duplicated code.

**Solution**: Extract the 70% that's identical into `devkit`, keep the 30% that's unique.

**Result**:
- ✅ Shared infrastructure across all projects
- ✅ Project-specific extensions stay in project
- ✅ Each project pins the devkit version that works for them
- ✅ No coordination overhead
- ✅ Production-ready error handling
- ✅ Validated configurations catch issues early
- ✅ Visual feedback for all operations
- ✅ Comprehensive test coverage

## Examples

### Basic Workflow
```bash
# Check system
devkit doctor

# Start everything
devkit start

# Run tests across all packages
devkit cmd test

# Format and lint
devkit fmt
devkit lint --fix

# Docker operations
devkit docker up
devkit docker logs
devkit docker shell
```

### Advanced Usage
```bash
# Run command with variant
devkit cmd build:watch

# Run for specific package
devkit cmd test -p api

# Parallel execution
devkit cmd test --parallel

# With debug logging
RUST_LOG=devkit=debug devkit cmd build

# Generate completions
devkit completions zsh > ~/.zshrc.d/devkit
```

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - System design and flow
- [IMPROVEMENTS_SUMMARY.md](IMPROVEMENTS_SUMMARY.md) - Recent improvements
- [DOGFOODING.md](DOGFOODING.md) - Using devkit to build devkit
- [examples/](examples/) - Example custom CLIs

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! This project follows pragmatic, non-over-engineered design principles:
- Real user value over speculative features
- Simple solutions over complex abstractions
- Tests for actual functionality
- Clear error messages with suggestions

See issues for areas where help is needed.
