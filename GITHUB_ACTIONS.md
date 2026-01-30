# Using devkit in GitHub Actions

This guide shows how to use devkit in your GitHub Actions workflows for CI/CD automation.

## Quick Start

Add the setup action to your workflow:

```yaml
- name: Setup devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main

- name: Run commands
  run: devkit cmd test
```

## Installation Methods

### Method 1: Use the Setup Action (Recommended)

The easiest way to use devkit in GitHub Actions:

```yaml
jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup devkit
        uses: crcn/devkit/.github/actions/setup-devkit@main
        with:
          version: latest

      - name: Run tests
        run: devkit cmd test
```

**Benefits:**
- ✅ Automatic platform detection
- ✅ Binary caching for faster runs
- ✅ Version management
- ✅ Works on Linux, macOS, and Windows

### Method 2: Install from Release

Use the install script directly:

```yaml
- name: Install devkit
  run: |
    curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash
    echo "$HOME/.local/bin" >> $GITHUB_PATH

- name: Use devkit
  run: devkit cmd test
```

### Method 3: Build from Source

For the latest unreleased changes:

```yaml
- name: Install Rust
  uses: dtolnay/rust-toolchain@stable

- name: Build devkit
  run: |
    git clone https://github.com/crcn/devkit
    cd devkit
    cargo build --release -p devkit-cli
    cp target/release/devkit ~/.local/bin/
    echo "$HOME/.local/bin" >> $GITHUB_PATH

- name: Use devkit
  run: devkit cmd test
```

## Common Workflow Patterns

### Run Tests

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Run all tests
        run: devkit cmd test

      - name: Run specific package tests
        run: devkit cmd test -p api
```

### Code Quality Checks

```yaml
name: Quality

on: [push, pull_request]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Format check
        run: devkit fmt --check

      - name: Lint
        run: devkit lint

      - name: Run tests
        run: devkit cmd test
```

### Multi-Platform Testing

```yaml
name: Cross-Platform

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main
      - run: devkit cmd test
```

### Docker-based Workflows

```yaml
name: Integration

on: [push, pull_request]

jobs:
  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Start services
        run: devkit docker up -d

      - name: Wait for services
        run: sleep 10

      - name: Run integration tests
        run: devkit cmd test:integration

      - name: Show logs on failure
        if: failure()
        run: devkit docker logs
```

### Database Migrations

```yaml
name: Database

on: [push, pull_request]

jobs:
  migrate:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Run migrations
        run: devkit db migrate
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/testdb

      - name: Run seeds
        run: devkit db seed
```

### Parallel Package Commands

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Build all packages in parallel
        run: devkit cmd build --parallel

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: build-output
          path: target/release/
```

### Deployment

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main

      - name: Check system
        run: devkit doctor

      - name: Build for production
        run: devkit cmd build:release

      - name: Deploy to ECS
        run: devkit ecs deploy --environment production
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

## Advanced Usage

### Caching Dependencies

Combine devkit with dependency caching:

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- uses: crcn/devkit/.github/actions/setup-devkit@main

- run: devkit cmd build
```

### Conditional Execution

Run devkit commands based on conditions:

```yaml
- name: Setup devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main

- name: Run integration tests (main branch only)
  if: github.ref == 'refs/heads/main'
  run: devkit cmd test:integration

- name: Deploy (on tag push)
  if: startsWith(github.ref, 'refs/tags/')
  run: devkit ecs deploy
```

### Matrix Testing with Variants

```yaml
jobs:
  test:
    strategy:
      matrix:
        variant: [default, watch, coverage]
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main
      - run: devkit cmd test:${{ matrix.variant }}
```

### Environment-specific Commands

```yaml
- name: Deploy to staging
  run: devkit cmd deploy
  env:
    DEVKIT_ENV: staging
    DATABASE_URL: ${{ secrets.STAGING_DATABASE_URL }}

- name: Deploy to production
  run: devkit cmd deploy
  env:
    DEVKIT_ENV: production
    DATABASE_URL: ${{ secrets.PROD_DATABASE_URL }}
```

## Configuration

### Project Setup

Ensure your repository has the necessary configuration files:

```
your-repo/
├── .dev/
│   └── config.toml          # Global devkit config
├── .github/
│   └── workflows/
│       └── ci.yml            # Your workflow
├── packages/
│   ├── api/
│   │   └── dev.toml          # Package-specific commands
│   └── web/
│       └── dev.toml
└── docker-compose.yml        # Optional
```

### Example .dev/config.toml

```toml
[project]
name = "my-project"

[workspaces]
packages = ["packages/*"]

[environments]
available = ["dev", "staging", "production"]
default = "dev"
```

### Example dev.toml

```toml
[cmd]
test = "cargo test"
lint = "cargo clippy"

[cmd.build]
default = "cargo build"
release = "cargo build --release"
```

## Troubleshooting

### devkit not found

Ensure the setup action adds to PATH:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: echo $PATH
- run: which devkit
```

### Commands fail with "feature not available"

Install required tools in your workflow:

```yaml
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y docker-compose

- uses: crcn/devkit/.github/actions/setup-devkit@main
```

### Docker not available

Use the Docker setup action:

```yaml
- name: Set up Docker Buildx
  uses: docker/setup-buildx-action@v3

- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: devkit docker up
```

### Rate limiting

The setup action uses `${{ github.token }}` by default. For private repos or heavy usage, create a PAT:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    github-token: ${{ secrets.PERSONAL_ACCESS_TOKEN }}
```

## Best Practices

1. **Use the setup action** - It handles caching and platform detection
2. **Pin versions in production** - Use specific version tags instead of `latest`
3. **Cache dependencies** - Combine with cargo/npm/yarn caching
4. **Test on multiple platforms** - Use matrix strategy for cross-platform validation
5. **Use quiet mode for cleaner logs** - Set `DEVKIT_QUIET=1` if needed
6. **Fail fast** - Set `fail-fast: false` in matrix for complete test results

## Resources

- [devkit Documentation](https://github.com/crcn/devkit)
- [Setup Action README](./.github/actions/setup-devkit/README.md)
- [Example Workflows](./.github/workflows/)

## License

MIT OR Apache-2.0
