# GitHub Actions Examples

Example workflows demonstrating how to use devkit in GitHub Actions.

## Examples

### [basic-ci.yml](./basic-ci.yml)
Basic CI workflow with tests and quality checks.

**Use case:** Simple projects that need basic CI

**Features:**
- Code checkout
- devkit installation
- Run tests
- Format and lint checks

### [monorepo-ci.yml](./monorepo-ci.yml)
Monorepo workflow with parallel package testing.

**Use case:** Monorepos with multiple packages

**Features:**
- Parallel test execution
- Separate quality checks job
- Build all packages
- Upload artifacts

### [docker-integration.yml](./docker-integration.yml)
Integration tests with Docker Compose services.

**Use case:** Projects that need database or service containers for testing

**Features:**
- Start Docker Compose services
- Health checks
- Integration tests
- Log collection on failure
- Cleanup

### [cross-platform.yml](./cross-platform.yml)
Test on Linux, macOS, and Windows.

**Use case:** Libraries or tools that need to work cross-platform

**Features:**
- Matrix strategy for multiple OS
- Platform-specific binary installation
- Consistent commands across platforms

## Quick Start

1. Choose the example that matches your use case
2. Copy it to `.github/workflows/` in your repository
3. Customize the workflow for your project
4. Ensure you have the necessary configuration files:
   - `.dev/config.toml` - Global devkit config
   - `dev.toml` files in your packages

## Configuration

### Minimal .dev/config.toml

```toml
[project]
name = "my-project"

[workspaces]
packages = ["packages/*"]
```

### Example dev.toml (in each package)

```toml
[cmd]
test = "cargo test"
build = "cargo build"
lint = "cargo clippy"
fmt = "cargo fmt"
```

## Common Patterns

### Run specific package tests

```yaml
- run: devkit cmd test -p api
```

### Parallel execution

```yaml
- run: devkit cmd build --parallel
```

### With variants

```yaml
- run: devkit cmd build:release
```

### Environment variables

```yaml
- run: devkit cmd deploy
  env:
    DEVKIT_ENV: production
    DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

## Troubleshooting

### devkit not found

Make sure the setup action is run before using devkit:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: devkit cmd test  # This will now work
```

### Commands not available

Check if required tools are installed:

```yaml
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y docker-compose

- uses: crcn/devkit/.github/actions/setup-devkit@main
```

### Platform-specific issues

Use `shell: bash` for cross-platform compatibility:

```yaml
- run: devkit cmd test
  shell: bash
```

## More Information

- [Full GitHub Actions Documentation](../../GITHUB_ACTIONS.md)
- [Setup Action README](../../.github/actions/setup-devkit/README.md)
- [devkit Documentation](../../README.md)
