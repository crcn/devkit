//! CI/CD extension for devkit
//!
//! Provides GitHub Actions integration via the gh CLI.

mod status;
mod workflows;

pub use status::{ci_runs, ci_status};
pub use workflows::{ci_cancel, ci_logs, ci_rerun, ci_trigger, ci_watch};

/// Check if this extension should be enabled
pub fn should_enable(ctx: &devkit_core::AppContext) -> bool {
    // Enable if we're in a git repo and gh CLI is available
    ctx.features.git && devkit_core::cmd_exists("gh")
}

/// Check if GitHub Actions is configured
pub fn has_github_actions(ctx: &devkit_core::AppContext) -> bool {
    ctx.features.github_actions
}
