# Smart Dependency Installation

devkit now automatically detects and installs dependencies for your entire workspace!

## Overview

The `devkit-ext-deps` extension provides:

- **Auto-detection** of package managers and languages
- **Smart installation** - only installs when needed (checks timestamps)
- **Workspace-aware** - discovers all packages via glob patterns
- **Prerun hooks** - automatically runs on startup before any commands

## Supported Languages & Package Managers

### JavaScript/TypeScript
- **npm** (package-lock.json)
- **yarn** (yarn.lock)
- **pnpm** (pnpm-lock.yaml)
- **bun** (bun.lockb)

Detects TypeScript via `tsconfig.json`

### Rust
- **cargo** (Cargo.toml)

### Python
- **pip** (requirements.txt)
- **poetry** (pyproject.toml with [tool.poetry])
- **pipenv** (Pipfile)
- **uv** (pyproject.toml with [tool.uv])

### Ruby
- **bundler** (Gemfile)

### Go
- **go mod** (go.mod)

### Java
- **maven** (pom.xml)
- **gradle** (build.gradle)

### PHP
- **composer** (composer.json)

### .NET/C#
- **dotnet** (*.csproj, *.fsproj, *.vbproj)

### Elixir
- **mix** (mix.exs)

## How It Works

### 1. Discovery

Uses glob patterns from `.dev/config.toml`:

```toml
[workspaces]
packages = ["packages/*", "apps/*", "services/*"]
exclude = ["legacy"]
```

Searches all patterns and detects:
- What language each package uses
- What package manager it needs
- Whether dependencies need installing

### 2. Smart Detection

Only installs when needed by checking:

- **Node**: Compare lock file/package.json vs node_modules timestamp
- **Rust**: Compare Cargo.toml vs Cargo.lock timestamp
- **Python**: Compare requirements.txt/pyproject.toml vs venv
- **Ruby**: Compare Gemfile vs Gemfile.lock
- **Go**: Compare go.mod vs go.sum
- And more...

### 3. Automatic Installation

On startup (via prerun hook):

```bash
./dev.sh start
# Automatically detects and installs dependencies before starting!
```

Or manually:

```bash
./dev.sh deps          # Install dependencies
./dev.sh deps --list   # Show what was discovered
```

## Configuration

Enable in `.dev/config.toml`:

```toml
[features]
deps = true  # Enable auto-dependency installation
```

Or in your custom CLI's `Cargo.toml`:

```toml
[dependencies]
devkit-ext-deps = { path = "../extensions/devkit-ext-deps" }
```

## Example Output

```
Found 3 package(s) that need dependencies installed
  api-server [Rust] via cargo
  web-app [TypeScript] via pnpm
  scripts [Python] via poetry

Install dependencies now? (Y/n)

Installing dependencies for 3 package(s)...
  Installing Rust dependencies in api-server...
  Installing TypeScript dependencies in web-app...
  Installing Python dependencies in scripts...
✓ All dependencies installed
```

## Prerun Hook System

Extensions can implement a `prerun()` hook that runs automatically on startup:

```rust
impl Extension for DepsExtension {
    fn prerun(&self, ctx: &AppContext) -> Result<()> {
        // Check and install dependencies
        check_and_install(ctx)
    }
}
```

The kitchen sink CLI automatically runs all prerun hooks before commands execute.

## Architecture

Following devkit's principles:

- **Core stays pure** - Detection logic is in an extension, not core
- **Extension infrastructure** - Prerun hooks are in core as they're foundational
- **Configurable** - Enable/disable via features
- **Smart defaults** - Works out of the box with zero config

## Files

- `extensions/devkit-ext-deps/` - The extension
  - `src/detection.rs` - Package & language detection
  - `src/install.rs` - Installation logic
  - `src/extension_impl.rs` - Extension trait implementation
- `.claude/rules.md` - Architecture rules (keeps core pure)
- `crates/devkit-core/src/extension.rs` - Prerun hook infrastructure

## Future Enhancements

Possible additions:

- [ ] Parallel installation (install multiple packages concurrently)
- [ ] Cache awareness (check if deps are already cached)
- [ ] Dry-run mode (show what would be installed)
- [ ] Install specific packages only
- [ ] Version pinning/constraints
- [ ] Pre-install hooks (run scripts before installing)
- [ ] Post-install hooks (run scripts after installing)
