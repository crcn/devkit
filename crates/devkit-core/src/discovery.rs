//! Command Discovery System
//!
//! The core of devkit - discovers all available commands in a repository
//! by scanning package managers, build tools, scripts, and services.

pub mod cargo_provider;
pub mod docker_provider;
pub mod history;
pub mod makefile_provider;
pub mod npm_provider;
pub mod script_provider;

use anyhow::Result;

use crate::context::AppContext;

pub use cargo_provider::CargoProvider;
pub use docker_provider::DockerProvider;
pub use history::CommandHistory;
pub use makefile_provider::MakefileProvider;
pub use npm_provider::NpmProvider;
pub use script_provider::ScriptProvider;

/// Categories for organizing discovered commands
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    /// Build and compilation commands
    Build,
    /// Test execution commands
    Test,
    /// Code quality (lint, format, typecheck)
    Quality,
    /// Services (Docker, databases, servers)
    Services,
    /// Database operations (migrations, seeds)
    Database,
    /// Development servers and watch modes
    Dev,
    /// Deployment and release commands
    Deploy,
    /// Git operations
    Git,
    /// Dependency management
    Dependencies,
    /// Custom scripts and tools
    Scripts,
    /// Uncategorized commands
    Other,
}

impl Category {
    pub fn emoji(&self) -> &'static str {
        match self {
            Category::Build => "📦",
            Category::Test => "🧪",
            Category::Quality => "✨",
            Category::Services => "🐳",
            Category::Database => "🗄️",
            Category::Dev => "🔥",
            Category::Deploy => "🚀",
            Category::Git => "🌿",
            Category::Dependencies => "📚",
            Category::Scripts => "📝",
            Category::Other => "🔧",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Category::Build => "Build",
            Category::Test => "Test",
            Category::Quality => "Quality",
            Category::Services => "Services",
            Category::Database => "Database",
            Category::Dev => "Development",
            Category::Deploy => "Deploy",
            Category::Git => "Git",
            Category::Dependencies => "Dependencies",
            Category::Scripts => "Scripts",
            Category::Other => "Other",
        }
    }
}

/// Scope of a command - workspace, package, or global
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandScope {
    /// Run across entire workspace
    Workspace,
    /// Run in specific package
    Package(String),
    /// Not package-specific (global tools, services)
    Global,
}

impl CommandScope {
    pub fn label(&self) -> String {
        match self {
            CommandScope::Workspace => "workspace".to_string(),
            CommandScope::Package(name) => name.clone(),
            CommandScope::Global => "global".to_string(),
        }
    }
}

/// A discovered command that can be executed
pub struct DiscoveredCommand {
    /// Unique identifier (e.g., "npm.build", "docker.logs")
    pub id: String,

    /// Display label for the menu
    pub label: String,

    /// Detailed description
    pub description: String,

    /// Source file/tool (e.g., "package.json", "Cargo.toml", "Makefile")
    pub source: String,

    /// Command category for grouping
    pub category: Category,

    /// Scope (workspace/package/global)
    pub scope: CommandScope,

    /// Execution handler
    pub handler: Box<dyn Fn(&AppContext) -> Result<()> + Send + Sync>,
}

impl DiscoveredCommand {
    /// Create a new discovered command
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        category: Category,
    ) -> Self {
        let id = id.into();
        let label = label.into();

        Self {
            id,
            label,
            description: String::new(),
            source: String::new(),
            category,
            scope: CommandScope::Global,
            handler: Box::new(|_| Ok(())),
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Set scope
    pub fn scope(mut self, scope: CommandScope) -> Self {
        self.scope = scope;
        self
    }

    /// Set handler
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(&AppContext) -> Result<()> + Send + Sync + 'static,
    {
        self.handler = Box::new(handler);
        self
    }

    /// Execute this command
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        (self.handler)(ctx)
    }
}

/// Trait for command providers
///
/// Providers discover commands by inspecting the repository.
/// Discovery MUST be fast and non-interactive - no shelling out,
/// no heavy IO. Inspect files only.
pub trait CommandProvider: Send + Sync {
    /// Provider name for debugging
    fn name(&self) -> &'static str;

    /// Check if this provider is relevant for this repository
    /// Should be a fast check (file existence, config inspection)
    fn is_available(&self, ctx: &AppContext) -> bool;

    /// Discover commands in this repository
    /// MUST be fast - no shell execution, minimal IO
    /// Results should be cacheable
    fn discover(&self, ctx: &AppContext) -> Result<Vec<DiscoveredCommand>>;
}

/// Discovery engine that aggregates all providers
pub struct DiscoveryEngine {
    providers: Vec<Box<dyn CommandProvider>>,
    cache: Option<Vec<DiscoveredCommand>>,
}

impl DiscoveryEngine {
    /// Create a new discovery engine
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            cache: None,
        }
    }

    /// Register a command provider
    pub fn register(&mut self, provider: Box<dyn CommandProvider>) {
        self.providers.push(provider);
    }

    /// Discover all commands (with caching)
    /// Returns references with the same lifetime as self
    pub fn discover(&mut self, ctx: &AppContext) -> &[DiscoveredCommand] {
        if self.cache.is_none() {
            let mut commands = Vec::new();

            for provider in &self.providers {
                if provider.is_available(ctx) {
                    if let Ok(cmds) = provider.discover(ctx) {
                        commands.extend(cmds);
                    }
                }
            }

            self.cache = Some(commands);
        }

        self.cache.as_ref().unwrap()
    }

    /// Force refresh the cache
    pub fn refresh(&mut self) {
        self.cache = None;
    }
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}
