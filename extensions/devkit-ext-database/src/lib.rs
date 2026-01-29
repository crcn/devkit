//! Database operations

use anyhow::{anyhow, Result};
use devkit_core::{AppContext, Extension, MenuItem, utils::cmd_exists};
use std::process::Command;

pub struct DatabaseExtension;

impl Extension for DatabaseExtension {
    fn name(&self) -> &str {
        "database"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.database
    }

    fn menu_items(&self) -> Vec<MenuItem> {
        use devkit_core::DevkitError;
        vec![
            MenuItem {
                label: "🗄  Database - Migrate".to_string(),
                handler: Box::new(|ctx| migrate(ctx).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "🗄  Database - Reset".to_string(),
                handler: Box::new(|ctx| reset(ctx).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "🗄  Database - Seed".to_string(),
                handler: Box::new(|ctx| seed(ctx).map_err(DevkitError::from)),
            },
            MenuItem {
                label: "🗄  Database - Shell".to_string(),
                handler: Box::new(|ctx| shell(ctx).map_err(DevkitError::from)),
            },
        ]
    }
}

pub fn migrate(ctx: &AppContext) -> Result<()> {
    if !cmd_exists("sqlx") {
        return Err(anyhow!("sqlx-cli not installed. Run: cargo install sqlx-cli"));
    }

    ctx.print_info("Running migrations...");

    let status = Command::new("sqlx")
        .args(["migrate", "run"])
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Migration failed"));
    }

    ctx.print_success("✓ Migrations complete");
    Ok(())
}

pub fn reset(ctx: &AppContext) -> Result<()> {
    ctx.print_warning("This will drop and recreate the database!");

    if !ctx.confirm("Are you sure?", false)? {
        ctx.print_info("Cancelled");
        return Ok(());
    }

    ctx.print_info("Resetting database...");

    // Drop
    let status = Command::new("sqlx")
        .args(["database", "drop", "-y"])
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Drop failed"));
    }

    // Create
    let status = Command::new("sqlx")
        .args(["database", "create"])
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Create failed"));
    }

    // Migrate
    migrate(ctx)?;

    ctx.print_success("✓ Database reset");
    Ok(())
}

pub fn seed(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Seeding database...");

    // Look for seed file
    let seed_file = ctx.repo.join("seeds/dev.sql");
    if !seed_file.exists() {
        return Err(anyhow!("Seed file not found: {}", seed_file.display()));
    }

    let status = Command::new("psql")
        .arg(std::env::var("DATABASE_URL")?)
        .arg("-f")
        .arg(&seed_file)
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Seed failed"));
    }

    ctx.print_success("✓ Database seeded");
    Ok(())
}

pub fn shell(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Opening database shell...");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/dev".to_string());

    let status = Command::new("psql")
        .arg(database_url)
        .current_dir(&ctx.repo)
        .status()?;

    if !status.success() {
        return Err(anyhow!("psql failed"));
    }

    Ok(())
}
