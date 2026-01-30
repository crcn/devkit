# Shay → devkit Migration Plan

This document tracks what can be extracted from Shay's dev-cli to devkit.

## ✅ Already Extracted

- **devkit-core**: Config, context, detection, utils
- **devkit-tasks**: Command discovery, execution, dependencies
- **devkit-ext-docker**: Docker compose, logs, shell

## 🎯 Should Extract (Generic Functionality)

### High Priority

#### devkit-ext-quality
- `quality.rs`: fmt, lint, check
- Generic code quality tools
- Works for Rust (cargo fmt/clippy) and JS (prettier/eslint)

#### devkit-ext-test
- `test.rs`: Test running
- `coverage.rs`: Code coverage
- `watch.rs`: Watch mode for tests/builds
- Generic testing infrastructure

#### devkit-ext-ci
- `ci.rs`: GitHub Actions integration
- View runs, logs, trigger workflows
- Re-run failed jobs
- Status checking

#### devkit-ext-git
- `status.rs`: Git status, branch info
- Release workflows (generic parts of `release.rs`)
- Git operations

### Medium Priority

#### devkit-ext-database
- `db.rs`: Generic parts (psql, migrate, reset, seed)
- Leave Shaya-specific cursor migrations in Shaya

#### devkit-ext-tunnel
- `tunnel.rs`: HTTP tunneling
- Useful for exposing localhost

#### devkit-ext-benchmark
- `benchmark.rs`: Performance benchmarking
- Load testing
- Generic enough for most projects

#### devkit-ext-env
- `env.rs`: Generic parts (pull/push env vars)
- ESC integration could be generalized

### Lower Priority

#### devkit-ext-todos
- `todos.rs`: Todo/checklist management
- Could be useful across projects
- Or just recommend external tools

#### devkit-ext-local-ci
- `local.rs`: Run GitHub Actions locally via `act`
- Useful for debugging CI

## 🚫 Stays in Shay (Project-Specific)

### Shay-Specific
- `houston.rs` - Shay's monitoring dashboard
- `migrate.rs` - Cursor-based data migrations (Shay pattern)
- `ecs.rs` - AWS ECS operations (Shay's infrastructure)
- `jobs.rs` - Job queue debugging (Shay's queue)
- `deploy.rs` - Pulumi deployment (Shay's infra)
- `mobile.rs` - GraphQL codegen (Shay's schema)
- `ai_assistant.rs` - AI tasks (Shaya-specific)
- `ai.rs` - AI fix (Shaya-specific)
- `ai_lint/` - AI linting (Shaya-specific)

## 📊 Extraction Priority

### Phase 3: Quality & Testing (Next)
```
extensions/
├── devkit-ext-quality/   # fmt, lint, check
├── devkit-ext-test/      # test, coverage, watch
└── devkit-ext-git/       # status, release basics
```

### Phase 4: CI & Infrastructure
```
extensions/
├── devkit-ext-ci/        # GitHub Actions
├── devkit-ext-database/  # Generic DB operations
└── devkit-ext-tunnel/    # HTTP tunneling
```

### Phase 5: Polish
```
extensions/
├── devkit-ext-benchmark/ # Benchmarking
├── devkit-ext-env/       # Environment variables
└── devkit-ext-todos/     # Todo management
```

## 🎯 Goal: Delete Shay's dev-cli

After extraction, Shay will:
1. **Depend on devkit**: `devkit-core`, `devkit-tasks`, and relevant extensions
2. **Keep Shay-specific**: Houston, ECS, migrations, AI tasks
3. **Custom CLI**: Build custom binary using devkit as library

```rust
// Shay's custom CLI
use devkit_core::AppContext;
use devkit_ext_docker;
use devkit_ext_quality;

fn main() -> Result<()> {
    let ctx = AppContext::new(false)?;

    // Generic commands from extensions
    if ctx.features.docker {
        // Use devkit-ext-docker
    }

    // Shay-specific commands
    if user_selected("houston") {
        houston::start(&ctx)?;
    }

    Ok(())
}
```

## 📈 Progress Tracking

- [x] Core infrastructure (15%)
- [x] Task system (15%)
- [x] Docker extension (10%)
- [ ] Quality & Testing (20%)
- [ ] CI & Infrastructure (15%)
- [ ] Polish extensions (10%)
- [ ] Shay migration (15%)

**Current: 40%**

## 🚀 When Can We Delete Shay's dev-cli?

After Phase 5 (all generic parts extracted):
1. Create Shay's custom CLI using devkit
2. Migrate Shay-specific commands
3. Test thoroughly
4. Delete `dev/cli/` directory in Shay
5. Update Shay's `dev.sh` to use new custom CLI
