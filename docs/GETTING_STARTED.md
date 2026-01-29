# Getting Started with devkit

## Quick Start

### 1. Install devkit (once ready)

```bash
cargo install devkit-cli
```

### 2. Initialize your project

```bash
cd your-project
devkit init
```

This creates:
- `.dev/config.toml` - Global configuration
- `dev.sh` - Convenience wrapper script
- Sample package `dev.toml` files (optional)

### 3. Start developing

```bash
./dev.sh
```

Shows interactive menu with detected features.

## Configuration

### Global Config (`.dev/config.toml`)

```toml
[project]
name = "my-project"

[workspaces]
packages = ["packages/*", "apps/*"]
exclude = []

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

# Custom commands
[cmd]
test = "cargo test"

[cmd.build]
default = "cargo build"
release = "cargo build --release"
watch = "cargo watch -x build"
deps = ["common:build"]  # Run common:build first
```

## Feature Detection

devkit automatically detects what's available:

- **Docker**: Checks for `docker-compose.yml` and docker binary
- **Database**: Checks for packages with `[database]` sections
- **Git**: Checks for `.git` directory
- **CI**: Checks for `.github/workflows`
- **Mobile**: Checks for packages with `[mobile]` sections

Only relevant commands are shown.

## Usage Patterns

### Interactive Mode

```bash
./dev.sh
```

Shows menu with available commands based on detected features.

### Direct Commands

```bash
./dev.sh start         # Start development environment
./dev.sh test          # Run tests
./dev.sh fmt --fix     # Format code
./dev.sh cmd build     # Run build command from dev.toml
```

### Package Commands

Define custom commands in `dev.toml`:

```bash
./dev.sh cmd test           # Run tests
./dev.sh cmd build          # Run default build
./dev.sh cmd build --watch  # Run build:watch variant
```

## Extending devkit

### As a Library

```rust
use devkit_core::{AppContext, Config};

fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new(false)?;

    // Add your custom commands
    if ctx.features.docker {
        println!("Docker available!");
    }

    Ok(())
}
```

### Custom Commands

Add project-specific commands via `dev.toml`:

```toml
[cmd.deploy]
default = "pulumi up"
preview = "pulumi preview"

[cmd.migrate]
default = "./scripts/migrate.sh"
```

Then run:
```bash
./dev.sh cmd deploy
./dev.sh cmd migrate
```
