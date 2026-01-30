# devkit Quick Start Guide

Get up and running with devkit in 5 minutes!

---

## Installation

### Option 1: Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash
```

This installs the binary to `~/.local/bin` and sets up your project.

### Option 2: Build from Source

```bash
git clone https://github.com/crcn/devkit
cd devkit
cargo build --release -p devkit-cli
sudo cp target/release/devkit /usr/local/bin/
```

---

## First Steps

### 1. Initialize Your Project

```bash
cd your-project
devkit init
```

This will:
- Auto-detect your tools (Docker, Rust, Node, Python)
- Let you choose features to enable
- Generate `.dev/config.toml`
- Create package `dev.toml` files

### 2. Check Status

```bash
devkit status
```

See what's detected:
- Docker ✓/✗
- Database ✓/✗
- Git ✓/✗
- Languages ✓/✗

### 3. Open Interactive Menu

```bash
devkit
```

Type to filter options! The menu shows all available commands and extensions.

---

## Common Workflows

### Development

```bash
# Start everything
devkit start

# Run tests
devkit cmd test

# Build with watch mode
devkit cmd build --watch

# Run specific package
devkit cmd test -p api
```

### Docker Operations

```bash
# Via menu
devkit  # Select "Docker operations"

# Or directly (if you know the extension)
devkit docker up
devkit docker logs
devkit docker shell
```

### Code Quality

```bash
# Format code
devkit cmd fmt

# Lint
devkit cmd lint

# Fix issues
devkit cmd lint --variant fix
```

---

## Configuration

### Global Config (`.dev/config.toml`)

```toml
[project]
name = "my-app"

[workspaces]
packages = ["packages/*", "apps/*"]

[aliases]
t = "test"
b = "build"
d = "docker"

[features]
docker = true
database = true
quality = true
```

### Package Config (`packages/api/dev.toml`)

```toml
[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
release = "cargo build --release"

[cmd.test]
default = "cargo test"

[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"
```

---

## Quick Tips

### Use Aliases
```bash
# Instead of: devkit cmd test
# Define in config:
[aliases]
t = "test"

# Then use:
devkit cmd t
```

### Use Templates
```toml
[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"
```

```bash
export env=prod
devkit cmd deploy  # Uses k8s/prod.yaml
```

### Watch Mode
```bash
devkit cmd build --watch  # Rebuilds on file changes
```

### Command History
```bash
devkit history           # See all commands
devkit history test      # Search history
```

---

## Extensions Quick Reference

### Cache Management
```bash
devkit  # Menu: "Clean all build caches"
devkit  # Menu: "Show cache statistics"
```

### Secrets Management
```bash
# Requires: op, doppler, or aws CLI
devkit  # Menu: "Pull secrets to .env"
```

### Security Scanning
```bash
devkit  # Menu: "Run security scan"
# Runs: cargo audit, npm audit, gitleaks
```

### Kubernetes
```bash
# Requires: kubectl
devkit  # Menu: "Show cluster status"
devkit  # Menu: "List pods"
```

### Monitoring Stack
```bash
# Requires: docker-compose
devkit  # Menu: "Start monitoring stack"
# Starts: Prometheus, Grafana, Loki, Tempo
```

### Visual Dashboard
```bash
devkit  # Menu: "Open visual dashboard"
# Interactive TUI with service status and logs
```

---

## Troubleshooting

### Command not found
```bash
devkit cmd --list  # See available commands
```

Check your `dev.toml` files:
```toml
[cmd]
test = "cargo test"  # Must be defined
```

### Feature not available
```bash
devkit doctor  # Check what's installed
```

Install missing tools:
- Docker: https://docker.com
- kubectl: https://kubernetes.io/docs/tasks/tools/
- etc.

### Build errors
```bash
cargo clean
devkit cmd build
```

### Nothing happens
Check if you're in the project root (where `.dev/` or `.git/` exists):
```bash
devkit status  # Should show project info
```

---

## Next Steps

### Customize Your Workflow

1. **Add commands** to `dev.toml`:
```toml
[cmd.deploy-staging]
default = "kubectl apply -f k8s/staging.yaml"
```

2. **Create aliases** in `.dev/config.toml`:
```toml
[aliases]
ds = "deploy-staging"
```

3. **Use templates** for flexibility:
```toml
[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"
vars = ["env"]
```

### Add Extensions

If using devkit as a library, add extensions:
```toml
# Cargo.toml
[dependencies]
devkit-core = { git = "https://github.com/crcn/devkit" }
devkit-ext-cache = { git = "https://github.com/crcn/devkit" }
devkit-ext-k8s = { git = "https://github.com/crcn/devkit" }
```

```rust
// main.rs
registry.register(Box::new(devkit_ext_cache::CacheExtension));
registry.register(Box::new(devkit_ext_k8s::K8sExtension));
```

---

## Examples

### Monorepo Setup
```toml
# .dev/config.toml
[workspaces]
packages = ["packages/*", "apps/*"]
exclude = ["packages/legacy"]
```

### Multi-Environment
```toml
[environments]
available = ["dev", "staging", "prod"]
default = "dev"

[cmd.deploy]
default = "kubectl apply -f k8s/{env}.yaml"
```

```bash
export env=staging
devkit cmd deploy
```

### Command Dependencies
```toml
[cmd.integration-test]
default = "cargo test --test integration"
deps = ["api:build", "db:migrate"]  # Runs these first
```

---

## Resources

- **Documentation**: See `README.md`
- **Architecture**: See `ARCHITECTURE.md`
- **Features**: See `ROADMAP.md`
- **Examples**: See `examples/` directory
- **AI Guide**: See `templates/cli/claude.md`

---

## Getting Help

```bash
devkit --help          # CLI help
devkit <command> --help  # Command-specific help
devkit doctor          # System health check
devkit status          # Project status
```

---

## Pro Tips

1. **Fuzzy search in menu** - Just start typing!
2. **History with !!** - Repeat last command (coming soon)
3. **Output formats** - Add `--format json` to commands (coming soon)
4. **Watch + notify** - Get desktop notifications on completion
5. **Remote dev** - SSH and sync files automatically

---

**You're ready to go! Start with `devkit init` and explore the interactive menu.**

Happy building! 🚀
