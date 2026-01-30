//! Application context with shared state and utilities

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::path::PathBuf;
use tracing::{info, warn};

use crate::config::Config;
use crate::detection::Features;
use crate::utils::repo_root;
use crate::validation::validate_config;

/// Application context passed to all commands
pub struct AppContext {
    pub repo: PathBuf,
    pub quiet: bool,
    pub config: Config,
    pub features: Features,
}

impl AppContext {
    pub fn new(quiet: bool) -> Result<Self> {
        let repo = repo_root()?;
        info!("Repository root: {}", repo.display());

        let config = Config::load(&repo)?;
        info!("Loaded config with {} packages", config.packages.len());

        // Validate configuration
        let validation = validate_config(&config)?;

        if !quiet {
            // Show warnings
            for warning in &validation.warnings {
                warn!("{}", warning);
            }
        }

        // Fail if there are errors
        if !validation.is_valid() {
            eprintln!("{}", style("Configuration validation failed:").red().bold());
            for error in &validation.errors {
                eprintln!("  {} {}", style("✗").red(), error);
            }
            return Err(anyhow::anyhow!("Configuration validation failed"));
        }

        let features = Features::detect(&repo, &config);
        info!(
            "Detected features: docker={}, git={}, cargo={}, node={}",
            features.docker, features.git, features.cargo, features.node
        );

        Ok(Self {
            repo,
            quiet,
            config,
            features,
        })
    }

    pub fn theme(&self) -> ColorfulTheme {
        ColorfulTheme::default()
    }

    pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        if self.quiet {
            return Ok(default);
        }
        Ok(Confirm::with_theme(&self.theme())
            .with_prompt(prompt)
            .default(default)
            .interact()?)
    }

    pub fn print_header(&self, msg: &str) {
        if !self.quiet {
            println!();
            println!("{}", style(msg).bold());
        }
    }

    pub fn print_success(&self, msg: &str) {
        if !self.quiet {
            println!("{}", style(msg).green());
        }
    }

    pub fn print_warning(&self, msg: &str) {
        if !self.quiet {
            println!("{}", style(msg).yellow());
        }
    }

    pub fn print_info(&self, msg: &str) {
        if !self.quiet {
            println!("{}", style(msg).cyan());
        }
    }

    pub fn print_error(&self, msg: &str) {
        if !self.quiet {
            eprintln!("{}", style(msg).red());
        }
    }
}
