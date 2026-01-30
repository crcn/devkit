# Quick Reference: setup-devkit Action

## Basic Usage

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: devkit cmd test
```

## Common Patterns

### Specify Version
```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    version: v0.1.0-abc1234
```

### With Token (Avoid Rate Limits)
```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Check Cache Status
```yaml
- id: setup
  uses: crcn/devkit/.github/actions/setup-devkit@main

- run: echo "Cache hit: ${{ steps.setup.outputs.cache-hit }}"
```

## Complete Examples

### Simple CI
```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main
      - run: devkit cmd test
```

### Multi-Platform
```yaml
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
        shell: bash
```

### With Docker
```yaml
jobs:
  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crcn/devkit/.github/actions/setup-devkit@main
      - run: devkit docker up -d
      - run: devkit cmd test:integration
      - if: failure()
        run: devkit docker logs
```

## Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `version` | No | `latest` | Version tag or 'latest' |
| `github-token` | No | `''` | GitHub token for API access |

## Outputs

| Output | Description |
|--------|-------------|
| `version` | Installed devkit version |
| `cache-hit` | Whether binary was cached |

## Supported Platforms

- ✅ Linux x86_64
- ✅ Linux ARM64
- ✅ macOS Intel
- ✅ macOS Apple Silicon
- ✅ Windows x86_64

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `devkit: command not found` | Add `shell: bash` to run step |
| Rate limit error | Add `github-token: ${{ secrets.GITHUB_TOKEN }}` |
| Wrong platform | Check runner OS matches supported platforms |
| Download fails | Verify release exists with binaries |
