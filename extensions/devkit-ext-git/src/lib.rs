//! Git extension for devkit
//!
//! Provides git status, release management, and versioning workflows.

mod release;
mod status;
mod version;

pub use release::{create_release, rollback, BumpType, ReleaseOptions};
pub use status::git_status;
pub use version::{get_current_version, get_recent_versions, Version};

/// Check if this extension should be enabled
pub fn should_enable(ctx: &devkit_core::AppContext) -> bool {
    // Enable if we're in a git repository
    ctx.features.git
}
