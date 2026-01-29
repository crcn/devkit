# devkit Installation Guide

## Quick Install

### Option 1: GitHub Raw (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
```

### Option 2: Short URL (if configured)

```bash
curl -fsSL https://devkit.sh/install | sh
```

### Option 3: Inspect First

```bash
# Download and inspect
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh > /tmp/install-devkit.sh
less /tmp/install-devkit.sh

# Run when satisfied
bash /tmp/install-devkit.sh
```

## What Gets Installed

The installer creates these files in your project:

```
your-project/
├── dev.sh                    # Wrapper script (executable)
├── .dev/
│   └── config.toml          # Global configuration
└── dev/
    └── cli/
        ├── Cargo.toml       # Your CLI dependencies
        └── src/
            └── main.rs      # Your CLI implementation
```

### `dev.sh` - The Wrapper

- Auto-installs Rust if not present (via rustup)
- Builds your CLI in release mode
- Caches the binary for fast subsequent runs
- Rebuilds only when source files change

### `dev/cli` - Your Custom CLI

Your project-specific CLI that:
- Starts with sensible defaults (start, stop, status, cmd, doctor)
- Uses devkit libraries for common tasks
- You customize by editing `src/main.rs`
- You extend by adding devkit extensions to `Cargo.toml`

### `.dev/config.toml` - Configuration

Global project config with:
- Project name (auto-detected)
- Workspace patterns for package discovery
- Environment settings (dev, prod)
- Service port mappings
- Git settings

## Post-Install

### 1. Test the Installation

```bash
./dev.sh doctor
```

This checks:
- ✓ git installed
- ✓ cargo installed
- ✓ docker installed (optional)

### 2. Run Interactive Menu

```bash
./dev.sh
```

You'll see:
```
What would you like to do?
> Start development environment
  Stop services
  Run commands (cmd)
  Status
  Doctor
  Exit
```

### 3. Customize Your CLI

Edit `dev/cli/src/main.rs` to add project-specific commands:

```rust
// Add to the Commands enum
#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,

    // Your custom command
    Deploy {
        #[arg(short, long)]
        env: String,
    },
}

// Add handler
match cli.command {
    Some(Commands::Deploy { env }) => {
        ctx.print_header(&format!("Deploying to {}", env));
        // Your deployment logic
        Ok(())
    }
    // ... other handlers
}
```

### 4. Add Extensions

Edit `dev/cli/Cargo.toml`:

```toml
[dependencies]
devkit-core = { git = "https://github.com/crcn/devkit" }
devkit-tasks = { git = "https://github.com/crcn/devkit" }

# Add extensions you need:
devkit-ext-docker = { git = "https://github.com/crcn/devkit" }
devkit-ext-database = { git = "https://github.com/crcn/devkit" }
devkit-ext-quality = { git = "https://github.com/crcn/devkit" }
```

Then rebuild:
```bash
./dev.sh status  # Forces rebuild
```

### 5. Add Package Commands

Create `dev.toml` in your packages:

**For Rust packages** (`packages/api/dev.toml`):
```toml
[cmd.build]
default = "cargo build"
watch = "cargo watch -x build"
release = "cargo build --release"

[cmd]
test = "cargo test"
```

**For Node packages** (`packages/app/dev.toml`):
```toml
[cmd.build]
default = "npx tsc"
watch = "npx tsc --watch"

[cmd]
test = "npx jest"
```

Then run:
```bash
./dev.sh cmd build        # Runs in all packages
./dev.sh cmd build:watch  # Uses watch variant
./dev.sh cmd test         # Runs tests
```

## Advanced Usage

### Custom Installation Directory

```bash
# Install to specific project
cd /path/to/project
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh
```

### Version Pinning

Edit `dev/cli/Cargo.toml` to pin to a specific version:

```toml
[dependencies]
devkit-core = { git = "https://github.com/crcn/devkit", tag = "v0.1.0" }
```

### Updating devkit

```bash
# Update to latest
cd dev/cli
cargo update devkit-core devkit-tasks

# Rebuild
cd ../..
./dev.sh status
```

## Troubleshooting

### "Rust not found"

The installer auto-installs Rust. If it fails:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "Build failed"

Check dependencies:
```bash
./dev.sh doctor
```

Clean and rebuild:
```bash
rm -rf target dev/cli/target
./dev.sh status
```

### "Command not found"

Make sure `dev.sh` is executable:
```bash
chmod +x dev.sh
```

## Uninstalling

Remove these files/directories:
```bash
rm dev.sh
rm -rf .dev
rm -rf dev/cli
rm -rf target  # Build artifacts
```

## Next Steps

- [Configuration Guide](CONFIG.md) - Customize `.dev/config.toml`
- [Package Commands](COMMANDS.md) - Using the command system
- [Extensions](EXTENSIONS.md) - Available extensions
- [Examples](examples/) - Example projects
