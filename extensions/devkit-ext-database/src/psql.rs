//! PostgreSQL shell access

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Open psql shell to the database
pub fn db_psql(ctx: &AppContext, database_url: &str) -> Result<()> {
    if !devkit_core::cmd_exists("psql") {
        return Err(anyhow!(
            "psql not found. Install PostgreSQL client tools."
        ));
    }

    ctx.print_header("Connecting to database");

    let code = CmdBuilder::new("psql")
        .arg(database_url)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    // 130 = SIGINT (Ctrl+D), which is expected
    if code != 0 && code != 130 {
        return Err(anyhow!("psql exited with code {}", code));
    }

    Ok(())
}

/// Execute a SQL query
pub fn db_query(ctx: &AppContext, database_url: &str, query: &str) -> Result<()> {
    if !devkit_core::cmd_exists("psql") {
        return Err(anyhow!(
            "psql not found. Install PostgreSQL client tools."
        ));
    }

    let code = CmdBuilder::new("psql")
        .args(["-c", query])
        .arg(database_url)
        .cwd(&ctx.repo)
        .run()?;

    if code != 0 {
        return Err(anyhow!("Query failed with code {}", code));
    }

    Ok(())
}
