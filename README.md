# devkit

**Universal command discovery and execution for any codebase.**

> Run `devkit` in any repository and instantly see every available command - no configuration required.

[![Release](https://img.shields.io/github/v/release/crcn/devkit)](https://github.com/crcn/devkit/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## 🎯 What is devkit?

**The Problem:** You clone a repo. Now what? Where are the commands? How do you run tests? Build? Deploy?

**The Solution:** `devkit` automatically discovers and surfaces **every available command** in any repository:

- **📦 Package managers**: npm scripts, Cargo commands, package.json scripts
- **🔨 Build tools**: Makefile targets, build scripts
- **🐳 Services**: Docker Compose services and operations
- **📝 Scripts**: Executable shell scripts in `bin/`, `scripts/`, etc.
- **✨ And more**: Automatically organized by category with fuzzy search

```bash
$ devkit
╭─────────────────────────────────────────╮
│  🚀 devkit - Command Discovery          │
╰─────────────────────────────────────────╯

? What would you like to do? (type to filter) ›
  🐳 Docker - Up
  🐳 Docker - Logs
  🐳 Docker - Shell

  ─── 📦 Build (3) ───
  📦 Build all packages
  📦 Build (release)
  📦 Build (watch mode)

  ─── 🧪 Test (2) ───
  🧪 Test all packages
  🧪 Test (watch mode)

  ─── ✨ Quality (4) ───
  ✨ Lint all packages
  ✨ Format all packages
  ✨ Check all packages
  ✨ Fix lint issues

  ─── 📝 Scripts (5) ───
  📝 deploy
  📝 setup
  📝 migrate
```

**Zero configuration. Works anywhere. See everything.**

---

## 🚀 Installation

### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash
```

This installs the binary to `~/.local/bin/devkit`.

### Manual Install

Download the latest release for your platform:

```bash
# Detect platform and install
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m | sed 's/arm64/aarch64/')
curl -fsSL "https://github.com/crcn/devkit/releases/latest/download/devkit-${PLATFORM}" -o devkit
chmod +x devkit
sudo mv devkit /usr/local/bin/
```

<details>
<summary>Platform-specific downloads</summary>

```bash
# Linux x86_64
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-linux-x86_64 -o devkit

# Linux ARM64
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-linux-aarch64 -o devkit

# macOS Intel
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-macos-x86_64 -o devkit

# macOS Apple Silicon
curl -fsSL https://github.com/crcn/devkit/releases/latest/download/devkit-macos-aarch64 -o devkit

# Then install
chmod +x devkit && sudo mv devkit /usr/local/bin/
```

Windows: Download `devkit-windows-x86_64.exe` from [releases](https://github.com/crcn/devkit/releases/latest)
</details>

### Build from Source

```bash
git clone https://github.com/crcn/devkit
cd devkit
cargo build --release -p devkit-cli
# Binary at target/release/devkit
```

---

## ⚡ Quick Start

```bash
# Run in any repository
cd your-project
devkit

# Or run specific commands directly
devkit docker up
devkit test
devkit build
```

That's it. No setup, no configuration files required.

---

## 🎨 Key Features

### 🔍 **Zero-Config Discovery**

Devkit automatically finds commands by scanning:

- **Cargo workspaces** → Build, test, clippy, fmt commands per package
- **Node.js projects** → All npm/yarn/pnpm scripts from package.json
- **Docker Compose** → Service operations (up, down, logs, shell, etc.)
- **Makefiles** → All targets with descriptions
- **Shell scripts** → Executable files in bin/, scripts/, tools/

**No `.devkit.toml` required.** Just run it.

### 🎯 **Smart Categorization**

Commands are automatically grouped by purpose:
- 📦 **Build** - Compilation, bundling
- 🧪 **Test** - Test runners
- ✨ **Quality** - Linting, formatting, type checking
- 🐳 **Services** - Docker, databases
- 🚀 **Deploy** - Deployment scripts
- 📝 **Scripts** - Custom project scripts

### 🔎 **Fuzzy Search**

Type to filter commands in real-time:
```
? What would you like to do? › docker
  🐳 Docker - Up
  🐳 Docker - Down
  🐳 Docker - Logs
```

### 📊 **Command History**

Recent commands appear at the top for quick re-runs:
```
─── Recent ───
↻ 🐳 Docker - Logs
↻ 🧪 Test all packages
↻ 📦 Build (watch mode)
```

### 🎭 **Rich Interactive Commands**

Docker operations include:
- **Multi-select containers** for logs
- **Interactive service selection** for up/down/restart
- **Shell access** with container picker
- **Live log following** for multiple containers

---

## 🏗️ Architecture

### Discovery Providers

Auto-discover commands from various sources:

```rust
pub trait CommandProvider {
    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>>;
}
```

**Built-in providers:**
- `CargoProvider` - Rust workspace commands
- `NpmProvider` - Node.js package scripts
- `MakefileProvider` - Make targets
- `ScriptProvider` - Executable scripts
- More coming...

### Extensions

Rich interactive commands for complex workflows:

```rust
pub trait Extension {
    fn is_available(&self, ctx: &AppContext) -> bool;
    fn menu_items(&self) -> Vec<MenuItem>;
}
```

**Built-in extensions:**
- `DockerExtension` - Interactive container operations

### Project Structure

```
devkit/
├── crates/
│   ├── devkit-core/       # Core abstractions, discovery system
│   ├── devkit-tasks/      # Command execution engine
│   └── devkit-cli/        # Main CLI binary
│
└── extensions/
    └── devkit-ext-docker/ # Docker Compose operations
```

---

## 📚 How It Works

1. **Scan**: Devkit scans your repository for commands
   - Checks for Cargo.toml, package.json, Makefile, docker-compose.yml
   - Finds executable scripts in common directories
   - Parses package manager configs

2. **Categorize**: Commands are automatically organized
   - By purpose (build, test, quality, etc.)
   - With emoji indicators
   - Grouped by source

3. **Present**: Interactive fuzzy-searchable menu
   - Type to filter instantly
   - Recent commands at top
   - Clear descriptions

4. **Execute**: Run commands with proper context
   - Correct working directory
   - Proper shell environment
   - Live output streaming

---

## 🎓 Examples

### Basic Discovery

```bash
# See all commands
devkit

# In a Rust workspace
📦 Build all packages
📦 Build (release)
🧪 Test all packages
✨ Lint all packages
✨ Format all packages
✨ Check all packages

# In a Node.js project
📦 build
🧪 test
🧪 test:watch
✨ lint
✨ lint:fix
✨ format
🔥 dev
```

### Docker Operations

```bash
# Interactive Docker menu
devkit

# Select from:
🐳 Docker - Up          # Start services (multi-select)
🐳 Docker - Down        # Stop all services
🐳 Docker - Restart     # Restart services (multi-select)
🐳 Docker - Logs        # Follow logs (multi-select containers)
🐳 Docker - Shell       # Shell into container (select one)
🐳 Docker - Build       # Build images (multi-select)
```

### Workspace Commands

```bash
# Rust workspace
devkit  # Shows commands for each package

📦 Build devkit-core
📦 Build devkit-cli
📦 Build devkit-tasks
🧪 Test devkit-core
🧪 Test devkit-cli

# Node workspace (pnpm/yarn/npm)
📦 build (packages/api)
📦 build (packages/web)
🧪 test (packages/api)
```

---

## 🔧 Use as a Library

### Basic Command Discovery

```rust
use devkit_core::{AppContext, discovery::*};

fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new(false)?;

    // Setup discovery engine
    let mut engine = DiscoveryEngine::new();
    engine.register(Box::new(CargoProvider::new()));
    engine.register(Box::new(NpmProvider::new()));

    // Discover all commands
    let commands = engine.discover(&ctx);

    for cmd in commands {
        println!("{}: {}", cmd.label, cmd.description);
    }

    Ok(())
}
```

### Create Custom Provider

```rust
use devkit_core::discovery::*;

pub struct MyProvider;

impl CommandProvider for MyProvider {
    fn name(&self) -> &'static str {
        "my-provider"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Check if provider should run
        ctx.repo.join("my-config.toml").exists()
    }

    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>> {
        let mut commands = Vec::new();

        commands.push(
            DiscoveredCommand::new("my.cmd", "🚀 My Command", Category::Scripts)
                .description("Does something cool")
                .handler(|ctx| {
                    println!("Running my command!");
                    Ok(())
                })
        );

        Ok(commands)
    }
}
```

### Create Custom Extension

```rust
use devkit_core::{Extension, MenuItem};

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my-extension"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Enable when conditions are met
        true
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![MenuItem {
            label: "🎨 My Interactive Command".to_string(),
            handler: Box::new(|ctx| {
                // Rich interactive workflow here
                Ok(())
            }),
        }]
    }
}
```

---

## 🎯 Design Philosophy

### Discovery Over Configuration

**Bad:** Require users to configure everything
```toml
# .devkit.toml
[commands]
build = "cargo build"
test = "cargo test"
# ... 50 more lines
```

**Good:** Discover automatically
```bash
$ devkit  # Just works, no config needed
```

### Organized by Intent

Commands are grouped by **what they do**, not **where they come from**:
- Build commands together (Cargo, npm, Make)
- Test commands together
- Quality tools together

### Fast and Responsive

- Instant fuzzy search filtering
- Cached discovery results
- Minimal startup time
- No heavy dependencies

---

## 🚦 Status

**Current:** Active development, core features complete

**Supported:**
- ✅ Cargo workspaces
- ✅ Node.js projects (npm/yarn/pnpm)
- ✅ Docker Compose
- ✅ Makefiles
- ✅ Shell scripts
- ✅ Command history
- ✅ Fuzzy search
- ✅ Interactive Docker operations

**Roadmap:**
- [ ] Python projects (Poetry, setup.py)
- [ ] Go modules
- [ ] Justfiles
- [ ] Task runners (Taskfile.yml)
- [ ] VS Code tasks.json
- [ ] Git hooks
- [ ] Multi-select command execution

---

## 📖 Documentation

- **Architecture**: How discovery and extensions work
- **Creating Providers**: Build custom command providers
- **Creating Extensions**: Build rich interactive commands

---

## 🤝 Contributing

Contributions welcome! Focus areas:
- New command providers (Python, Go, etc.)
- Improved command categorization
- Better shell script parsing
- Performance improvements

---

## 📄 License

MIT OR Apache-2.0

---

## 💡 Why devkit?

**The core insight:** Every project has commands. Finding them shouldn't require reading docs.

**Before devkit:**
```bash
$ cd new-project
$ cat README.md  # Scroll... scroll...
$ cat package.json  # Find scripts...
$ cat Makefile  # Find targets...
$ ls bin/  # Find scripts...
```

**With devkit:**
```bash
$ cd new-project
$ devkit
# See everything, organized, searchable
```

**Result:** Zero onboarding friction. Instant productivity.

---

**Made with ❤️ in Rust**
