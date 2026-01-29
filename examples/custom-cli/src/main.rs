//! Example custom CLI using devkit
//!
//! This demonstrates how to build a custom dev CLI that:
//! - Uses devkit extensions for generic functionality
//! - Adds project-specific commands
//! - Auto-detects features and shows relevant commands

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use devkit_core::AppContext;
use dialoguer::Select;

#[derive(Parser)]
#[command(name = "dev")]
#[command(about = "Example project development CLI")]
struct Cli {
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

    /// Docker operations
    Docker {
        #[command(subcommand)]
        action: DockerAction,
    },

    /// Run package commands from dev.toml
    Cmd {
        command: String,
        #[arg(long)]
        parallel: bool,
    },

    /// Custom project command
    Custom { message: String },
}

#[derive(Subcommand)]
enum DockerAction {
    Up,
    Down,
    Logs,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let ctx = AppContext::new(cli.quiet)?;

    match cli.command {
        Some(cmd) => run_command(&ctx, cmd),
        None => run_interactive(&ctx),
    }
}

fn run_command(ctx: &AppContext, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Start => {
            ctx.print_header("Starting development environment");

            if ctx.features.docker {
                ctx.print_info("Docker detected - starting containers");
                devkit_ext_docker::compose_up(ctx, &[], false)?;
            } else {
                ctx.print_warning("Docker not detected - skipping container startup");
            }

            ctx.print_success("Development environment ready!");
            Ok(())
        }

        Commands::Stop => {
            if ctx.features.docker {
                devkit_ext_docker::compose_down(ctx)?;
            }
            Ok(())
        }

        Commands::Docker { action } => match action {
            DockerAction::Up => devkit_ext_docker::compose_up(ctx, &[], false),
            DockerAction::Down => devkit_ext_docker::compose_down(ctx),
            DockerAction::Logs => {
                let containers = devkit_ext_docker::list_running_containers(ctx)?;
                if let Some(c) = containers.first() {
                    devkit_ext_docker::follow_logs(ctx, &c.id)
                } else {
                    Err(anyhow::anyhow!("No containers running"))
                }
            }
        },

        Commands::Cmd { command, parallel } => {
            use devkit_tasks::{run_cmd, CmdOptions};

            let opts = CmdOptions {
                parallel,
                variant: None,
                packages: vec![],
                capture: false,
            };

            let results = run_cmd(ctx, &command, &opts)?;
            devkit_tasks::print_results(ctx, &results);

            if results.iter().any(|r| !r.success) {
                return Err(anyhow::anyhow!("Some commands failed"));
            }
            Ok(())
        }

        Commands::Custom { message } => {
            ctx.print_header("Custom Project Command");
            ctx.print_info(&format!("Message: {}", message));
            ctx.print_success("This is your project-specific command!");
            Ok(())
        }
    }
}

fn run_interactive(ctx: &AppContext) -> Result<()> {
    println!();
    println!("{}", style("Example Project Dev CLI").bold().cyan());
    println!("{}", style("─".repeat(40)).dim());
    println!();

    // Show detected features
    println!("Detected features:");
    if ctx.features.docker {
        println!("  {} Docker", style("✓").green());
    }
    if ctx.features.git {
        println!("  {} Git", style("✓").green());
    }
    if ctx.features.commands {
        println!("  {} Package commands", style("✓").green());
    }
    println!();

    loop {
        let mut items = vec![];

        // Generic commands (auto-show based on detection)
        items.push("Start development environment");

        if ctx.features.docker {
            items.push("Docker operations");
        }

        if ctx.features.commands {
            items.push("Run package command");
        }

        // Separator
        items.push("─────────────────────");

        // Project-specific commands
        items.push("Custom project command");

        items.push("─────────────────────");
        items.push("Exit");

        let choice = Select::with_theme(&ctx.theme())
            .with_prompt("What do you want to do?")
            .items(&items)
            .default(0)
            .interact()?;

        match items[choice] {
            "Start development environment" => {
                run_command(ctx, Commands::Start)?;
            }
            "Docker operations" => {
                docker_menu(ctx)?;
            }
            "Run package command" => {
                cmd_menu(ctx)?;
            }
            "Custom project command" => {
                run_command(
                    ctx,
                    Commands::Custom {
                        message: "Hello from interactive menu!".to_string(),
                    },
                )?;
            }
            "Exit" => break,
            _ => {}
        }
    }

    Ok(())
}

fn docker_menu(ctx: &AppContext) -> Result<()> {
    let items = vec![
        "Start containers",
        "Stop containers",
        "View logs",
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
            } else {
                ctx.print_warning("No containers running");
            }
        }
        _ => {}
    }

    Ok(())
}

fn cmd_menu(ctx: &AppContext) -> Result<()> {
    use devkit_tasks::list_commands;

    let commands = list_commands(&ctx.config);
    if commands.is_empty() {
        ctx.print_warning("No commands defined in dev.toml files");
        return Ok(());
    }

    let items: Vec<_> = commands.keys().map(|s| s.as_str()).collect();

    let choice = Select::with_theme(&ctx.theme())
        .with_prompt("Select command to run")
        .items(&items)
        .interact()?;

    run_command(
        ctx,
        Commands::Cmd {
            command: items[choice].to_string(),
            parallel: false,
        },
    )
}
