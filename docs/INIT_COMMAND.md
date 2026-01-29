# Init Command Design

The `devkit init` command sets up a project for devkit usage.

## Goals

1. **Zero friction** - One command to get started
2. **Smart detection** - Detect project type and configure accordingly
3. **Non-destructive** - Don't overwrite existing configs
4. **Minimal setup** - Create only what's needed

## What it Creates

### Always Created

**`dev.sh`** - Convenience wrapper that:
- Ensures Rust is installed (via rustup)
- Builds devkit-cli if source is present
- Installs devkit-cli via cargo install if not present
- Executes devkit with all arguments

**`.dev/config.toml`** - Global config with detected values:
```toml
[project]
name = "detected-from-git-or-directory"

[workspaces]
packages = ["packages/*"]  # Detected from directory structure

[environments]
available = ["dev", "prod"]
default = "dev"
```

### Conditionally Created

**`packages/*/dev.toml`** - If packages detected, offer to create samples:
```toml
# For Rust packages with migrations
[database]
migrations = "migrations"

# For all packages - common commands
[cmd]
test = "cargo test"  # or "npm test" for JS

[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
```

**`docker-compose.yml`** - If user wants Docker, create minimal setup:
```yaml
version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## Detection Logic

```rust
fn detect_project_type(repo_root: &Path) -> ProjectType {
    if has_cargo_workspace(repo_root) {
        ProjectType::RustMonorepo
    } else if has_lerna_or_nx(repo_root) {
        ProjectType::NodeMonorepo
    } else if has_cargo_toml(repo_root) {
        ProjectType::RustSingle
    } else if has_package_json(repo_root) {
        ProjectType::NodeSingle
    } else {
        ProjectType::Generic
    }
}
```

Based on project type:
- Rust: Suggest cargo commands, check for migrations
- Node: Suggest npm/yarn commands
- Generic: Minimal config, let user add commands

## Interactive Prompts

```
🎉 Welcome to devkit!

I'll help you set up your development environment.

✓ Detected: Rust monorepo with 5 packages
✓ Found: docker-compose.yml
✓ Found: .git directory

What would you like to set up?

  [x] Create dev.sh wrapper
  [x] Create .dev/config.toml
  [ ] Generate package dev.toml files
  [ ] Add sample docker-compose.yml (skip - already exists)

Continue? (Y/n)
```

## Implementation Plan

```rust
pub fn init(ctx: &AppContext) -> Result<()> {
    // 1. Detect project type
    let project_type = detect_project_type(&ctx.repo);

    // 2. Show what will be created
    print_init_plan(&project_type);

    // 3. Confirm with user
    if !ctx.confirm("Create these files?", true)? {
        return Ok(());
    }

    // 4. Create files
    create_dev_sh(&ctx.repo)?;
    create_config_toml(&ctx.repo, &project_type)?;

    // 5. Offer optional setup
    if ctx.confirm("Generate package dev.toml samples?", false)? {
        generate_package_configs(&ctx)?;
    }

    // 6. Success message
    ctx.print_success("✓ devkit initialized!");
    ctx.print_info("Run: ./dev.sh");

    Ok(())
}
```

## dev.sh Template

```bash
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export REPO_ROOT="$SCRIPT_DIR"

# Ensure Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install devkit if not present
if ! command -v devkit &> /dev/null; then
    echo "Installing devkit..."
    cargo install devkit-cli
fi

# Run devkit
exec devkit "$@"
```

## Future Enhancements

- **Templates**: Support different project templates (rails, django, nextjs, etc.)
- **Plugins**: Auto-detect and suggest relevant devkit plugins
- **Migration**: Detect existing tools (make, just, etc.) and offer to migrate
- **Updates**: `devkit init --update` to refresh scripts/configs
