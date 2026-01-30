# Extracting Extensions from Shay

## Current State

- ✅ devkit-core - Working
- ✅ devkit-tasks - Working
- ⚠️ devkit-ext-* - Stubbed out, not implemented
- ✅ devkit-cli - Structure ready, waiting for extensions

## Goal

Extract working implementations from `shay/dev/cli/src/cmd/` into devkit extensions.

## Extraction Process

### 1. Docker Extension

**Source:** `~/Developer/fourthplaces/shay/dev/cli/src/cmd/docker.rs`

**Extract functions:**
```rust
// From shay
pub fn docker_compose_up(ctx: &AppContext, services: &[String], build: bool) -> Result<()>
pub fn docker_compose_down(ctx: &AppContext) -> Result<()>
pub fn docker_compose_restart(ctx: &AppContext, services: &[String]) -> Result<()>
pub fn attach_container_with_logs(ctx: &AppContext, service: &str) -> Result<()>
pub fn docker_shell(ctx: &AppContext, service: Option<&str>) -> Result<()>
```

**Target:** `devkit/extensions/devkit-ext-docker/src/lib.rs`

```rust
// devkit-ext-docker/src/lib.rs
use devkit_core::AppContext;
use anyhow::Result;

pub fn compose_up(ctx: &AppContext, services: &[String], build: bool) -> Result<()> {
    // Copy implementation from shay
    todo!()
}

pub fn compose_down(ctx: &AppContext) -> Result<()> {
    todo!()
}

// ... etc
```

### 2. Database Extension

**Source:** `~/Developer/fourthplaces/shay/dev/cli/src/cmd/db.rs`

**Extract functions:**
```rust
pub fn db_migrate_with_config(ctx: &AppContext, config: Option<&Config>, env: Option<&str>) -> Result<()>
pub fn db_reset(ctx: &AppContext) -> Result<()>
pub fn db_seed(ctx: &AppContext) -> Result<()>
pub fn db_psql(ctx: &AppContext) -> Result<()>
```

**Target:** `devkit/extensions/devkit-ext-database/src/lib.rs`

### 3. Quality Extension

**Source:** `~/Developer/fourthplaces/shay/dev/cli/src/cmd/quality.rs`

**Extract functions:**
```rust
pub fn run_fmt(ctx: &AppContext, fix: bool, quiet: bool) -> Result<()>
pub fn run_lint(ctx: &AppContext, fix: bool, ai_fix: bool) -> Result<()>
pub fn run_check(ctx: &AppContext) -> Result<()>
```

**Target:** `devkit/extensions/devkit-ext-quality/src/lib.rs`

### 4. Test Extension

**Source:** `~/Developer/fourthplaces/shay/dev/cli/src/cmd/test.rs`

**Extract functions:**
```rust
pub fn run_tests(ctx: &AppContext, package: Option<&str>, filter: Option<&str>, quiet: bool) -> Result<()>
pub fn watch_tests(ctx: &AppContext) -> Result<()>
```

**Target:** `devkit/extensions/devkit-ext-test/src/lib.rs`

### 5. CI Extension

**Source:** `~/Developer/fourthplaces/shay/dev/cli/src/cmd/ci.rs`

**Extract functions:**
```rust
pub fn ci_status(ctx: &AppContext) -> Result<()>
pub fn ci_runs(ctx: &AppContext, limit: u32, workflow: Option<&str>) -> Result<()>
pub fn ci_watch(ctx: &AppContext, run_id: Option<&str>) -> Result<()>
// ... etc
```

**Target:** `devkit/extensions/devkit-ext-ci/src/lib.rs`

## Step-by-Step Example: Docker Extension

### Step 1: Create Extension Crate

```bash
cd devkit/extensions/devkit-ext-docker
mkdir -p src
```

### Step 2: Copy Cargo.toml

```toml
# extensions/devkit-ext-docker/Cargo.toml
[package]
name = "devkit-ext-docker"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Docker compose operations for devkit"

[dependencies]
anyhow.workspace = true
devkit-core.workspace = true
```

### Step 3: Extract Functions

```bash
# Copy relevant functions from shay
cp ~/Developer/fourthplaces/shay/dev/cli/src/cmd/docker.rs \
   extensions/devkit-ext-docker/src/lib.rs

# Edit to make it a library
# - Change pub fn ... to public API
# - Remove shay-specific dependencies
# - Use devkit-core::AppContext
```

### Step 4: Update devkit-cli to Use It

```toml
# crates/devkit-cli/Cargo.toml
[dependencies]
devkit-ext-docker = { path = "../../extensions/devkit-ext-docker", optional = true }

[features]
docker = ["devkit-ext-docker"]
```

```rust
// crates/devkit-cli/src/main.rs
#[cfg(feature = "docker")]
use devkit_ext_docker;

#[cfg(feature = "docker")]
fn handle_docker(ctx: &AppContext, action: DockerAction) -> Result<()> {
    match action {
        DockerAction::Up => devkit_ext_docker::compose_up(ctx, &[], false),
        DockerAction::Down => devkit_ext_docker::compose_down(ctx),
        // ... etc
    }
}
```

### Step 5: Test

```bash
cd devkit
cargo build -p devkit-cli --features docker
./target/debug/devkit docker up
```

## Dependencies to Extract

Some utilities from shay might be needed:

### cmd_builder.rs
Shay has a `CmdBuilder` - might want to extract to devkit-core or keep in each extension.

### compose.rs
Docker compose utilities - extract to devkit-ext-docker.

### services.rs
Service detection - might belong in devkit-core or devkit-ext-docker.

## Priority Order

Recommended extraction order based on usefulness:

1. **devkit-ext-docker** - Most universally useful
2. **devkit-ext-database** - Common need
3. **devkit-ext-quality** - Fmt/lint/test
4. **devkit-ext-ci** - GitHub Actions integration
5. **devkit-ext-env** - Environment variable management
6. **devkit-ext-deploy** - Pulumi deployment
7. Rest as needed

## Testing Strategy

For each extension:

1. **Unit tests** - Test core logic
2. **Integration tests** - Test with AppContext
3. **Example project** - Small project using the extension
4. **Kitchen sink test** - Verify it works in devkit-cli

## Example: Complete Docker Extension

```rust
// extensions/devkit-ext-docker/src/lib.rs
use anyhow::Result;
use devkit_core::{AppContext, utils::docker_compose_program};
use std::process::Command;

pub fn compose_up(ctx: &AppContext, services: &[String], build: bool) -> Result<()> {
    let (program, mut args) = docker_compose_program()?;
    args.push("up".to_string());
    args.push("-d".to_string());

    if build {
        args.push("--build".to_string());
    }

    args.extend(services.iter().map(|s| s.to_string()));

    ctx.print_info("Starting containers...");

    let status = Command::new(&program)
        .args(&args)
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Docker compose up failed"));
    }

    ctx.print_success("✓ Containers started");
    Ok(())
}

pub fn compose_down(ctx: &AppContext) -> Result<()> {
    let (program, mut args) = docker_compose_program()?;
    args.push("down".to_string());

    ctx.print_info("Stopping containers...");

    let status = Command::new(&program)
        .args(&args)
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Docker compose down failed"));
    }

    ctx.print_success("✓ Containers stopped");
    Ok(())
}

// Add more functions...
```

## Workspace Configuration

Update main `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/devkit-core",
    "crates/devkit-tasks",
    "crates/devkit-cli",
    # Add extensions
    "extensions/devkit-ext-docker",
    "extensions/devkit-ext-database",
    "extensions/devkit-ext-quality",
    # ... etc
]
```

## Benefits of Extraction

- ✅ Reusable across projects
- ✅ Independently versioned
- ✅ Optional (via features)
- ✅ Testable in isolation
- ✅ Community can contribute
- ✅ Clear boundaries

## After Extraction

Once extensions are extracted:

1. **Update kitchen sink CLI** - Uncomment extension usage
2. **Test thoroughly** - Verify all commands work
3. **Document** - Add examples for each extension
4. **Publish** - Make available via GitHub
5. **Iterate** - Improve based on usage

## Timeline Estimate

- Docker extension: 2-3 hours
- Database extension: 2-3 hours
- Quality extension: 1-2 hours
- CI extension: 2-3 hours
- Each additional: 1-3 hours

Total for top 4 extensions: ~1-2 days of focused work

## You're Ready!

The infrastructure is in place:
- ✅ Installer with two modes
- ✅ Kitchen sink CLI structure
- ✅ Feature flag system
- ✅ Config system

Now just extract the working code from shay into the extensions!
