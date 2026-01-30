# Release Process

## Overview

The devkit release system automatically builds and distributes pre-compiled binaries for multiple platforms.

## Architecture

### 1. GitHub Actions Workflow (`.github/workflows/release.yml`)

**Trigger**: Automatically runs on every push to `main` branch

**Platforms**:
- Linux x86_64
- Linux ARM64 (aarch64)
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

**Process**:
1. Builds release binaries for all platforms in parallel
2. Strips binaries to reduce size (Linux/macOS)
3. Generates SHA256 checksums
4. Creates GitHub release with version tag format: `v{version}-{short-sha}` (e.g., `v0.1.0-abc1234`)
5. Uploads all binaries and checksums as release assets

### 2. Installation System

#### Kitchen Sink Mode (Recommended)

**Binary Location**: `~/.local/bin/devkit` (globally accessible)

**Installation Flow**:
```bash
# User runs install script
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh

# Script does:
# 1. Detects platform/architecture
# 2. Downloads pre-built binary from GitHub releases
# 3. Installs to ~/.local/bin/devkit
# 4. Makes it executable
# 5. Creates dev.sh wrapper
# 6. Creates .dev/config.toml
```

**Usage**:
```bash
# Direct command (recommended)
devkit                  # Interactive menu
devkit start            # Start all services
devkit cmd build        # Run build commands

# Or via wrapper
./dev.sh                # Calls 'devkit' from PATH
```

**Benefits**:
- Single global installation
- No per-project downloads
- No Rust toolchain required
- Automatic updates via re-running install script

#### Custom CLI Mode

**Binary Location**: Built from `dev/cli/` project

**Installation Flow**:
```bash
# Install script creates:
# - dev.sh wrapper (builds custom CLI)
# - dev/cli/ project structure
# - .dev/config.toml

# On first run, dev.sh builds the custom CLI
./dev.sh  # Triggers cargo build
```

**Usage**:
```bash
./dev.sh [command]
```

**Benefits**:
- Full customization
- Add only needed extensions
- Project-specific CLI logic

## Release Workflow

### Automatic Release (Main Branch)

Every push to `main` triggers a release:

```bash
git push origin main
# GitHub Actions automatically:
# - Builds binaries
# - Creates release
# - Uploads assets
```

### Manual Release (Workflow Dispatch)

Trigger manually from GitHub Actions UI if needed.

## Version Management

Version is read from workspace `Cargo.toml`:
```toml
[workspace.package]
version = "0.1.0"
```

To release a new version:
1. Update version in `Cargo.toml`
2. Commit and push to `main`
3. Release is created automatically with tag `v0.1.0-{sha}`

## Binary Naming Convention

Format: `devkit-{os}-{arch}[.exe]`

Examples:
- `devkit-linux-x86_64`
- `devkit-macos-aarch64`
- `devkit-windows-x86_64.exe`

## Download URLs

**Latest release**:
```
https://github.com/crcn/devkit/releases/latest/download/devkit-{platform}
```

**Specific version**:
```
https://github.com/crcn/devkit/releases/download/v0.1.0-abc1234/devkit-{platform}
```

## Caching Strategy

GitHub Actions uses caching to speed up builds:
- Cargo registry cache
- Cargo index cache
- Target directory cache (per platform)

## Fallback Behavior

If binary download fails, the install script provides instructions for building from source:

```bash
git clone https://github.com/crcn/devkit.git
cd devkit
cargo build --release -p devkit-cli
```

## Security

- All binaries include SHA256 checksums in `SHA256SUMS`
- Binaries are stripped (Linux/macOS) to reduce size
- Built in GitHub's secure CI environment
- Release permissions controlled by GITHUB_TOKEN

## Future Enhancements

Potential improvements:
- Code signing for macOS/Windows binaries
- Homebrew tap for easier macOS installation
- APT/YUM repositories for Linux
- Automatic version bump on release
- Changelog generation
