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

    fn menu_items(&self) -> Vec<MenuItem> {
        use devkit_core::DevkitError;
        vec![MenuItem {
            label: "📦 Install dependencies".to_string(),
            handler: Box::new(|ctx| {
                println!();
                check_and_install(ctx).map_err(DevkitError::from)
            }),
        }]
    }

    fn handle_command(&self, ctx: &AppContext, command: &str, _args: &[String]) -> Option<Result<()>> {
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

    /// Prerun hook - automatically install dependencies on startup
    fn prerun(&self, ctx: &AppContext) -> Result<()> {
        use devkit_core::DevkitError;
        if ctx.quiet {
            // In quiet mode, just check and install if needed
            check_and_install(ctx).map_err(DevkitError::from)?;
        } else {
            // In interactive mode, show what we found
            let packages = crate::discover_packages(ctx);
            let needs_install: Vec<_> = packages.iter().filter(|p| p.needs_install).collect();

            if !needs_install.is_empty() {
                ctx.print_info(&format!(
                    "Found {} package(s) that need dependencies installed",
                    needs_install.len()
                ));

                for pkg in &needs_install {
                    println!(
                        "  {} [{}] via {}",
                        pkg.name,
                        pkg.language.name(),
                        pkg.package_manager.name()
                    );
                }

                // Ask for confirmation before installing
                if ctx.confirm("Install dependencies now?", true)? {
                    check_and_install(ctx).map_err(DevkitError::from)?;
                } else {
                    ctx.print_warning("Skipping dependency installation");
                    ctx.print_info("Run './dev.sh deps' to install later");
                }
            }
        }

        Ok(())
    }
}
