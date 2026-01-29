# Extraction Plan

This document tracks what's been extracted from Shaya dev-cli and what remains.

## ✅ Completed

### devkit-core
- [x] `context.rs` - AppContext with quiet mode, theming, and output helpers
- [x] `config.rs` - Configuration system (`.dev/config.toml` + `dev.toml`)
- [x] `utils.rs` - Repo root detection, command existence checks, browser opening
- [x] `detection.rs` - Feature detection (docker, database, git, etc.)

## 🚧 In Progress

### devkit-compose
- [ ] Extract `compose.rs` - Docker compose operations
- [ ] Extract `cmd/docker.rs` - Docker container management

### devkit-tasks
- [ ] Extract `cmd/cmd.rs` - Package command discovery and execution
- [ ] Extract `cmd_builder.rs` - Command building utilities

## 📋 TODO

### Generic Features (Extract to devkit)

#### Core Operations
- [ ] `cmd/quality.rs` - Fmt, lint, check
- [ ] `cmd/test.rs` - Test running
- [ ] `cmd/coverage.rs` - Code coverage
- [ ] `cmd/watch.rs` - Watch mode
- [ ] `cmd/status.rs` - Status display
- [ ] `cmd/release.rs` - Release workflows
- [ ] `cmd/ci.rs` - CI/CD operations (GitHub Actions)
- [ ] `cmd/env.rs` - Environment variable management
- [ ] `cmd/tunnel.rs` - HTTP tunneling
- [ ] `cmd/db.rs` - Database operations (generic parts)

#### UI Components
- [ ] `menu.rs` - Menu system
- [ ] `interactive/` - Interactive mode
- [ ] `history.rs` - Command history
- [ ] `services.rs` - Service management

### Project-Specific (Stay in Shaya)

These features are Shaya-specific and should NOT be extracted:

- `cmd/houston.rs` - Shaya's monitoring dashboard
- `cmd/migrate.rs` - Cursor-based data migrations
- `cmd/ecs.rs` - AWS ECS operations (though could be made generic with config)
- `cmd/jobs.rs` - Job queue debugging
- `cmd/deploy.rs` - Pulumi deployment (could be generic)
- `cmd/mobile.rs` - GraphQL codegen
- `cmd/ai_assistant.rs` - AI tasks
- `cmd/ai.rs` - AI fix
- `cmd/ai_lint/` - AI linting
- `cmd/todos.rs` - Todo management
- `cmd/local.rs` - Local CI via act
- `cmd/benchmark.rs` - Benchmarking

## Design Decisions

### Feature Detection
Commands are automatically hidden if not detected in the project:
- No `docker-compose.yml`? No docker commands
- No `[database]` sections? No database commands
- No `.github/workflows`? No CI commands

This is implemented in `devkit-core/src/detection.rs`.

### Configuration Philosophy
- Defaults work out of the box (no config required)
- Convention over configuration (discover packages automatically)
- Explicit overrides via `.dev/config.toml` and `dev.toml`

### Package Commands
The `[cmd]` system in `dev.toml` is the extension point:
```toml
[cmd]
test = "npm test"

[cmd.build]
default = "npm run build"
watch = "npm run build:watch"
deps = ["utils:build"]  # Run utils:build first
```

This allows projects to add custom commands without modifying devkit code.

## Migration Path

1. **Phase 1: Core** ✅
   - Extract config, context, utils, detection
   - Create workspace structure

2. **Phase 2: Tasks** (Current)
   - Extract command discovery/execution system
   - Extract docker compose operations

3. **Phase 3: Commands**
   - Extract generic command implementations
   - Create example custom commands

4. **Phase 4: Integration**
   - Update Shaya to consume devkit as library
   - Test across other projects
   - Polish documentation

## Testing Strategy

- Unit tests for each extracted module
- Integration tests with sample projects
- Test across multiple project types:
  - Rust monorepo (like Shaya)
  - Node monorepo
  - Mixed Rust + Node
  - Single package projects
