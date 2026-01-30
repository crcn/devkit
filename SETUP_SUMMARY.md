# GitHub Actions Setup Summary

This document summarizes the GitHub Actions integration that has been set up for devkit.

## What Was Created

### 1. Setup Action (`.github/actions/setup-devkit/`)

A reusable composite GitHub Action that installs devkit in workflows.

**Files:**
- `action.yml` - Action definition
- `README.md` - Detailed usage documentation

**Features:**
- Multi-platform support (Linux, macOS, Windows)
- Automatic platform detection
- Binary caching for faster runs
- Version management (latest or specific versions)
- Smart GitHub API usage with token support

### 2. CI Workflow (`.github/workflows/ci.yml`)

A comprehensive CI workflow for the devkit repository itself.

**Jobs:**
- `test` - Run Rust tests, clippy, and formatting checks
- `integration` - Build and test devkit CLI
- `example-release-install` - Demonstrate using the setup action

### 3. Example Workflows (`examples/github-actions/`)

Ready-to-use workflow examples for different use cases:

- **basic-ci.yml** - Simple CI with tests and quality checks
- **monorepo-ci.yml** - Parallel testing for monorepos
- **docker-integration.yml** - Integration tests with Docker services
- **cross-platform.yml** - Multi-platform testing matrix
- **README.md** - Examples documentation

### 4. Documentation

- **GITHUB_ACTIONS.md** - Comprehensive guide for using devkit in CI/CD
- **README.md** - Updated with GitHub Actions section
- This **SETUP_SUMMARY.md** file

## How to Use

### For the devkit Repository

The CI workflow is already configured. It will run automatically on:
- Pushes to `main` branch
- Pull requests to `main` branch
- Manual workflow dispatch

### For External Projects

Projects that want to use devkit in their CI/CD can:

#### Option 1: Use the Setup Action

```yaml
- name: Setup devkit
  uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    version: latest
    github-token: ${{ secrets.GITHUB_TOKEN }}

- name: Run commands
  run: devkit cmd test
```

#### Option 2: Copy Example Workflows

1. Choose an example from `examples/github-actions/`
2. Copy to `.github/workflows/` in your project
3. Customize for your needs

#### Option 3: Install Script

```yaml
- name: Install devkit
  run: |
    curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash
    echo "$HOME/.local/bin" >> $GITHUB_PATH
```

## Testing the Setup

### Local Validation

To validate the workflows locally (requires act):

```bash
# Install act (GitHub Actions local runner)
# macOS: brew install act
# Linux: see https://github.com/nektos/act

# Test the CI workflow
act -j test

# Test the integration job
act -j integration
```

### GitHub Validation

The workflows will be automatically validated by GitHub when you:

1. Push to a branch
2. Create a pull request
3. Manually trigger the workflow

### Manual Testing

To test the setup action manually:

1. Create a test workflow in your fork
2. Use the setup action
3. Run simple devkit commands
4. Verify output

Example test workflow:

```yaml
name: Test Setup

on: workflow_dispatch

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./.github/actions/setup-devkit
      - run: |
          devkit --version || true
          devkit doctor || true
```

## Release Process

For devkit releases to work with the setup action:

1. **Build binaries** - The release workflow already does this
2. **Create release** - Happens automatically on push to main
3. **Upload assets** - Binaries are uploaded to the release

The setup action downloads binaries from GitHub releases:
- `https://github.com/crcn/devkit/releases/latest/download/devkit-{platform}`
- `https://github.com/crcn/devkit/releases/download/{version}/devkit-{platform}`

### Release Checklist

When creating a new release:

1. ✅ Ensure version is updated in `Cargo.toml`
2. ✅ Push to main (or create release tag)
3. ✅ Wait for release workflow to complete
4. ✅ Verify binaries are uploaded to release
5. ✅ Test installation with setup action

### Binary Naming Convention

The release workflow creates these binary names:
- `devkit-linux-x86_64`
- `devkit-linux-aarch64`
- `devkit-macos-x86_64`
- `devkit-macos-aarch64`
- `devkit-windows-x86_64.exe`

The setup action expects this naming convention.

## Configuration Requirements

For projects using devkit in CI/CD:

### Minimal Setup

```
your-repo/
├── .dev/
│   └── config.toml    # Required: Global devkit config
└── .github/
    └── workflows/
        └── ci.yml      # Your workflow using devkit
```

### Example .dev/config.toml

```toml
[project]
name = "my-project"

[workspaces]
packages = ["packages/*"]
```

### Example dev.toml (per package)

```toml
[cmd]
test = "cargo test"
build = "cargo build"
lint = "cargo clippy"
```

## Troubleshooting

### Issue: Binary not found

**Solution:** Ensure the setup action completed successfully and PATH was updated:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
- run: echo $PATH
- run: which devkit
```

### Issue: Platform not supported

**Current support:**
- Linux: x86_64, aarch64
- macOS: x86_64, arm64
- Windows: x86_64

**Solution:** Check runner OS and architecture match supported platforms.

### Issue: Download fails

**Possible causes:**
1. No releases exist yet
2. Network/GitHub outage
3. Binary name mismatch

**Solution:** Check release workflow status and verify binary names.

### Issue: Rate limit errors

**Solution:** Provide explicit GitHub token:

```yaml
- uses: crcn/devkit/.github/actions/setup-devkit@main
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

## Next Steps

1. **Test the CI workflow** - Push a commit to trigger it
2. **Create a release** - Let the release workflow run
3. **Test external usage** - Try the setup action from another repo
4. **Update documentation** - Add any project-specific notes
5. **Share examples** - Point users to example workflows

## Resources

- [Setup Action README](./.github/actions/setup-devkit/README.md)
- [GitHub Actions Guide](./GITHUB_ACTIONS.md)
- [Example Workflows](./examples/github-actions/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)

## Maintenance

### Updating the Setup Action

When updating the action:

1. Test changes locally if possible
2. Update version in documentation examples
3. Update CHANGELOG if maintained
4. Consider backward compatibility

### Updating Workflows

When updating workflows:

1. Test syntax with `actionlint` or similar
2. Run on a test branch first
3. Verify all jobs complete successfully
4. Update documentation to match

## Status

✅ Setup action created and documented
✅ CI workflow configured
✅ Example workflows provided
✅ Documentation complete
⏳ Pending: First release with binaries
⏳ Pending: External project testing

## License

MIT OR Apache-2.0
