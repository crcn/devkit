# Building a Custom CLI with devkit

This guide shows how to build a custom CLI using devkit as a library, combining generic extensions with your project-specific commands.

## Project Structure

```
your-project/
├── Cargo.toml
├── dev/
│   └── cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs              # CLI entry point
│           ├── commands/
│           │   ├── mod.rs
│           │   ├── houston.rs       # Your custom command
│           │   └── migrate.rs       # Your custom command
│           └── menu.rs              # Interactive menu
├── .dev/
│   └── config.toml                  # devkit config
└── dev.sh                           # Wrapper script
```

## Step 1: Add Dependencies

**`dev/cli/Cargo.toml`:**
```toml
[package]
name = "shay-dev"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "dev"
path = "src/main.rs"

[dependencies]
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
console = "0.15"
dialoguer = { version = "0.11", features = ["fuzzy-select"] }

# devkit core
devkit-core = "0.1"
devkit-tasks = "0.1"

# devkit extensions (pick what you need)
devkit-ext-docker = "0.1"
devkit-ext-quality = "0.1"
devkit-ext-test = "0.1"
devkit-ext-ci = "0.1"
```

## Step 2: Create CLI Structure

**`dev/cli/src/main.rs`:**
```rust
mod commands;
mod menu;

use anyhow::Result;
use clap::{Parser, Subcommand};
use devkit_core::AppContext;

#[derive(Parser)]
#[command(name = "dev")]
#[command(about = "Shay development CLI")]
struct Cli {
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // =========================================================================
    // Generic Commands (from devkit extensions)
    // =========================================================================

    /// Start development environment
    Start,

    /// Stop all services
    Stop,

    /// Docker operations
    #[command(alias = "d")]
    Docker {
        #[command(subcommand)]
        action: DockerAction,
    },

    /// Run tests
    #[command(alias = "t")]
    Test {
        #[arg(short, long)]
        package: Option<String>,
        #[arg(short, long)]
        watch: bool,
    },

    /// Code quality
    Fmt { #[arg(long)] fix: bool },
    Lint { #[arg(long)] fix: bool },
    Check,

    /// Run package commands from dev.toml
    Cmd {
        command: String,
        #[arg(long)]
        parallel: bool,
        #[arg(short, long)]
        package: Vec<String>,
    },

    // =========================================================================
    // Project-Specific Commands (your custom ones)
    // =========================================================================

    /// Start Houston monitoring dashboard
    Houston,

    /// Data migration management
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },

    /// ECS operations
    Ecs {
        #[command(subcommand)]
        action: EcsAction,
    },
}

#[derive(Subcommand)]
enum DockerAction {
    Up { services: Vec<String> },
    Down,
    Logs { service: Option<String> },
    Shell { service: Option<String> },
}

#[derive(Subcommand)]
enum MigrateAction {
    List,
    Run { name: String },
    Status { name: String },
}

#[derive(Subcommand)]
enum EcsAction {
    Exec,
    Health { env: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let ctx = AppContext::new(cli.quiet)?;

    match cli.command {
        Some(cmd) => run_command(&ctx, cmd),
        None => menu::run_interactive(&ctx),
    }
}

fn run_command(ctx: &AppContext, cmd: Commands) -> Result<()> {
    match cmd {
        // Generic commands using extensions
        Commands::Start => {
            ctx.print_header("Starting development environment");

            // Start docker if available
            if ctx.features.docker {
                devkit_ext_docker::compose_up(ctx, &[], false)?;
            }

            ctx.print_success("Development environment started!");
            Ok(())
        }

        Commands::Stop => {
            if ctx.features.docker {
                devkit_ext_docker::compose_down(ctx)?;
            }
            Ok(())
        }

        Commands::Docker { action } => match action {
            DockerAction::Up { services } => {
                devkit_ext_docker::compose_up(ctx, &services, false)
            }
            DockerAction::Down => {
                devkit_ext_docker::compose_down(ctx)
            }
            DockerAction::Logs { service } => {
                let containers = devkit_ext_docker::list_running_containers(ctx)?;
                let container = service
                    .and_then(|s| containers.iter().find(|c| c.label.contains(&s)))
                    .or_else(|| containers.first())
                    .ok_or_else(|| anyhow::anyhow!("No containers running"))?;

                devkit_ext_docker::follow_logs(ctx, &container.id)
            }
            DockerAction::Shell { service } => {
                let containers = devkit_ext_docker::list_running_containers(ctx)?;
                let container = service
                    .and_then(|s| containers.iter().find(|c| c.label.contains(&s)))
                    .or_else(|| containers.first())
                    .ok_or_else(|| anyhow::anyhow!("No containers running"))?;

                devkit_ext_docker::open_shell(ctx, &container.id)
            }
        },

        Commands::Test { package, watch } => {
            use devkit_ext_test::{run_tests, watch_tests};
            if watch {
                watch_tests(ctx, package.as_deref())
            } else {
                run_tests(ctx, package.as_deref())
            }
        }

        Commands::Fmt { fix } => {
            devkit_ext_quality::run_fmt(ctx, fix)
        }

        Commands::Lint { fix } => {
            devkit_ext_quality::run_lint(ctx, fix)
        }

        Commands::Check => {
            devkit_ext_quality::run_check(ctx)
        }

        Commands::Cmd { command, parallel, package } => {
            use devkit_tasks::{run_cmd, CmdOptions};

            let opts = CmdOptions {
                parallel,
                variant: None,
                packages: package,
                capture: false,
            };

            let results = run_cmd(ctx, &command, &opts)?;
            devkit_tasks::print_results(ctx, &results);

            if results.iter().any(|r| !r.success) {
                return Err(anyhow::anyhow!("Some commands failed"));
            }
            Ok(())
        }

        // Project-specific commands
        Commands::Houston => {
            commands::houston::start(ctx)
        }

        Commands::Migrate { action } => {
            commands::migrate::run(ctx, action)
        }

        Commands::Ecs { action } => {
            commands::ecs::run(ctx, action)
        }
    }
}
```

## Step 3: Interactive Menu

**`dev/cli/src/menu.rs`:**
```rust
use anyhow::Result;
use dialoguer::Select;
use devkit_core::AppContext;

pub fn run_interactive(ctx: &AppContext) -> Result<()> {
    loop {
        let mut items = vec![];

        // Generic commands (auto-show based on features)
        items.push("Start development environment");

        if ctx.features.docker {
            items.push("Docker operations");
        }

        if ctx.features.commands {
            items.push("Run package command");
        }

        items.push("Code quality (fmt/lint/check)");

        if ctx.features.git {
            items.push("Git status");
        }

        // Project-specific commands
        items.push("─────────────────────"); // Separator
        items.push("Houston (monitoring)");
        items.push("Data migrations");
        items.push("ECS operations");

        items.push("─────────────────────");
        items.push("Exit");

        let choice = Select::with_theme(&ctx.theme())
            .with_prompt("Shay Development CLI")
            .items(&items)
            .default(0)
            .interact()?;

        match items[choice] {
            "Start development environment" => {
                crate::run_command(ctx, crate::Commands::Start)?;
            }
            "Docker operations" => {
                docker_menu(ctx)?;
            }
            "Code quality (fmt/lint/check)" => {
                quality_menu(ctx)?;
            }
            "Houston (monitoring)" => {
                crate::commands::houston::start(ctx)?;
            }
            "Exit" => break,
            _ => {}
        }
    }

    Ok(())
}

fn docker_menu(ctx: &AppContext) -> Result<()> {
    let items = vec![
        "Start containers (up)",
        "Stop containers (down)",
        "View logs",
        "Open shell",
        "Back",
    ];

    let choice = Select::with_theme(&ctx.theme())
        .with_prompt("Docker Operations")
        .items(&items)
        .interact()?;

    match choice {
        0 => devkit_ext_docker::compose_up(ctx, &[], false)?,
        1 => devkit_ext_docker::compose_down(ctx)?,
        2 => {
            let containers = devkit_ext_docker::list_running_containers(ctx)?;
            if let Some(c) = containers.first() {
                devkit_ext_docker::follow_logs(ctx, &c.id)?;
            }
        }
        3 => {
            let containers = devkit_ext_docker::list_running_containers(ctx)?;
            if let Some(c) = containers.first() {
                devkit_ext_docker::open_shell(ctx, &c.id)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn quality_menu(ctx: &AppContext) -> Result<()> {
    let items = vec![
        "Format code (fmt)",
        "Lint code",
        "Run all checks",
        "Back",
    ];

    let choice = Select::with_theme(&ctx.theme())
        .with_prompt("Code Quality")
        .items(&items)
        .interact()?;

    match choice {
        0 => devkit_ext_quality::run_fmt(ctx, true)?,
        1 => devkit_ext_quality::run_lint(ctx, true)?,
        2 => devkit_ext_quality::run_check(ctx)?,
        _ => {}
    }

    Ok(())
}
```

## Step 4: Custom Commands

**`dev/cli/src/commands/houston.rs`:**
```rust
use anyhow::Result;
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

pub fn start(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Starting Houston");

    let houston_dir = ctx.repo.join("dev/houston-server");

    CmdBuilder::new("cargo")
        .args(["run", "--release"])
        .cwd(houston_dir)
        .inherit_io()
        .run()?;

    Ok(())
}
```

**`dev/cli/src/commands/migrate.rs`:**
```rust
use anyhow::Result;
use devkit_core::AppContext;

pub fn run(ctx: &AppContext, action: crate::MigrateAction) -> Result<()> {
    match action {
        crate::MigrateAction::List => {
            ctx.print_header("Available migrations");
            // Your implementation
            Ok(())
        }
        crate::MigrateAction::Run { name } => {
            ctx.print_header(&format!("Running migration: {}", name));
            // Your implementation
            Ok(())
        }
        crate::MigrateAction::Status { name } => {
            ctx.print_header(&format!("Migration status: {}", name));
            // Your implementation
            Ok(())
        }
    }
}
```

**`dev/cli/src/commands/mod.rs`:**
```rust
pub mod houston;
pub mod migrate;
pub mod ecs;
```

## Step 5: Build Script

**`dev.sh`:**
```bash
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export REPO_ROOT="$SCRIPT_DIR"

# Build the custom CLI
DEV_CLI_BIN="$REPO_ROOT/target/release/dev"
DEV_CLI_DIR="$REPO_ROOT/dev/cli"

needs_rebuild() {
  [[ ! -f "$DEV_CLI_BIN" ]] && return 0
  find "$DEV_CLI_DIR/src" -name '*.rs' -newer "$DEV_CLI_BIN" 2>/dev/null | grep -q . && return 0
  [[ "$DEV_CLI_DIR/Cargo.toml" -nt "$DEV_CLI_BIN" ]] && return 0
  return 1
}

if needs_rebuild; then
  echo "Building custom dev CLI..."
  cargo build --release --manifest-path "$DEV_CLI_DIR/Cargo.toml" --quiet
fi

exec "$DEV_CLI_BIN" "$@"
```

## Usage

```bash
# Interactive menu
./dev.sh

# Direct commands
./dev.sh start
./dev.sh docker up
./dev.sh test --watch
./dev.sh houston
./dev.sh migrate list

# Package commands from dev.toml
./dev.sh cmd build
./dev.sh cmd test --parallel
```

## Key Benefits

1. **Mix & Match**: Use devkit extensions + your custom commands
2. **Feature Detection**: Commands auto-hide if features not detected
3. **Type Safety**: Full Rust type checking for all commands
4. **Reusable**: Share devkit extensions across all projects
5. **Project-Specific**: Keep your unique commands in your repo

## Migration Path for Shay

1. Create `dev/cli/` structure above
2. Copy Houston, migrations, ECS commands from old dev-cli
3. Use devkit extensions for generic stuff
4. Test thoroughly
5. Delete old `dev/cli/` directory
6. Ship it! 🚀
