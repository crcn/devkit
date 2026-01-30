//! Extension trait implementation for deps

use devkit_core::{AppContext, Extension, MenuItem, Result};

use crate::{check_and_install, print_summary};

pub struct DepsExtension;

impl Extension for DepsExtension {
    fn name(&self) -> &str {
        "deps"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        // Available if any packages are detected
        !crate::discover_packages(ctx).is_empty()
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        use devkit_core::DevkitError;
        vec![MenuItem {
            label: "📦 Install dependencies".to_string(),
            handler: Box::new(|ctx| {
                println!();
                check_and_install(ctx).map_err(DevkitError::from)
            }),
        }]
    }

    fn handle_command(
        &self,
        ctx: &AppContext,
        command: &str,
        _args: &[String],
    ) -> Option<Result<()>> {
        use devkit_core::DevkitError;
        match command {
            "deps" | "install" => Some(check_and_install(ctx).map_err(DevkitError::from)),
            "deps:list" => Some({
                print_summary(ctx);
                Ok(())
            }),
            _ => None,
        }
    }

    /// Prerun hook - disabled to avoid prompting on every run
    /// Users can run `devkit deps` or `./dev deps` manually to install dependencies
    fn prerun(&self, _ctx: &AppContext) -> Result<()> {
        // Disabled: was prompting every time devkit runs
        // Users should explicitly run `devkit deps` when needed
        Ok(())
    }
}
