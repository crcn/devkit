//! Test extension for devkit
//!
//! Provides test running, coverage, and watch functionality for Rust and JavaScript projects.

use devkit_core::{AppContext, Extension, MenuItem};

mod coverage;
mod test;
mod watch;

pub use coverage::{run_coverage, CoverageOptions};
pub use test::{run_tests, TestOptions};
pub use watch::watch_tests;

pub struct TestExtension;

impl Extension for TestExtension {
    fn name(&self) -> &str {
        "test"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // DEPRECATED: Most projects should use [cmd.test] with variants instead.
        // This extension provides coverage tools, but is opt-in only.
        // Disabled by default - use [cmd.test] with variants instead
        false
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🧪 Test - Run All".to_string(),
                handler: Box::new(|ctx| {
                    run_tests(ctx, &TestOptions::default())?;
                    Ok(())
                }),
            },
            MenuItem {
                label: "🧪 Test - Watch".to_string(),
                handler: Box::new(|ctx| watch_tests(ctx, None).map_err(Into::into)),
            },
            MenuItem {
                label: "📊 Test - Coverage".to_string(),
                handler: Box::new(|ctx| {
                    run_coverage(ctx, &CoverageOptions::default()).map_err(Into::into)
                }),
            },
        ]
    }
}
