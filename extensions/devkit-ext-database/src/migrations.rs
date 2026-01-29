//! Database migration management

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;
use std::path::PathBuf;

/// Create the database
pub fn db_create(ctx: &AppContext, database_url: &str) -> Result<()> {
    ctx.print_header("Creating database");

    // Check if sqlx is available
    if !devkit_core::cmd_exists("sqlx") {
        return Err(anyhow!(
            "sqlx CLI not found. Install with: cargo install sqlx-cli"
        ));
    }

    let code = CmdBuilder::new("sqlx")
        .args(["database", "create"])
        .env("DATABASE_URL", database_url)
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to create database"));
    }

    ctx.print_success("Database created");
    Ok(())
}

/// Drop the database
pub fn db_drop(ctx: &AppContext, database_url: &str, force: bool) -> Result<()> {
    ctx.print_header("Dropping database");

    if !devkit_core::cmd_exists("sqlx") {
        return Err(anyhow!(
            "sqlx CLI not found. Install with: cargo install sqlx-cli"
        ));
    }

    let mut args = vec!["database".to_string(), "drop".to_string()];
    if force {
        args.push("-y".to_string());
    }

    let code = CmdBuilder::new("sqlx")
        .args(&args)
        .env("DATABASE_URL", database_url)
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Failed to drop database"));
    }

    ctx.print_success("Database dropped");
    Ok(())
}

/// Run database migrations
pub fn db_migrate(
    ctx: &AppContext,
    database_url: &str,
    migrations_dir: Option<&PathBuf>,
) -> Result<()> {
    ctx.print_header("Running migrations");

    if !devkit_core::cmd_exists("sqlx") {
        return Err(anyhow!(
            "sqlx CLI not found. Install with: cargo install sqlx-cli"
        ));
    }

    let mut args = vec!["migrate".to_string(), "run".to_string()];

    if let Some(dir) = migrations_dir {
        let migrations_rel = dir.strip_prefix(&ctx.repo).unwrap_or(dir);
        args.push("--source".to_string());
        args.push(migrations_rel.to_string_lossy().to_string());
    }

    let code = CmdBuilder::new("sqlx")
        .args(&args)
        .env("DATABASE_URL", database_url)
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Migrations failed"));
    }

    ctx.print_success("Migrations complete");
    Ok(())
}

/// Reset the database (drop, create, migrate)
pub fn db_reset(
    ctx: &AppContext,
    database_url: &str,
    migrations_dir: Option<&PathBuf>,
) -> Result<()> {
    ctx.print_header("Resetting database");

    // Drop (ignore errors if database doesn't exist)
    let _ = db_drop(ctx, database_url, true);

    // Create
    db_create(ctx, database_url)?;

    // Migrate
    db_migrate(ctx, database_url, migrations_dir)?;

    ctx.print_success("Database reset complete");
    Ok(())
}
