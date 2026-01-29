//! Application context with shared state and utilities

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::path::PathBuf;

use crate::config::Config;
use crate::detection::Features;
use crate::utils::repo_root;

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
        let config = Config::load(&repo)?;
        let features = Features::detect(&repo, &config);
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
