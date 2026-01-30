//! Extension trait for devkit plugins
//!
//! Extensions can:
//! - Register interactive menu items
//! - Provide commands
//! - Handle submenus

use crate::{AppContext, Result};

/// Menu item that an extension provides
pub struct MenuItem {
    /// Display label (with emoji)
    pub label: String,
    /// Handler function
    pub handler: Box<dyn Fn(&AppContext) -> Result<()>>,
}

/// Extension trait - implement this to create a devkit extension
pub trait Extension {
    /// Extension name (e.g., "docker", "database")
    fn name(&self) -> &str;

    /// Check if this extension is available in the current project
    /// Uses AppContext.features for auto-detection
    fn is_available(&self, ctx: &AppContext) -> bool;

    /// Get menu items for the main interactive menu
    /// Only called if is_available() returns true
    fn menu_items(&self) -> Vec<MenuItem>;

    /// Optional: Handle CLI subcommand
    /// Return None if this extension doesn't handle CLI commands directly
    fn handle_command(&self, _ctx: &AppContext, _command: &str, _args: &[String]) -> Option<Result<()>> {
        None
    }

    /// Optional: Prerun hook - runs on startup before any commands
    /// Use this to ensure the repo is in a runnable state
    /// Examples: install dependencies, pull docker images, run migrations
    /// Return Ok(()) if everything is ready, Err if setup failed
    fn prerun(&self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}

/// Extension registry - collects all extensions
pub struct ExtensionRegistry {
    extensions: Vec<Box<dyn Extension>>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn register(&mut self, extension: Box<dyn Extension>) {
        self.extensions.push(extension);
    }

    /// Get all available extensions for the current project
    pub fn available_extensions<'a>(&'a self, ctx: &'a AppContext) -> Vec<&'a Box<dyn Extension>> {
        self.extensions
            .iter()
            .filter(|ext| ext.is_available(ctx))
            .collect()
    }

    /// Get all menu items from available extensions
    pub fn menu_items(&self, ctx: &AppContext) -> Vec<MenuItem> {
        self.available_extensions(ctx)
            .into_iter()
            .flat_map(|ext| ext.menu_items())
            .collect()
    }

    /// Run all prerun hooks from available extensions
    /// Returns the first error encountered, or Ok if all succeeded
    pub fn run_prerun_hooks(&self, ctx: &AppContext) -> Result<()> {
        for ext in self.available_extensions(ctx) {
            ext.prerun(ctx)?;
        }
        Ok(())
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
