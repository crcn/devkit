//! devkit - Universal command discovery and execution for any codebase
//!
//! Discovers and surfaces all available commands in a repository by scanning
//! package managers, build tools, scripts, and services.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use std::collections::HashMap;
use std::io;
use std::process::ExitCode;

use devkit_core::{
    discovery::{
        CargoProvider, CommandHistory, DiscoveryEngine, MakefileProvider, NpmProvider,
        ScriptProvider,
    },
    AppContext, Category, ExtensionRegistry,
};

#[derive(Parser)]
#[command(name = "devkit")]
#[command(about = "Universal command discovery for any codebase", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize devkit in a new project
    Init {
        /// Skip interactive prompts
        #[arg(long)]
        no_interactive: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },

    /// Check for updates
    Update,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {:#}", e);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn run(cli: Cli) -> Result<()> {
    // Handle subcommands
    match cli.command {
        Some(Commands::Init { no_interactive }) => {
            let ctx = AppContext::new(false)?;
            devkit_core::init::init_project(&ctx.repo, !no_interactive)?;
            return Ok(());
        }

        Some(Commands::Completions { shell }) => {
            generate_completions(shell);
            return Ok(());
        }

        Some(Commands::Update) => {
            devkit_core::update::check_for_updates(false)?;
            return Ok(());
        }

        None => {
            // Interactive menu mode
        }
    }

    // Load context
    let ctx = AppContext::new(false)?;

    // Setup extension registry (rich interactive commands)
    let mut extensions = ExtensionRegistry::new();

    #[cfg(feature = "docker")]
    extensions.register(Box::new(devkit_ext_docker::DockerExtension));

    // Setup discovery engine (auto-discovered commands)
    let mut engine = DiscoveryEngine::new();
    engine.register(Box::new(CargoProvider::new()));
    engine.register(Box::new(NpmProvider::new()));
    engine.register(Box::new(MakefileProvider::new()));
    engine.register(Box::new(ScriptProvider::new()));

    // Load command history
    let mut history = CommandHistory::load(&ctx.repo).unwrap_or_default();

    // Print header
    println!();
    println!("╭─────────────────────────────────────────╮");
    println!("│  🚀 devkit - Command Discovery          │");
    println!("╰─────────────────────────────────────────╯");
    println!();
    println!("Repository: {}", ctx.repo.display());
    println!("Project: {}", ctx.config.global.project.name);
    println!();

    // Interactive menu loop
    loop {
        // Get extension menu items (rich interactive commands)
        let extension_items = extensions.menu_items(&ctx);

        // Discover all commands (auto-discovered)
        let commands = engine.discover(&ctx);

        // Group discovered commands by category
        let mut grouped: HashMap<Category, Vec<&devkit_core::DiscoveredCommand>> = HashMap::new();
        for cmd in commands {
            grouped.entry(cmd.category).or_default().push(cmd);
        }

        // Build combined menu
        let mut menu_items: Vec<String> = Vec::new();
        let mut command_map: Vec<&devkit_core::DiscoveredCommand> = Vec::new();
        let mut extension_handler_map: Vec<&devkit_core::MenuItem> = Vec::new();

        // Add extension menu items first (interactive commands)
        if !extension_items.is_empty() {
            for item in &extension_items {
                menu_items.push(item.label.clone());
                extension_handler_map.push(item);
            }
            menu_items.push("".to_string());
        }

        // Show recent commands
        let recent = history.recent_commands();
        if !recent.is_empty() {
            menu_items.push("─── Recent ───".to_string());
            for entry in recent.iter().take(5) {
                // Find the command in discovered commands
                if let Some(cmd) = commands.iter().find(|c| c.id == entry.id) {
                    menu_items.push(format!("↻ {}", cmd.label));
                    command_map.push(cmd);
                }
            }
            menu_items.push("".to_string());
        }

        // Group commands by category
        let category_order = [
            Category::Dev,
            Category::Build,
            Category::Test,
            Category::Quality,
            Category::Services,
            Category::Database,
            Category::Deploy,
            Category::Git,
            Category::Dependencies,
            Category::Scripts,
            Category::Other,
        ];

        for category in &category_order {
            if let Some(cmds) = grouped.get(category) {
                if !cmds.is_empty() {
                    // Category header
                    menu_items.push(format!(
                        "─── {} {} ({}) ───",
                        category.emoji(),
                        category.label(),
                        cmds.len()
                    ));

                    // Add commands
                    for cmd in cmds {
                        menu_items.push(cmd.label.clone());
                        command_map.push(cmd);
                    }

                    menu_items.push("".to_string());
                }
            }
        }

        // Add multi-select option at the end
        if !extension_items.is_empty() || !command_map.is_empty() {
            menu_items.push("".to_string());
            menu_items.push("⚡ Run multiple commands...".to_string());
        }

        // Show menu
        let selection = FuzzySelect::with_theme(&ctx.theme())
            .with_prompt("What would you like to do? (type to filter)")
            .items(&menu_items)
            .default(0)
            .interact_opt()?;

        // Handle selection
        match selection {
            Some(idx) => {
                // Check if it's a separator or empty line
                let selected_text = &menu_items[idx];
                if selected_text.starts_with("───") || selected_text.is_empty() {
                    continue;
                }

                // Check if multi-select mode was chosen
                if selected_text == "⚡ Run multiple commands..." {
                    let ext_refs: Vec<&devkit_core::MenuItem> = extension_items.iter().collect();
                    handle_multi_select(&ctx, &ext_refs, &command_map, &mut history)?;
                    continue;
                }

                // Calculate which item was selected (extension vs discovered command)
                let mut item_idx = 0;

                for (i, item) in menu_items.iter().enumerate() {
                    if i == idx {
                        if !item.starts_with("───") && !item.is_empty() {
                            break;
                        }
                    }

                    if !item.starts_with("───") && !item.is_empty() {
                        if i < idx {
                            item_idx += 1;
                        }
                    }
                }

                // Check if it's an extension item
                if item_idx < extension_handler_map.len() {
                    let item = extension_handler_map[item_idx];

                    println!();
                    println!("Running: {}", item.label);
                    println!("─────────────────────────────────────────");
                    println!();

                    let result = (item.handler)(&ctx);

                    println!();
                    if let Err(e) = result {
                        ctx.print_error(&format!("Error: {:#}", e));
                    } else {
                        ctx.print_success("✓ Command completed");
                    }
                    println!();
                } else {
                    // It's a discovered command
                    let cmd_idx = item_idx - extension_handler_map.len();
                    if cmd_idx < command_map.len() {
                        let cmd = command_map[cmd_idx];

                        println!();
                        println!("Running: {}", cmd.label);
                        println!("─────────────────────────────────────────");
                        println!();

                        let result = cmd.execute(&ctx);

                        println!();
                        if let Err(e) = result {
                            ctx.print_error(&format!("Error: {:#}", e));
                        } else {
                            ctx.print_success("✓ Command completed");

                            // Record in history
                            history.record(&cmd.id, &cmd.label);
                            let _ = history.save(&ctx.repo);
                        }
                        println!();
                    }
                }
            }
            None => {
                // User pressed Ctrl+C
                println!();
                break;
            }
        }
    }

    Ok(())
}

fn handle_multi_select(
    ctx: &AppContext,
    extension_items: &[&devkit_core::MenuItem],
    command_map: &[&devkit_core::DiscoveredCommand],
    history: &mut CommandHistory,
) -> Result<()> {
    // Build selectable items list with indices
    let mut items = Vec::new();
    let mut item_indices = Vec::new(); // (type: 0=extension, 1=command, index)

    // Add extension items
    for (i, item) in extension_items.iter().enumerate() {
        items.push(item.label.clone());
        item_indices.push((0, i));
    }

    // Add discovered commands
    for (i, cmd) in command_map.iter().enumerate() {
        items.push(cmd.label.clone());
        item_indices.push((1, i));
    }

    if items.is_empty() {
        println!("No commands available for multi-select");
        return Ok(());
    }

    // Show multi-select dialog
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commands to run (space to select, enter to confirm)")
        .items(&items)
        .interact()?;

    if selections.is_empty() {
        println!("No commands selected");
        return Ok(());
    }

    println!();
    println!("Running {} commands sequentially...", selections.len());
    println!("─────────────────────────────────────────");
    println!();

    // Execute selected commands sequentially
    let mut results: Vec<(String, Result<()>)> = Vec::new();

    for &idx in &selections {
        let (item_type, item_idx) = item_indices[idx];

        match item_type {
            0 => {
                // Extension item
                let ext_item = extension_items[item_idx];
                println!("▶ Running: {}", ext_item.label);
                println!();

                let result = (ext_item.handler)(ctx);
                // Convert to anyhow::Result
                let anyhow_result: Result<()> = result.map_err(|e| anyhow::anyhow!("{:#}", e));
                results.push((ext_item.label.clone(), anyhow_result));

                println!();
            }
            1 => {
                // Discovered command
                let cmd = command_map[item_idx];
                println!("▶ Running: {}", cmd.label);
                println!();

                let result = cmd.execute(ctx);

                // Record in history if successful
                let is_ok = result.is_ok();
                if is_ok {
                    history.record(&cmd.id, &cmd.label);
                }

                // Convert result to anyhow::Result for consistent error handling
                let anyhow_result: Result<()> = result.map_err(|e| anyhow::anyhow!("{:#}", e));
                results.push((cmd.label.clone(), anyhow_result));

                println!();
            }
            _ => {}
        }
    }

    // Show summary
    println!("─────────────────────────────────────────");
    println!("Summary:");
    println!();

    let mut success_count = 0;
    let mut error_count = 0;

    for (label, result) in &results {
        match result {
            Ok(_) => {
                ctx.print_success(&format!("✓ {}", label));
                success_count += 1;
            }
            Err(e) => {
                ctx.print_error(&format!("✗ {}: {:#}", label, e));
                error_count += 1;
            }
        }
    }

    println!();
    println!("─────────────────────────────────────────");
    println!(
        "Completed: {} succeeded, {} failed",
        success_count, error_count
    );
    println!();

    // Save history
    let _ = history.save(&ctx.repo);

    Ok(())
}

fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
}
