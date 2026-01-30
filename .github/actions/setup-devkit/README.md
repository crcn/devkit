# Setup devkit Action

GitHub Action to install the devkit CLI for use in workflows.

## Usage

### Basic

```yaml
- name: Setup devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main
```

### With specific version

```yaml
- name: Setup devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    version: v0.1.0-abc1234
```

### Full example

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup devkit
        uses: crcn/devkit/.github/actions/setup-devkit@main
        with:
          version: latest

      - name: Run devkit commands
        run: |
          devkit doctor
          devkit cmd test
          devkit fmt --check
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `version` | Version of devkit to install (e.g., `v0.1.0-abc1234` or `latest`) | No | `latest` |
| `github-token` | GitHub token for API access (avoids rate limits) | No | `${{ github.token }}` |

## Outputs

| Output | Description |
|--------|-------------|
| `version` | The installed version of devkit |
| `cache-hit` | Whether the devkit binary was restored from cache |

## Features

- ✅ **Multi-platform support**: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows
- ✅ **Smart caching**: Caches binaries by version for faster subsequent runs
- ✅ **Version detection**: Automatically finds latest release when version is `latest`
- ✅ **Rate limit friendly**: Uses GitHub token to avoid API rate limits

## Platform Support

| OS | Architecture | Supported |
|----|--------------|-----------|
| Linux | x86_64 | ✅ |
| Linux | ARM64 | ✅ |
| macOS | x86_64 (Intel) | ✅ |
| macOS | ARM64 (Apple Silicon) | ✅ |
| Windows | x86_64 | ✅ |

## How It Works

1. Detects the runner's platform (OS + architecture)
2. Resolves the requested version (or finds latest release)
3. Checks cache for existing binary
4. Downloads binary from GitHub releases if not cached
5. Installs to `~/.local/bin` and adds to PATH
6. Verifies installation

## Examples

### Run tests with devkit

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: devkit cmd test --parallel
```

### Check code quality

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: |
    devkit fmt --check
    devkit lint
```

### Matrix testing across platforms

```yaml
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main
      - run: devkit cmd test
```

### Use specific version with cache info

```yaml
- name: Setup devkit
  id: devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    version: v0.1.0-abc1234

- name: Check cache status
  run: |
    echo "Version: ${{ steps.devkit.outputs.version }}"
    echo "Cache hit: ${{ steps.devkit.outputs.cache-hit }}"
```

## Troubleshooting

### Binary not found after installation

The action adds `~/.local/bin` to PATH. If you still get "command not found", verify PATH:

```yaml
- run: echo $PATH
- run: which devkit
```

### Rate limit errors

Provide an explicit GitHub token:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Platform not supported

Check the error message for unsupported platforms. Current support:
- Linux: x86_64, aarch64
- macOS: x86_64, arm64
- Windows: x86_64

## License

MIT OR Apache-2.0
