# devkit-ext-deps

Smart dependency detection and installation extension for devkit.

## Features

- **Auto-detect** package managers (cargo, npm, yarn, pnpm)
- **Auto-detect** languages (Rust, JavaScript, TypeScript)
- **Smart detection** - only install when needed (checks timestamps)
- **Workspace-aware** - discovers all packages in your workspace

## Usage

```rust
use devkit_core::AppContext;
use devkit_ext_deps::{check_and_install, print_summary};

fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new(false)?;

    // Print what was discovered
    print_summary(&ctx);

    // Install dependencies for packages that need them
    check_and_install(&ctx)?;

    Ok(())
}
```

## Detection Logic

### Rust Packages
- Detects `Cargo.toml`
- Uses `cargo fetch` to install dependencies
- Checks if `Cargo.toml` is newer than `Cargo.lock`

### Node Packages
- Detects `package.json`
- Auto-detects package manager based on lock files:
  - `pnpm-lock.yaml` → pnpm
  - `yarn.lock` → yarn
  - `package-lock.json` → npm (default)
- Detects TypeScript via `tsconfig.json`
- Checks if lock file or `package.json` is newer than `node_modules`

## Integration

Add to your CLI's `Cargo.toml`:

```toml
[dependencies]
devkit-ext-deps = { path = "../extensions/devkit-ext-deps" }
```

Then call `check_and_install()` in your startup command.
