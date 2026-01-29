//! Git extension for devkit
//!
//! Provides git status, release management, and versioning workflows.

use devkit_core::{AppContext, Extension, MenuItem};

mod release;
mod status;
mod version;

pub use release::{create_release, rollback, BumpType, ReleaseOptions};
pub use status::git_status;
pub use version::{get_current_version, get_recent_versions, Version};

pub struct GitExtension;

impl Extension for GitExtension {
    fn name(&self) -> &str {
        "git"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.git
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "📊 Git - Status".to_string(),
                handler: Box::new(|ctx| {
                    git_status(ctx).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "🚀 Git - Release (Patch)".to_string(),
                handler: Box::new(|ctx| {
                    create_release(ctx, &ReleaseOptions {
                        bump: BumpType::Patch,
                        ..Default::default()
                    }).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "🚀 Git - Release (Minor)".to_string(),
                handler: Box::new(|ctx| {
                    create_release(ctx, &ReleaseOptions {
                        bump: BumpType::Minor,
                        ..Default::default()
                    }).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "🚀 Git - Release (Major)".to_string(),
                handler: Box::new(|ctx| {
                    create_release(ctx, &ReleaseOptions {
                        bump: BumpType::Major,
                        ..Default::default()
                    }).map_err(Into::into)
                }),
            },
        ]
    }
}
