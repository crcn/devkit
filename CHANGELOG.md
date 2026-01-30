# Changelog

## [Unreleased] - 2026-01-30

### Added 🎉

#### Commands Extension - Project-Specific Commands in Menu
- **New `devkit-ext-commands` extension** surfaces all `[cmd.*]` entries in the interactive menu
- Supports command variants (default, watch, release, fix, etc.)
- Auto-generates emoji-tagged menu items based on command names
- Example: `[cmd.build]` with `default`, `watch`, `release` shows as 3 menu items

#### Docker Multi-Container Support
- **Multi-container log following** - Select multiple containers to follow simultaneously
- Space bar to multi-select, "[All]" option for all containers
- Live streaming with `docker compose logs -f`
- Interactive selection for all Docker operations (up, restart, build, shell)

#### Smart Devkit Init
- **Auto-generates `dev.toml` files** for all packages in your project
- Scans for Rust (`Cargo.toml`) and Node (`package.json`) packages
- Detects capabilities: build, lint, fmt, typecheck, test, dev
- Generates appropriate `[cmd]` entries with variants
- Keeps devkit lean by using project-specific commands

Example:
```bash
cd my-monorepo
devkit init
# ✓ Created packages/server/dev.toml
# ✓ Created packages/web-app/dev.toml
# ✓ Created packages/api/dev.toml
```

#### Feature Detection Improvements
- **Pulumi detection** - Checks for `Pulumi.yaml`/`Pulumi.yml` files + CLI
- **Test detection** - Checks for test directories and `[cmd.test]` entries
- **Commands detection** - Detects packages with `[cmd]` sections
- Kitchen sink philosophy: Menu items only appear when features are actually present

### Changed 🔄

#### Extension System
- `Extension::menu_items()` now receives `&AppContext` parameter
- Enables dynamic menu generation based on project state
- All extensions updated to new signature

#### Quality & Test Extensions
- **Quality extension** - Now opt-in only (redundant with commands extension)
- **Test extension** - Disabled by default (use `[cmd.test]` instead)
- Projects should use `[cmd]` entries for lint, fmt, test
- Extensions still available if explicitly enabled in config

### Fixed 🐛

#### Dependency Installation
- Removed annoying auto-prompt on every `devkit` run
- Users must explicitly run `devkit deps` when needed
- No more interrupting workflow with dependency checks

### Migration Guide

#### From Quality Extension to [cmd]

**Before (using quality extension):**
```bash
devkit
# Select "✨ Format (check)"
# Select "✨ Lint (fix)"
```

**After (using [cmd] - recommended):**

Add to your `dev.toml`:
```toml
[cmd.lint]
default = "cargo clippy --all-targets -- -D warnings"
fix = "cargo clippy --fix --allow-dirty"

[cmd.fmt]
default = "cargo fmt --all --check"
fix = "cargo fmt --all"
```

Then:
```bash
devkit init  # Auto-generates dev.toml if not present
devkit
# Select "🔍 Lint"
# Select "🔍 Lint (fix)"
# Select "💅 Fmt"
# Select "💅 Fmt (fix)"
```

#### From Test Extension to [cmd.test]

**Before:**
```bash
devkit
# Select "🧪 Test - Run All"
# Select "🧪 Test - Watch"
```

**After:**

Add to your `dev.toml`:
```toml
[cmd.test]
default = "cargo test"
watch = "cargo watch -x test"
```

Then:
```bash
devkit
# Select "🧪 Test"
# Select "🧪 Test (watch)"
```

### Breaking Changes ⚠️

- `Extension::menu_items()` signature changed - now takes `&AppContext`
- Quality extension disabled by default - set `quality = true` in `.dev/config.toml` to re-enable
- Test extension disabled by default - use `[cmd.test]` instead

### Performance

- Faster menu generation with cached command detection
- Reduced extension overhead by removing redundant wrappers

---

## Philosophy: Lean CLI + Rich Commands

Instead of building everything into devkit, projects define their own commands:

**Old Way (Heavy):**
- Devkit has built-in lint/fmt/test logic
- Works for some projects, not others
- Hard to customize

**New Way (Lean):** ✅
- `devkit init` scans and generates `dev.toml`
- Each project defines its own commands
- Devkit surfaces them in the menu
- Complete flexibility, zero assumptions
