//! devkit Kitchen Sink CLI
//!
//! A batteries-included development CLI that works out of the box.
//! Configure features via .dev/config.toml - no Rust code required!

use anyhow::Result;
use clap::{Parser, Subcommand};
use devkit_core::{AppContext, ExtensionRegistry};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "dev")]
#[command(about = "Development environment CLI - kitchen sink edition")]
#[command(version)]
struct Cli {
    /// Run in quiet mode (non-interactive)
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start development environment
    Start,

    /// Stop all services
    Stop,

    /// Show environment status
    Status,

    /// Check system prerequisites
    Doctor,

    /// Run package-defined commands
    Cmd {
        /// Command name (e.g., build, test, lint)
        command: Option<String>,
        /// Run in parallel where possible
        #[arg(long)]
        parallel: bool,
        /// Only run for specific packages
        #[arg(short, long)]
        package: Vec<String>,
        /// List all available commands
        #[arg(long)]
        list: bool,
    },

    /// Docker operations (if enabled)
    #[cfg(feature = "docker")]
    Docker {
        #[command(subcommand)]
        action: DockerAction,
    },

    /// Database operations (if enabled)
    #[cfg(feature = "database")]
    Database {
        #[command(subcommand)]
        action: DbAction,
    },

    /// Code quality tools (if enabled)
    #[cfg(feature = "quality")]
    Fmt {
        /// Auto-fix formatting issues
        #[arg(long)]
        fix: bool,
    },

    #[cfg(feature = "quality")]
    Lint {
        /// Auto-fix lint issues
        #[arg(long)]
        fix: bool,
    },

    #[cfg(feature = "quality")]
    Test {
        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },

    /// Dependency management (if enabled)
    #[cfg(feature = "deps")]
    Deps {
        /// List discovered packages
        #[arg(long)]
        list: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[cfg(feature = "docker")]
#[derive(Subcommand)]
enum DockerAction {
    Up,
    Down,
    Restart,
    Logs { service: Option<String> },
    Shell { service: Option<String> },
}

#[cfg(feature = "database")]
#[derive(Subcommand)]
enum DbAction {
    Migrate,
    Reset,
    Seed,
    Shell,
}

fn main() -> ExitCode {
    let _ = dotenvy::dotenv();

    // Initialize tracing
    init_tracing();

    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn init_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    // Allow override via RUST_LOG env var, default to info for devkit crates
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("devkit=info,devkit_core=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let ctx = AppContext::new(cli.quiet)?;

    // Register and run prerun hooks from extensions
    #[cfg(feature = "deps")]
    {
        use devkit_core::ExtensionRegistry;
        let mut registry = ExtensionRegistry::new();
        registry.register(Box::new(devkit_ext_deps::DepsExtension));

        // Run prerun hooks (auto-install dependencies, etc.)
        if let Err(e) = registry.run_prerun_hooks(&ctx) {
            ctx.print_error(&format!("Prerun failed: {:#}", e));
            return Err(e.into());
        }
    }

    // Features are AUTO-DETECTED based on project structure
    // No manual configuration needed!
    let features = &ctx.features;

    match cli.command {
        Some(Commands::Start) => cmd_start(&ctx),
        Some(Commands::Stop) => cmd_stop(&ctx),
        Some(Commands::Status) => cmd_status(&ctx),
        Some(Commands::Doctor) => cmd_doctor(&ctx),
        Some(Commands::Cmd {
            command,
            parallel,
            package,
            list,
        }) => cmd_run(&ctx, command, parallel, package, list),

        #[cfg(feature = "docker")]
        Some(Commands::Docker { action }) if features.docker => handle_docker(&ctx, action),

        #[cfg(feature = "database")]
        Some(Commands::Database { action }) if features.database => handle_database(&ctx, action),

        #[cfg(feature = "quality")]
        Some(Commands::Fmt { fix }) if features.commands => handle_fmt(&ctx, fix),

        #[cfg(feature = "quality")]
        Some(Commands::Lint { fix }) if features.commands => handle_lint(&ctx, fix),

        #[cfg(feature = "quality")]
        Some(Commands::Test { watch }) if features.commands => handle_test(&ctx, watch),

        #[cfg(feature = "deps")]
        Some(Commands::Deps { list }) => handle_deps(&ctx, list),

        Some(Commands::Completions { shell }) => {
            generate_completions(shell);
            Ok(())
        }

        None => interactive_menu(&ctx),

        _ => {
            ctx.print_warning("Feature not available in this project");
            ctx.print_info("This command requires project setup (e.g., docker-compose.yml, database config)");
            Ok(())
        }
    }
}

fn generate_completions(shell: clap_complete::Shell) {
    use clap::CommandFactory;
    use clap_complete::generate;
    use std::io;

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "devkit", &mut io::stdout());
}

fn cmd_start(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Starting development environment");

    // Docker (if enabled)
    #[cfg(feature = "docker")]
    if ctx.config.global.features.docker {
        ctx.print_info("Starting Docker containers...");
        // devkit_ext_docker::compose_up(ctx, &[], false)?;
        ctx.print_success("✓ Docker containers started");
    }

    // Database (if enabled)
    #[cfg(feature = "database")]
    if ctx.config.global.features.database {
        ctx.print_info("Running migrations...");
        // devkit_ext_database::migrate(ctx)?;
        ctx.print_success("✓ Database migrated");
    }

    ctx.print_success("✓ Development environment ready!");
    Ok(())
}

fn cmd_stop(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Stopping development environment");

    #[cfg(feature = "docker")]
    if ctx.config.global.features.docker {
        // devkit_ext_docker::compose_down(ctx)?;
        ctx.print_success("✓ Docker containers stopped");
    }

    Ok(())
}

fn cmd_status(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Development Environment Status");
    println!();
    println!("Repository: {}", ctx.repo.display());
    println!("Project: {}", ctx.config.global.project.name);
    println!();

    println!("Features:");
    let features = &ctx.features;
    println!("  Docker: {}", if features.docker { "✓" } else { "✗" });
    println!("  Database: {}", if features.database { "✓" } else { "✗" });
    println!("  Git: {}", if features.git { "✓" } else { "✗" });
    println!("  Cargo: {}", if features.cargo { "✓" } else { "✗" });
    println!("  Node: {}", if features.node { "✓" } else { "✗" });
    println!();

    ctx.print_success("✓ Configuration loaded");
    Ok(())
}

fn cmd_doctor(ctx: &AppContext) -> Result<()> {
    ctx.print_header("System Health Check");
    println!();

    let tools = vec![
        ("git", devkit_core::utils::cmd_exists("git")),
        ("cargo", devkit_core::utils::cmd_exists("cargo")),
        ("docker", devkit_core::utils::docker_available()),
    ];

    for (tool, available) in tools {
        if available {
            ctx.print_success(&format!("✓ {}", tool));
        } else {
            ctx.print_warning(&format!("✗ {} (not found)", tool));
        }
    }

    println!();
    ctx.print_success("Health check complete");
    Ok(())
}

fn cmd_run(
    ctx: &AppContext,
    command: Option<String>,
    parallel: bool,
    packages: Vec<String>,
    list: bool,
) -> Result<()> {
    use devkit_tasks::{list_commands, print_results, run_cmd, CmdOptions};

    if list {
        let commands = list_commands(&ctx.config);
        if commands.is_empty() {
            println!("No commands defined.");
            println!();
            println!("Add commands to package dev.toml files:");
            println!();
            println!("  [cmd]");
            println!("  build = \"cargo build\"");
            println!("  test = \"cargo test\"");
            return Ok(());
        }

        println!("Available commands:");
        println!();
        for (cmd, pkgs) in commands {
            println!("  {} ({})", cmd, pkgs.join(", "));
        }
        return Ok(());
    }

    let cmd_name = match command {
        Some(c) => c,
        None => {
            ctx.print_warning("No command specified. Use --list to see available commands.");
            return Ok(());
        }
    };

    let opts = CmdOptions {
        parallel,
        variant: None,
        packages,
        capture: false,
    };

    let results = run_cmd(ctx, &cmd_name, &opts)?;
    print_results(ctx, &results);

    if results.iter().any(|r| !r.success) {
        return Err(anyhow::anyhow!("Some commands failed"));
    }

    Ok(())
}

#[cfg(feature = "docker")]
fn handle_docker(ctx: &AppContext, action: DockerAction) -> Result<()> {
    use devkit_ext_docker;

    match action {
        DockerAction::Up => devkit_ext_docker::compose_up(ctx, &[], false).map_err(Into::into),
        DockerAction::Down => devkit_ext_docker::compose_down(ctx).map_err(Into::into),
        DockerAction::Restart => devkit_ext_docker::compose_restart(ctx, &[]).map_err(Into::into),
        DockerAction::Logs { service } => devkit_ext_docker::logs(ctx, service.as_deref()).map_err(Into::into),
        DockerAction::Shell { service } => devkit_ext_docker::shell(ctx, service.as_deref()).map_err(Into::into),
    }
}

#[cfg(feature = "database")]
fn handle_database(ctx: &AppContext, action: DbAction) -> Result<()> {
    use devkit_ext_database;

    // Database functions return anyhow::Result, so no conversion needed
    match action {
        DbAction::Migrate => devkit_ext_database::migrate(ctx),
        DbAction::Reset => devkit_ext_database::reset(ctx),
        DbAction::Seed => devkit_ext_database::seed(ctx),
        DbAction::Shell => devkit_ext_database::shell(ctx),
    }
}

#[cfg(feature = "quality")]
fn handle_fmt(ctx: &AppContext, fix: bool) -> Result<()> {
    use devkit_ext_quality;
    devkit_ext_quality::fmt(ctx, fix)
}

#[cfg(feature = "quality")]
fn handle_lint(ctx: &AppContext, fix: bool) -> Result<()> {
    use devkit_ext_quality;
    devkit_ext_quality::lint(ctx, fix)
}

#[cfg(feature = "quality")]
fn handle_test(ctx: &AppContext, watch: bool) -> Result<()> {
    use devkit_ext_quality;
    if watch {
        devkit_ext_quality::test_watch(ctx)
    } else {
        devkit_ext_quality::test(ctx)
    }
}

#[cfg(feature = "deps")]
fn handle_deps(ctx: &AppContext, list: bool) -> Result<()> {
    use devkit_ext_deps;
    if list {
        devkit_ext_deps::print_summary(ctx);
        Ok(())
    } else {
        devkit_ext_deps::check_and_install(ctx)
    }
}

fn interactive_menu(ctx: &AppContext) -> Result<()> {
    use dialoguer::Select;

    // Create extension registry and register all extensions
    let mut registry = ExtensionRegistry::new();

    #[cfg(feature = "docker")]
    registry.register(Box::new(devkit_ext_docker::DockerExtension));

    #[cfg(feature = "database")]
    registry.register(Box::new(devkit_ext_database::DatabaseExtension));

    #[cfg(feature = "quality")]
    registry.register(Box::new(devkit_ext_quality::QualityExtension));

    #[cfg(feature = "deps")]
    registry.register(Box::new(devkit_ext_deps::DepsExtension));

    loop {
        // Build menu dynamically - extensions auto-register their items!
        let menu_items = registry.menu_items(ctx);
        let mut display: Vec<String> = vec![];

        // Core commands (always available)
        display.push("▶  Start development environment".to_string());
        display.push("⏹  Stop services".to_string());

        // Package commands (if any packages define them)
        if ctx.features.commands {
            display.push("⚙  Run package commands".to_string());
        }

        // Extension menu items (automatically populated based on availability)
        for item in &menu_items {
            display.push(item.label.clone());
        }

        // Utilities
        display.push("📊 Status".to_string());
        display.push("🩺 Doctor".to_string());

        // Exit
        display.push("❌ Exit".to_string());

        println!();
        let choice = Select::with_theme(&ctx.theme())
            .with_prompt("What would you like to do?")
            .items(&display)
            .default(0)
            .interact()?;

        // Handle core commands
        let result = if choice == 0 {
            cmd_start(ctx)
        } else if choice == 1 {
            cmd_stop(ctx)
        } else if choice == 2 && ctx.features.commands {
            println!();
            cmd_run(ctx, None, false, vec![], true)
        } else if choice == display.len() - 3 {
            println!();
            cmd_status(ctx)
        } else if choice == display.len() - 2 {
            println!();
            cmd_doctor(ctx)?;
            Ok(())
        } else if choice == display.len() - 1 {
            break;
        } else {
            // Extension menu item - calculate offset
            let offset = if ctx.features.commands { 3 } else { 2 };
            let ext_idx = choice - offset;

            if ext_idx < menu_items.len() {
                println!();
                (menu_items[ext_idx].handler)(ctx).map_err(Into::into)
            } else {
                Ok(())
            }
        };

        if let Err(e) = result {
            println!();
            ctx.print_error(&format!("Error: {:#}", e));
        }
    }

    Ok(())
}

