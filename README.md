# devkit

**A comprehensive development environment orchestration toolkit written in Rust.**

> 🚧 **Status**: Early development - Phase 2 complete (task system extracted)

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
deps = ["common:build"]  # Dependency resolution

[cmd.test]
default = "cargo test"
```

Then run:
```bash
./dev.sh cmd build --watch  # Automatic variant selection
./dev.sh cmd test           # Respects dependencies
```

### 🚀 Zero-Friction Setup
One command to initialize any project:
```bash
devkit init  # Creates dev.sh, .dev/config.toml, detects project type
./dev.sh     # Interactive menu with auto-detected features
```

### 🔧 Extend or Use Standalone
- **Library**: Import `devkit-core` and `devkit-tasks` for custom CLIs
- **Standalone**: Use the default `devkit` binary
- **Both**: Mix and match as needed

## Architecture

```
devkit/
├── devkit-core/       ✅ Config, context, detection
├── devkit-compose/    🚧 Docker operations (TODO)
├── devkit-tasks/      ✅ Command discovery & execution
└── devkit-cli/        🚧 CLI binary (TODO)
```

### devkit-core
Core abstractions for all devkit tools:
- **Config system**: Discovers packages, loads `.dev/config.toml` and `dev.toml`
- **Feature detection**: Auto-detect Docker, Git, databases, CI, mobile, etc.
- **AppContext**: Shared state, theming, quiet mode
- **Utilities**: Repo root detection, command checks, browser opening

### devkit-tasks
Task discovery and execution engine:
- **Command discovery**: Find commands in package `dev.toml` files
- **Dependency resolution**: Topological sort, circular dependency detection
- **Parallel execution**: Run independent commands concurrently
- **Variant support**: `build:watch`, `lint:fix`, etc.

### devkit-compose (TODO - Phase 3)
Docker compose operations

### devkit-cli (TODO - Phase 4)
Full CLI with all commands

## Quick Start

### Installation (when published)
```bash
cargo install devkit-cli
```

### Initialize Your Project
```bash
cd your-project
devkit init
```

This creates:
- `dev.sh` - Auto-installs Rust and devkit, then runs commands
- `.dev/config.toml` - Global config with detected values
- Sample `dev.toml` files (optional)

### Run Commands
```bash
./dev.sh              # Interactive menu
./dev.sh cmd test     # Run tests
./dev.sh cmd build    # Build packages
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

```rust
use devkit_core::AppContext;
use devkit_tasks::{run_cmd, CmdOptions};

fn main() -> anyhow::Result<()> {
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

## Development Roadmap

- [x] **Phase 1**: Core infrastructure (config, context, detection)
- [x] **Phase 2**: Task system (command discovery, execution, dependencies)
- [ ] **Phase 3**: Docker operations
- [ ] **Phase 4**: CLI commands (test, fmt, lint, etc.)
- [ ] **Phase 5**: Init command
- [ ] **Phase 6**: Integration testing & crates.io release

Current progress: **~30%**

## Why devkit?

**Problem**: Multiple projects with similar dev workflows, lots of duplicated code.

**Solution**: Extract the 70% that's identical into `devkit`, keep the 30% that's unique.

**Result**:
- Shared infrastructure across all projects
- Project-specific extensions stay in project
- Each project pins the devkit version that works for them
- No coordination overhead

## License

MIT OR Apache-2.0

## Contributing

See [EXTRACTING.md](EXTRACTING.md) for extraction progress and [STATUS.md](STATUS.md) for current state.
