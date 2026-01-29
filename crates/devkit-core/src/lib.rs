//! Core types and utilities for devkit

pub mod config;
pub mod context;
pub mod detection;
pub mod error;
pub mod extension;
pub mod utils;
pub mod validation;

pub use config::Config;
pub use context::AppContext;
pub use detection::Features;
pub use error::{DevkitError, Result};
pub use extension::{Extension, ExtensionRegistry, MenuItem};
pub use utils::cmd_exists;
pub use validation::{validate_config, ValidationReport};
