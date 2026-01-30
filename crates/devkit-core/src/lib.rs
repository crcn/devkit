//! Core types and utilities for devkit

pub mod config;
pub mod context;
pub mod detection;
pub mod error;
pub mod extension;
pub mod extension_loader;
pub mod external_extension;
pub mod history;
pub mod init;
pub mod output;
pub mod update;
pub mod utils;
pub mod validation;

pub use config::{CmdEntry, Config};
pub use context::AppContext;
pub use detection::Features;
pub use error::{DevkitError, Result};
pub use extension::{Extension, ExtensionRegistry, MenuItem};
pub use utils::{cmd_exists, docker_available};
pub use validation::{validate_config, ValidationReport};
