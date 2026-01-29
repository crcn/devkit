//! Test extension for devkit
//!
//! Provides test running, coverage, and watch functionality for Rust and JavaScript projects.

mod coverage;
mod test;
mod watch;

pub use coverage::run_coverage;
pub use test::run_tests;
pub use watch::watch_tests;

/// Check if this extension should be enabled
pub fn should_enable(ctx: &devkit_core::AppContext) -> bool {
    // Enable if we have Rust or Node projects
    ctx.features.cargo || ctx.features.node
}
