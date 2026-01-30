//! Commands extension - surfaces [cmd] configurations in interactive menu
//!
//! This extension makes project-specific commands easily discoverable by adding them
//! to the interactive menu. Supports command variants like watch, release, fix, etc.

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, CmdEntry, Extension, MenuItem};
use devkit_tasks::{run_cmd, CmdOptions};
use dialoguer::theme::ColorfulTheme;
use std::collections::HashMap;

pub struct CommandsExtension;

impl Extension for CommandsExtension {
    fn name(&self) -> &str {
        "commands"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.commands
    }

    fn menu_items(&self, ctx: &AppContext) -> Vec<MenuItem> {
        let mut items = Vec::new();
        let mut all_commands: HashMap<String, Vec<(String, Option<String>)>> = HashMap::new();

        // Collect all commands and their variants from all packages
        for (pkg_name, pkg_config) in &ctx.config.packages {
            for (cmd_name, cmd_entry) in &pkg_config.cmd {
                let entry = all_commands.entry(cmd_name.clone()).or_default();

                match cmd_entry {
                    CmdEntry::Simple(_) => {
                        entry.push((pkg_name.clone(), None));
                    }
                    CmdEntry::Full(config) => {
                        // Add default variant
                        entry.push((pkg_name.clone(), None));

                        // Add all variants
                        for variant_name in config.variants.keys() {
                            entry.push((pkg_name.clone(), Some(variant_name.clone())));
                        }
                    }
                }
            }
        }

        // Create menu items for each command
        for (cmd_name, packages) in all_commands {
            // Group by variant
            let mut by_variant: HashMap<Option<String>, Vec<String>> = HashMap::new();
            for (pkg_name, variant) in packages {
                by_variant.entry(variant).or_default().push(pkg_name);
            }

            // Create menu items
            for (variant, _pkgs) in by_variant {
                let emoji = get_command_emoji(&cmd_name);
                let label = if let Some(ref v) = variant {
                    format!("{} {} ({})", emoji, capitalize(&cmd_name), v)
                } else {
                    format!("{} {}", emoji, capitalize(&cmd_name))
                };

                let cmd_name_owned = cmd_name.clone();
                let variant_owned = variant.clone();

                items.push(MenuItem {
                    label,
                    group: None,
                    handler: Box::new(move |ctx| {
                        execute_command(ctx, &cmd_name_owned, variant_owned.as_deref())
                    }),
                });
            }
        }

        // Sort items alphabetically
        items.sort_by(|a, b| a.label.cmp(&b.label));
        items
    }
}

/// Execute a command with optional variant
fn execute_command(
    ctx: &AppContext,
    cmd_name: &str,
    variant: Option<&str>,
) -> devkit_core::Result<()> {
    let opts = CmdOptions {
        packages: vec![],
        parallel: false,
        variant: variant.map(String::from),
        capture: false,
    };

    run_cmd(ctx, cmd_name, &opts).map_err(|e| devkit_core::DevkitError::Other(e))?;
    Ok(())
}

/// Get emoji for command name
fn get_command_emoji(cmd_name: &str) -> &'static str {
    match cmd_name {
        "build" => "🔨",
        "test" => "🧪",
        "lint" => "🔍",
        "fmt" | "format" => "💅",
        "typecheck" => "📝",
        "dev" | "serve" => "🚀",
        "deploy" => "☁️",
        "migrate" | "migration" => "🗄️",
        "seed" => "🌱",
        "clean" => "🧹",
        "watch" => "👀",
        "start" => "▶️",
        "stop" => "⏹️",
        "restart" => "🔄",
        "run" => "▶️",
        _ => "⚙️",
    }
}

/// Capitalize first letter of string
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
