# devkit

A comprehensive development environment orchestration toolkit written in Rust.

## Overview

`devkit` provides a unified CLI interface for managing development workflows across:
- Docker container orchestration
- Environment variable management
- Database operations
- Code quality tools (fmt, lint, test)
- Build and watch modes
- CI/CD integration
- Release workflows
- Custom project commands via `dev.toml`

## Architecture

```
devkit/
├── devkit-core      # Core abstractions (context, config, utils)
├── devkit-compose   # Docker compose operations
├── devkit-tasks     # Task discovery and execution engine
└── devkit-cli       # Default CLI binary
```

## Usage

### As a standalone tool

```bash
cargo install devkit-cli
cd your-project
devkit init
devkit start
```

### As a library

```toml
[dependencies]
devkit-core = "0.1"
devkit-tasks = "0.1"
```

Build your own CLI with project-specific extensions:

```rust
use devkit_core::AppContext;
use devkit_tasks::TaskRunner;

fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new(false)?;
    // Add your custom commands here
    Ok(())
}
```

## Configuration

Projects configure devkit via `.dev/config.toml`:

```toml
[global]
name = "my-project"

[global.environments]
available = ["dev", "prod"]
default = "dev"

[global.workspaces]
packages = ["packages/*"]

[docker]
compose_file = "docker-compose.yml"
```

### Package Commands

Packages define their own commands in `dev.toml`:

```toml
[cmd]
test = "npm test"

[cmd.build]
default = "npm run build"
watch = "npm run build:watch"
deps = ["utils:build"]  # Dependencies
```

Run commands:
```bash
devkit cmd test           # Run tests
devkit cmd build --watch  # Build in watch mode
```

## Project Status

🚧 **Early Development** - Extracted from production use in multiple projects with 70% code overlap.

## License

MIT OR Apache-2.0
