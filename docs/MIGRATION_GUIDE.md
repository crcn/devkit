# Migration Guide: Embracing [cmd] - Lean CLI Philosophy

## Overview

Recent updates to devkit embrace a **lean CLI philosophy**: instead of building every feature into devkit itself, projects define their own commands via `[cmd]` entries in `dev.toml` files.

This guide helps you migrate from built-in extensions to project-specific commands.

---

## Quick Start

### 1. Auto-Generate dev.toml Files

The fastest way to migrate:

```bash
cd your-project
devkit init

# Output:
# Scanning for packages...
#   ✓ Created packages/server/dev.toml
#   ✓ Created packages/web-app/dev.toml
# ✓ Generated 2 package dev.toml file(s)
```

This scans your project and creates `dev.toml` files with detected commands.

### 2. Try the Interactive Menu

```bash
devkit

# You'll see commands from your dev.toml files:
# 🔨 Build
# 🔨 Build (watch)
# 🔨 Build (release)
# 🧪 Test
# 🧪 Test (watch)
# 🔍 Lint
# 🔍 Lint (fix)
# 💅 Fmt
# 💅 Fmt (fix)
```

---

## Migrating from Quality Extension

### What Changed

The **quality extension** (fmt, lint, test) is now redundant because:
1. Commands extension surfaces `[cmd.fmt]`, `[cmd.lint]`, `[cmd.test]` directly
2. Each project can define its own lint/fmt/test commands
3. No more one-size-fits-all approach

### Migration Steps

#### For Rust Projects

Create `dev.toml` in your package:

```toml
[cmd.lint]
default = "cargo clippy --all-targets --all-features -- -D warnings"
fix = "cargo clippy --fix --allow-dirty --allow-staged"

[cmd.fmt]
default = "cargo fmt --all --check"
fix = "cargo fmt --all"

[cmd.typecheck]
default = "cargo check --all-targets"

[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
release = "cargo build --release"

[cmd]
test = "cargo test"
```

#### For Node/TypeScript Projects

Create `dev.toml` in your package:

```toml
[cmd.lint]
default = "npm run lint"
fix = "npm run lint -- --fix"

[cmd.fmt]
default = "npm run format -- --check"
fix = "npm run format"

[cmd.typecheck]
default = "npm run typecheck"

[cmd.build]
default = "npm run build"
watch = "npm run dev"

[cmd]
test = "npm run test"
```

Or for Yarn:

```toml
[cmd.lint]
default = "yarn lint"
fix = "yarn lint --fix"

[cmd.fmt]
default = "yarn format --check"
fix = "yarn format"

[cmd.typecheck]
default = "yarn typecheck"

[cmd.build]
default = "yarn build"

[cmd]
test = "yarn test"
```

### Running Commands

**Via Interactive Menu:**
```bash
devkit
# Select "🔍 Lint (fix)"
```

**Via CLI:**
```bash
devkit cmd lint              # Run default variant
devkit cmd lint --variant fix  # Run fix variant
devkit cmd test --package server  # Run for specific package
```

---

## Migrating from Test Extension

### What Changed

The **test extension** is disabled by default because:
1. Most projects should use `[cmd.test]` with variants
2. Coverage can be added as a variant if needed
3. Project-specific test commands are more flexible

### Migration Steps

Add test command to your `dev.toml`:

**Basic:**
```toml
[cmd]
test = "cargo test"
```

**With Variants:**
```toml
[cmd.test]
default = "cargo test"
watch = "cargo watch -x test"
coverage = "cargo tarpaulin --out Html"
```

**For Node:**
```toml
[cmd.test]
default = "npm test"
watch = "npm test -- --watch"
coverage = "npm test -- --coverage"
```

### Running Tests

```bash
devkit cmd test                    # Run default
devkit cmd test --variant watch    # Run with watch
devkit cmd test --variant coverage # Run with coverage
```

---

## Understanding Command Variants

Command variants allow multiple ways to run the same command:

### Common Patterns

```toml
[cmd.build]
default = "cargo build"           # Normal build
watch = "cargo watch -x build"    # Auto-rebuild on changes
release = "cargo build --release" # Optimized build
```

```toml
[cmd.lint]
default = "eslint ."              # Check only
fix = "eslint . --fix"            # Fix issues
```

```toml
[cmd.fmt]
default = "prettier --check ."    # Check formatting
fix = "prettier --write ."        # Fix formatting
```

### In the Menu

Each variant appears as a separate menu item:
- "🔨 Build" → runs `default`
- "🔨 Build (watch)" → runs `watch` variant
- "🔨 Build (release)" → runs `release` variant

---

## Customizing for Your Project

### Example: Monorepo with Multiple Languages

```toml
# packages/rust-api/dev.toml
[cmd.build]
default = "cargo build -p rust-api"
watch = "cargo watch -x 'build -p rust-api'"
release = "cargo build -p rust-api --release"

[cmd.lint]
default = "cargo clippy -p rust-api"
fix = "cargo clippy -p rust-api --fix"

[cmd]
test = "cargo test -p rust-api"
```

```toml
# packages/web-app/dev.toml
[cmd.build]
default = "yarn build"
watch = "yarn dev"

[cmd.lint]
default = "yarn lint"
fix = "yarn lint --fix"

[cmd.typecheck]
default = "yarn typecheck"

[cmd]
test = "yarn test"
```

### Example: Custom Scripts

```toml
[cmd.deploy]
default = "sh scripts/deploy.sh staging"
prod = "sh scripts/deploy.sh production"

[cmd.seed]
default = "cargo run --bin seed -- --env dev"
prod = "cargo run --bin seed -- --env prod"

[cmd.benchmark]
default = "cargo bench"
baseline = "cargo bench -- --save-baseline"
```

---

## Benefits of [cmd] Approach

### ✅ Flexibility
- Each project defines exactly what it needs
- No generic assumptions about your tooling
- Easy to add custom scripts

### ✅ Discoverability
- All commands visible in interactive menu
- Variants clearly labeled
- No hidden functionality

### ✅ Simplicity
- No complex extension configuration
- TOML is easy to read and write
- Copy patterns from other projects

### ✅ Performance
- No runtime detection overhead
- Commands run exactly as specified
- No wrapper layers

---

## Advanced Patterns

### Dependencies

Run commands in order:

```toml
[cmd.build]
default = "cargo build"
deps = ["proto:build", "common:build"]
```

### Package-Specific

Target specific packages:

```bash
devkit cmd build --package server
devkit cmd test --package api
```

### Parallel Execution

Run across packages in parallel:

```bash
devkit cmd lint --parallel
```

---

## Keeping Quality Extension (Optional)

If you prefer the old quality extension:

Add to `.dev/config.toml`:
```toml
[features]
quality = true
```

The extension will reappear in the menu. But we recommend using `[cmd]` entries instead for better flexibility.

---

## Getting Help

```bash
devkit cmd --help        # See all command options
devkit cmd --list        # List available commands
devkit init --help       # See init options
```

---

## Summary

**Old Way:**
- Built-in extensions for lint/fmt/test
- One-size-fits-all approach
- Limited customization

**New Way:**
- Project defines commands in `dev.toml`
- Complete flexibility
- Discoverable in interactive menu
- Lean CLI, rich commands

**Migration:**
```bash
devkit init  # Auto-generates dev.toml files
devkit       # Try the new menu
```

Welcome to the lean devkit era! 🎉
