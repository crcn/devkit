//! Database seeding

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;
use std::path::Path;

/// Run database seeds
pub fn db_seed(ctx: &AppContext, database_url: &str, seed_path: Option<&Path>) -> Result<()> {
    ctx.print_header("Seeding database");

    // Check for seed script first (scripts/seed.sh)
    let seed_script = ctx.repo.join("scripts/seed.sh");
    if seed_script.exists() {
        let code = CmdBuilder::new("bash")
            .arg(seed_script.to_string_lossy().to_string())
            .env("DATABASE_URL", database_url)
            .cwd(&ctx.repo)
            .run()?;

        if code != 0 {
            return Err(anyhow!("Seed script failed"));
        }
    } else if let Some(path) = seed_path {
        // Check for SQL seed file
        if !path.exists() {
            return Err(anyhow!("Seed file not found: {}", path.display()));
        }

        if !devkit_core::cmd_exists("psql") {
            return Err(anyhow!(
                "psql not found. Install PostgreSQL client tools."
            ));
        }

        let code = CmdBuilder::new("psql")
            .args(["-f", &path.to_string_lossy()])
            .env("DATABASE_URL", database_url)
            .cwd(&ctx.repo)
            .run()?;

        if code != 0 {
            return Err(anyhow!("Seed SQL failed"));
        }
    } else {
        return Err(anyhow!(
            "No seed script found. Create scripts/seed.sh or provide seed_path"
        ));
    }

    ctx.print_success("Database seeded");
    Ok(())
}
