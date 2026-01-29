//! CI/CD extension for devkit
//!
//! Provides GitHub Actions integration via the gh CLI.

use devkit_core::{AppContext, Extension, MenuItem};

mod status;
mod workflows;

pub use status::{ci_runs, ci_status};
pub use workflows::{ci_cancel, ci_logs, ci_rerun, ci_trigger, ci_watch};

pub struct CiExtension;

impl Extension for CiExtension {
    fn name(&self) -> &str {
        "ci"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.git && devkit_core::cmd_exists("gh")
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "🔄 CI - Status".to_string(),
                handler: Box::new(|ctx| {
                    ci_status(ctx, None).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "🔄 CI - Watch Latest".to_string(),
                handler: Box::new(|ctx| {
                    ci_watch(ctx, None).map_err(Into::into)
                }),
            },
            MenuItem {
                label: "📋 CI - List Runs".to_string(),
                handler: Box::new(|ctx| {
                    ci_runs(ctx, 10, None).map_err(Into::into)
                }),
            },
        ]
    }
}
