# Dogfooding devkit

Use devkit to build devkit! This is the best way to validate the design and discover improvements.

## Setup

### 1. Install devkit in the devkit repo

```bash
cd ~/Developer/crcn/devkit

# Run the installer (choose kitchen sink)
./install.sh
# Select: 1 (Kitchen Sink)
```

### 2. Configure workspace

The installer creates `.dev/config.toml`:

```toml
[project]
name = "devkit"

[workspaces]
packages = ["crates/*", "extensions/*"]

[environments]
available = ["dev"]
default = "dev"
```

### 3. Add commands to crates

Create `crates/devkit-core/dev.toml`:

```toml
[cmd.typecheck]
default = "cargo check --all-targets"

[cmd.lint]
default = "cargo clippy --all-targets -- -D warnings"
fix = "cargo clippy --fix --allow-dirty --allow-staged"

[cmd.fmt]
default = "cargo fmt --check"
fix = "cargo fmt"

[cmd.build]
default = "cargo build"
watch = "cargo watch -x check"
release = "cargo build --release"

[cmd]
test = "cargo test"
doc = "cargo doc --no-deps"
```

Create similar `dev.toml` for:
- `crates/devkit-tasks/dev.toml`
- `crates/devkit-cli/dev.toml`
- `extensions/*/dev.toml` (when created)

## Daily Workflow

### Morning: Start Development

```bash
./dev.sh
# Interactive menu appears

# Or directly:
./dev.sh status
```

### Check Everything

```bash
./dev.sh cmd typecheck
```

Output:
```
[typecheck] Running cargo check --all-targets on devkit-core...
    Checking devkit-core v0.1.0
✓ Success

[typecheck] Running cargo check --all-targets on devkit-tasks...
    Checking devkit-tasks v0.1.0
✓ Success

[typecheck] Running cargo check --all-targets on devkit-cli...
    Checking devkit-cli v0.1.0
✓ Success

✓ 3 package(s) succeeded: devkit-core, devkit-tasks, devkit-cli
```

### Run Tests

```bash
./dev.sh cmd test --parallel
```

### Format Code

```bash
./dev.sh cmd fmt:fix
```

### Before Commit

```bash
# Run all checks
./dev.sh cmd typecheck
./dev.sh cmd lint
./dev.sh cmd test
```

Or create a pre-commit command:

```toml
# Add to any dev.toml
[cmd]
precommit = "cargo check && cargo clippy && cargo test"
```

```bash
./dev.sh cmd precommit
```

## Watch Mode for Development

```bash
./dev.sh cmd build:watch
```

Now devkit rebuilds on every change!

## Building Extensions

When you extract an extension:

```bash
# Create the extension
mkdir -p extensions/devkit-ext-docker/src

# Add dev.toml
cat > extensions/devkit-ext-docker/dev.toml <<'EOF'
[cmd.typecheck]
default = "cargo check"

[cmd]
test = "cargo test"
build = "cargo build"
EOF

# Now use it!
./dev.sh cmd build -p devkit-ext-docker
./dev.sh cmd test -p devkit-ext-docker
```

## Documentation

```bash
# Generate docs for all crates
./dev.sh cmd doc

# Open in browser
open target/doc/devkit_core/index.html
```

## CI Integration

In `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install devkit
        run: |
          # Kitchen sink mode
          echo "1" | ./install.sh

      - name: Run checks
        run: |
          ./dev.sh cmd typecheck
          ./dev.sh cmd lint
          ./dev.sh cmd test --parallel

      - name: Build release
        run: ./dev.sh cmd build:release
```

## Benefits

### 1. Immediate Feedback

Every time you use `./dev.sh`, you're testing:
- The installer
- The CLI
- The command system
- The interactive menu
- The auto-detection

### 2. Discover Pain Points

- Commands feel awkward? Fix them.
- Menu confusing? Improve it.
- Missing features? Add them.
- Error messages unclear? Better messages.

### 3. Real-World Testing

Using devkit to build devkit is the ultimate integration test.

### 4. Documentation by Example

The devkit repo becomes a living example of how to use devkit.

## Commands You'll Use Daily

```bash
# Check types across all crates
./dev.sh cmd typecheck

# Run linter
./dev.sh cmd lint

# Fix lint issues
./dev.sh cmd lint:fix

# Format code
./dev.sh cmd fmt:fix

# Run tests
./dev.sh cmd test

# Run tests in parallel
./dev.sh cmd test --parallel

# Watch mode for development
./dev.sh cmd build:watch -p devkit-core

# Build specific crate
./dev.sh cmd build -p devkit-cli

# Build everything in release mode
./dev.sh cmd build:release

# Generate docs
./dev.sh cmd doc

# Show status
./dev.sh status

# Health check
./dev.sh doctor
```

## Advanced: Package Dependencies

Add dependencies between crates:

```toml
# crates/devkit-cli/dev.toml
[cmd.build]
default = "cargo build"
deps = ["devkit-core:build", "devkit-tasks:build"]
```

Now `./dev.sh cmd build -p devkit-cli` automatically builds dependencies first!

## Pro Tips

### 1. Alias for Speed

```bash
# Add to ~/.bashrc or ~/.zshrc
alias dev='./dev.sh'

# Now:
dev cmd test
dev cmd build:watch
```

### 2. Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/bin/bash
./dev.sh cmd fmt:fix
./dev.sh cmd lint
./dev.sh cmd test
```

### 3. Custom Commands

Add project-specific commands:

```toml
# root dev.toml
[cmd]
release = "cargo build --release -p devkit-cli && strip target/release/devkit"
install = "cargo install --path crates/devkit-cli"
clean = "cargo clean"
update = "cargo update"
```

## Measuring Success

After dogfooding for a week, you should have:

- ✅ Fixed several UX issues
- ✅ Added missing features
- ✅ Better error messages
- ✅ Faster workflows
- ✅ More confidence in the design
- ✅ Real-world examples
- ✅ Battle-tested code

## Next Level: Use in Other Projects

Once devkit works well for building devkit, use it in other projects:

1. Your side projects
2. Work projects
3. Open source contributions

Each project will reveal new requirements and improvements.

## The Feedback Loop

```
Use devkit → Find issue → Fix it → Use devkit → Repeat
```

This tight feedback loop ensures devkit stays useful and ergonomic.

## Start Now!

```bash
cd ~/Developer/crcn/devkit

# Install (kitchen sink mode)
./install.sh

# Add dev.toml to crates
# (templates above)

# Start using it!
./dev.sh
```

Within an hour you'll know:
- What works great
- What needs fixing
- What's missing
- Where to focus next

**Dogfooding is the fastest path to a great product!** 🐕
