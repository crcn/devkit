# devkit Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         User's Project                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  dev.sh (wrapper)                                                │
│  ├─ Ensures Rust installed                                       │
│  ├─ Builds dev/cli (once)                                        │
│  └─ Executes: ./target/release/dev-cli [args]                    │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │  dev/cli (custom CLI binary)                           │    │
│  │  ┌──────────────────────────────────────────────────┐  │    │
│  │  │  User's Commands                                  │  │    │
│  │  │  - Start/Stop                                     │  │    │
│  │  │  - Deploy                                         │  │    │
│  │  │  - Custom workflows                               │  │    │
│  │  └──────────────────────────────────────────────────┘  │    │
│  │                          ↓                              │    │
│  │  ┌──────────────────────────────────────────────────┐  │    │
│  │  │  devkit Libraries (via Cargo.toml)               │  │    │
│  │  │                                                   │  │    │
│  │  │  ┌────────────────┐  ┌────────────────┐         │  │    │
│  │  │  │  devkit-core   │  │ devkit-tasks   │         │  │    │
│  │  │  ├────────────────┤  ├────────────────┤         │  │    │
│  │  │  │ • Config       │  │ • Discovery    │         │  │    │
│  │  │  │ • Context      │  │ • Execution    │         │  │    │
│  │  │  │ • Detection    │  │ • Dependencies │         │  │    │
│  │  │  │ • Utils        │  │ • Parallel     │         │  │    │
│  │  │  └────────────────┘  └────────────────┘         │  │    │
│  │  │                                                   │  │    │
│  │  │  ┌─────────────────────────────────────────┐    │  │    │
│  │  │  │  Optional Extensions (pick & choose)    │    │  │    │
│  │  │  ├─────────────────────────────────────────┤    │  │    │
│  │  │  │ • devkit-ext-docker                     │    │  │    │
│  │  │  │ • devkit-ext-database                   │    │  │    │
│  │  │  │ • devkit-ext-quality                    │    │  │    │
│  │  │  │ • devkit-ext-ci                         │    │  │    │
│  │  │  │ • devkit-ext-deploy                     │    │  │    │
│  │  │  │ • ... 6 more                            │    │  │    │
│  │  │  └─────────────────────────────────────────┘    │  │    │
│  │  └──────────────────────────────────────────────────┘  │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  .dev/config.toml                                                │
│  ├─ Project settings                                             │
│  ├─ Workspaces                                                   │
│  ├─ Environments                                                 │
│  └─ Services                                                     │
│                                                                   │
│  packages/*/dev.toml                                             │
│  ├─ [cmd] sections                                               │
│  ├─ Command variants                                             │
│  ├─ Dependencies                                                 │
│  └─ Capabilities (database, mobile, etc.)                        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Installation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  curl -fsSL https://raw.../install.sh | sh                       │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│                     install.sh                                    │
├─────────────────────────────────────────────────────────────────┤
│  1. Check dependencies (curl, git)                               │
│  2. Detect project root and type                                 │
│  3. Download templates from GitHub                               │
│  4. Create dev.sh wrapper                                        │
│  5. Create dev/cli/ project                                      │
│  6. Create .dev/config.toml                                      │
│  7. Add example dev.toml to packages                             │
│  8. Update .gitignore                                            │
│  9. Print next steps                                             │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│                    User's Project                                 │
│  ✓ dev.sh                                                        │
│  ✓ dev/cli/Cargo.toml                                            │
│  ✓ dev/cli/src/main.rs                                           │
│  ✓ .dev/config.toml                                              │
│  ✓ packages/*/dev.toml                                           │
└─────────────────────────────────────────────────────────────────┘
```

## Command Execution Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  ./dev.sh cmd build:watch -p api                                 │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  dev.sh checks if rebuild needed                                 │
│  ├─ First run? → Build (30s)                                     │
│  ├─ Source changed? → Rebuild (5s)                               │
│  └─ Cached? → Skip (0s)                                          │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  exec ./target/release/dev-cli cmd build:watch -p api            │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  devkit-core: AppContext::new()                                  │
│  ├─ Find repo root                                               │
│  ├─ Load .dev/config.toml                                        │
│  ├─ Discover packages (glob patterns)                            │
│  ├─ Load packages/*/dev.toml                                     │
│  └─ Detect features (docker, git, etc.)                          │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  Parse command: "build:watch"                                    │
│  ├─ Command: "build"                                             │
│  ├─ Variant: "watch"                                             │
│  └─ Package filter: ["api"]                                      │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  devkit-tasks: Find command                                      │
│  ├─ Search packages for [cmd.build]                              │
│  ├─ Filter to package "api"                                      │
│  └─ Found: packages/api/dev.toml                                 │
│                                                                   │
│     [cmd.build]                                                  │
│     default = "cargo build"                                      │
│     watch = "cargo watch -x run"  ← Select this variant          │
│     deps = ["common:build"]       ← Resolve dependencies         │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  Dependency Resolution (Topological Sort)                        │
│  ├─ api:build depends on common:build                            │
│  ├─ Check common for circular deps                               │
│  └─ Execution order: [common, api]                               │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  Execute Commands                                                 │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ 1. common:build                                            │ │
│  │    $ cargo build                                           │ │
│  │    ✓ Success                                               │ │
│  └────────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ 2. api:build (watch variant)                               │ │
│  │    $ cargo watch -x run                                    │ │
│  │    [Running indefinitely...]                               │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Configuration Hierarchy

```
┌─────────────────────────────────────────────────────────────────┐
│  .dev/config.toml (Global)                                       │
├─────────────────────────────────────────────────────────────────┤
│  [project]                                                       │
│  name = "my-app"                                                 │
│                                                                   │
│  [workspaces]                                                    │
│  packages = ["packages/*"]  ← Discovery patterns                 │
│                                                                   │
│  [environments]                                                  │
│  available = ["dev", "prod"]                                     │
│  default = "dev"                                                 │
│                                                                   │
│  [services]                                                      │
│  api = 8080                                                      │
│  postgres = 5432                                                 │
└─────────────────────────────────────────────────────────────────┘
                      │
                      │ Discovers packages
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  packages/api/dev.toml (Package-specific)                        │
├─────────────────────────────────────────────────────────────────┤
│  [database]                                                      │
│  migrations = "migrations"  ← Declares capability                │
│                                                                   │
│  [cmd.build]                ← Commands with variants             │
│  default = "cargo build"                                         │
│  watch = "cargo watch -x run"                                    │
│  release = "cargo build --release"                               │
│  deps = ["common:build"]    ← Dependencies                       │
│                                                                   │
│  [cmd]                      ← Simple commands                    │
│  test = "cargo test"                                             │
└─────────────────────────────────────────────────────────────────┘
                      │
                      │ Package name inferred from:
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  packages/api/Cargo.toml                                         │
│  [package]                                                       │
│  name = "api-server"  ← Package name used in commands            │
└─────────────────────────────────────────────────────────────────┘
```

## Extension System

```
┌─────────────────────────────────────────────────────────────────┐
│  Your CLI (dev/cli)                                              │
└─────────────────────┬───────────────────────────────────────────┘
                      │
                      │ Cargo.toml dependencies
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  Core Libraries (Always)                                         │
│  ├─ devkit-core                                                  │
│  └─ devkit-tasks                                                 │
└─────────────────────────────────────────────────────────────────┘
                      │
                      │ Optional: Add as needed
                      ↓
┌─────────────────────────────────────────────────────────────────┐
│  Extensions (Pick & Choose)                                      │
│                                                                   │
│  ┌──────────────────┐  ┌──────────────────┐                     │
│  │ devkit-ext-docker│  │devkit-ext-database│                    │
│  ├──────────────────┤  ├──────────────────┤                     │
│  │ • compose_up     │  │ • migrate        │                     │
│  │ • compose_down   │  │ • seed           │                     │
│  │ • logs           │  │ • reset          │                     │
│  │ • shell          │  │ • psql           │                     │
│  └──────────────────┘  └──────────────────┘                     │
│                                                                   │
│  ┌──────────────────┐  ┌──────────────────┐                     │
│  │devkit-ext-quality│  │  devkit-ext-ci   │                     │
│  ├──────────────────┤  ├──────────────────┤                     │
│  │ • run_fmt        │  │ • ci_status      │                     │
│  │ • run_lint       │  │ • ci_runs        │                     │
│  │ • run_check      │  │ • ci_watch       │                     │
│  │ • run_tests      │  │ • ci_trigger     │                     │
│  └──────────────────┘  └──────────────────┘                     │
│                                                                   │
│  ┌──────────────────┐  ┌──────────────────┐                     │
│  │ devkit-ext-env   │  │ devkit-ext-deploy│                     │
│  ├──────────────────┤  ├──────────────────┤                     │
│  │ • pull_env       │  │ • deploy         │                     │
│  │ • push_env       │  │ • preview        │                     │
│  │ • set_var        │  │ • outputs        │                     │
│  └──────────────────┘  └──────────────────┘                     │
│                                                                   │
│  And 5 more extensions...                                        │
└─────────────────────────────────────────────────────────────────┘
```

## Key Design Principles

### 1. Library-First
- devkit is a library, not a framework
- You build YOUR CLI using devkit components
- Maximum flexibility, minimum lock-in

### 2. Configuration over Code
- Commands defined in TOML, not Rust
- Easy to add/modify without rebuilding
- Package maintainers define their own commands

### 3. Smart Defaults
- Auto-detect project type
- Sensible config templates
- Zero config to get started

### 4. Incremental Adoption
- Start minimal (core + tasks)
- Add extensions as needed
- Keep project-specific logic in project

### 5. No Coordination Needed
- Each project pins its devkit version
- Projects upgrade independently
- No breaking changes for others

### 6. Fast Execution
- Binary cached (not script overhead)
- Release mode builds
- ~0.1s startup after first build

### 7. Discoverable
- Interactive menus
- Help text for all commands
- Examples in templates
