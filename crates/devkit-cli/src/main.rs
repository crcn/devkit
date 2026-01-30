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

    /// Check for updates
    Update {
        /// Force update check (ignore cache)
        #[arg(long)]
        force: bool,
    },

    /// Initialize a new devkit project
    Init {
        /// Skip interactive prompts
        #[arg(long)]
        no_interactive: bool,
    },

    /// View command history
    History {
        /// Search pattern
        search: Option<String>,
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
    let mut cli = Cli::parse();
    let ctx = AppContext::new(cli.quiet)?;

    // Resolve command aliases
    resolve_aliases(&mut cli, &ctx);

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

        #[cfg(feature = "deps")]
        Some(Commands::Deps { list }) => handle_deps(&ctx, list),

        Some(Commands::Completions { shell }) => {
            generate_completions(shell);
            Ok(())
        }

        Some(Commands::Update { force }) => cmd_update(&ctx, force),

        Some(Commands::Init { no_interactive }) => {
            devkit_core::init::init_project(&ctx.repo, !no_interactive).map_err(Into::into)
        }

        Some(Commands::History { search }) => cmd_history(&ctx, search.as_deref()),

        None => {
            // Check for updates in background (non-blocking)
            check_for_updates_background(&ctx);
            interactive_menu(&ctx)
        }

        _ => {
            ctx.print_warning("Feature not available in this project");
            ctx.print_info(
                "This command requires project setup (e.g., docker-compose.yml, database config)",
            );
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
        DockerAction::Logs { service } => {
            devkit_ext_docker::logs(ctx, service.as_deref()).map_err(Into::into)
        }
        DockerAction::Shell { service } => {
            devkit_ext_docker::shell(ctx, service.as_deref()).map_err(Into::into)
        }
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
    use dialoguer::FuzzySelect;
    use std::collections::{HashMap, HashSet};

    // Create extension registry and register all extensions
    let mut registry = ExtensionRegistry::new();

    #[cfg(feature = "docker")]
    registry.register(Box::new(devkit_ext_docker::DockerExtension));

    #[cfg(feature = "database")]
    registry.register(Box::new(devkit_ext_database::DatabaseExtension));

    #[cfg(feature = "deps")]
    registry.register(Box::new(devkit_ext_deps::DepsExtension));

    #[cfg(feature = "git")]
    registry.register(Box::new(devkit_ext_git::GitExtension));

    #[cfg(feature = "ecs")]
    registry.register(Box::new(devkit_ext_ecs::EcsExtension));

    #[cfg(feature = "pulumi")]
    registry.register(Box::new(devkit_ext_pulumi::PulumiExtension));

    #[cfg(feature = "ci")]
    registry.register(Box::new(devkit_ext_ci::CiExtension));

    #[cfg(feature = "commands")]
    registry.register(Box::new(devkit_ext_commands::CommandsExtension));

    // Start with all groups expanded for better discoverability and filtering
    let menu_items_initial = registry.menu_items(ctx);
    let mut expanded_groups: HashSet<String> = HashSet::new();

    // Auto-expand all groups initially
    for item in &menu_items_initial {
        if let Some(group) = &item.group {
            expanded_groups.insert(group.clone());
        }
    }

    loop {
        // Build menu dynamically
        let menu_items = registry.menu_items(ctx);

        // Group items by their group field
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        let mut ungrouped: Vec<usize> = vec![];

        for (idx, item) in menu_items.iter().enumerate() {
            if let Some(group) = &item.group {
                groups.entry(group.clone()).or_default().push(idx);
            } else {
                ungrouped.push(idx);
            }
        }

        // Build display list
        #[derive(Clone)]
        enum DisplayItem {
            GroupHeader(String),
            Item(usize),
            Exit,
        }

        let mut display: Vec<String> = vec![];
        let mut display_mapping: Vec<DisplayItem> = vec![];

        // Add ungrouped items first
        for &idx in &ungrouped {
            display.push(menu_items[idx].label.clone());
            display_mapping.push(DisplayItem::Item(idx));
        }

        // Add groups (sorted for consistent ordering)
        let mut group_names: Vec<_> = groups.keys().cloned().collect();
        group_names.sort();

        for group_name in group_names {
            let indices = &groups[&group_name];
            let is_expanded = expanded_groups.contains(&group_name);

            if is_expanded {
                // Show group header with ▼ indicator
                display.push(format!("▼ {}", group_name));
                display_mapping.push(DisplayItem::GroupHeader(group_name.clone()));

                // Show all items in group with indentation
                for &idx in indices {
                    display.push(format!("  {}", menu_items[idx].label));
                    display_mapping.push(DisplayItem::Item(idx));
                }
            } else {
                // Show collapsed group with ▶ indicator
                display.push(format!("▶ {}", group_name));
                display_mapping.push(DisplayItem::GroupHeader(group_name));
            }
        }

        // Add exit option
        display.push("❌ Exit".to_string());
        display_mapping.push(DisplayItem::Exit);

        println!();
        let choice = FuzzySelect::with_theme(&ctx.theme())
            .with_prompt("What would you like to do? (type to filter)")
            .items(&display)
            .default(0)
            .interact()?;

        // Handle selection
        match &display_mapping[choice] {
            DisplayItem::GroupHeader(group_name) => {
                // Toggle group expansion
                if expanded_groups.contains(group_name) {
                    expanded_groups.remove(group_name);
                } else {
                    expanded_groups.insert(group_name.clone());
                }
            }
            DisplayItem::Item(idx) => {
                println!();
                let result: Result<()> = (menu_items[*idx].handler)(ctx).map_err(Into::into);
                if let Err(e) = result {
                    println!();
                    ctx.print_error(&format!("Error: {:#}", e));
                }
            }
            DisplayItem::Exit => {
                break;
            }
        }
    }

    Ok(())
}

fn cmd_update(ctx: &AppContext, force: bool) -> Result<()> {
    ctx.print_header("Checking for updates");

    match devkit_core::update::check_for_updates(force) {
        Ok(Some(info)) => {
            println!();
            ctx.print_warning(&format!(
                "New version available: {} → {}",
                info.current_version, info.latest_version
            ));
            println!();
            println!("Download: {}", info.download_url);
            println!();
            println!("To update:");
            println!(
                "  curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | bash"
            );
            println!();
        }
        Ok(None) => {
            ctx.print_success("✓ You're on the latest version!");
        }
        Err(e) => {
            ctx.print_warning(&format!("Failed to check for updates: {}", e));
            ctx.print_info(
                "You can still check manually at: https://github.com/crcn/devkit/releases",
            );
        }
    }

    Ok(())
}

fn check_for_updates_background(ctx: &AppContext) {
    use std::thread;

    let quiet = ctx.quiet;
    thread::spawn(move || {
        if let Ok(Some(info)) = devkit_core::update::check_for_updates(false) {
            if !quiet {
                eprintln!();
                eprintln!(
                    "💡 Update available: {} → {} (run 'devkit update' for details)",
                    info.current_version, info.latest_version
                );
                eprintln!();
            }
        }
    });
}

fn resolve_aliases(cli: &mut Cli, ctx: &AppContext) {
    let aliases = &ctx.config.global.aliases.aliases;

    if let Some(Commands::Cmd {
        command: Some(cmd), ..
    }) = &mut cli.command
    {
        if let Some(resolved) = aliases.get(cmd.as_str()) {
            tracing::debug!("Resolved alias '{}' to '{}'", cmd, resolved);
            *cmd = resolved.clone();
        }
    }
}

fn cmd_history(ctx: &AppContext, search: Option<&str>) -> Result<()> {
    ctx.print_header("Command History");
    println!();

    let history = match search {
        Some(pattern) => devkit_core::history::search_history(pattern)?,
        None => devkit_core::history::load_history()?,
    };

    if history.is_empty() {
        ctx.print_info("No command history found");
        return Ok(());
    }

    for entry in history.iter().rev().take(20) {
        let status = if entry.success { "✓" } else { "✗" };
        let timestamp = chrono::DateTime::from_timestamp(entry.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        println!("{} {} - {}", status, timestamp, entry.command);
    }

    println!();
    ctx.print_info(&format!("Showing {} entries", history.len().min(20)));

    Ok(())
}
