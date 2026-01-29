//! Tunnel extension for devkit
//!
//! Provides HTTP tunneling via ngrok or cloudflared.

use anyhow::{anyhow, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

/// Start an HTTP tunnel to localhost
pub fn start_tunnel(ctx: &AppContext, port: u16, subdomain: Option<&str>) -> Result<()> {
    // Try ngrok first, then cloudflared
    if devkit_core::cmd_exists("ngrok") {
        start_ngrok_tunnel(ctx, port, subdomain)
    } else if devkit_core::cmd_exists("cloudflared") {
        start_cloudflared_tunnel(ctx, port)
    } else {
        Err(anyhow!(
            "No tunnel tool found. Install ngrok or cloudflared:\n\
             - ngrok: brew install ngrok\n\
             - cloudflared: brew install cloudflared"
        ))
    }
}

fn start_ngrok_tunnel(ctx: &AppContext, port: u16, subdomain: Option<&str>) -> Result<()> {
    ctx.print_header(&format!("Starting ngrok tunnel to port {}", port));

    let mut args = vec!["http".to_string(), port.to_string()];

    if let Some(sub) = subdomain {
        args.push("--subdomain".to_string());
        args.push(sub.to_string());
    }

    let code = CmdBuilder::new("ngrok")
        .args(&args)
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 && code != 130 {
        return Err(anyhow!("ngrok exited with code {}", code));
    }

    Ok(())
}

fn start_cloudflared_tunnel(ctx: &AppContext, port: u16) -> Result<()> {
    ctx.print_header(&format!(
        "Starting cloudflared tunnel to port {}",
        port
    ));

    let code = CmdBuilder::new("cloudflared")
        .args(["tunnel", "--url", &format!("http://localhost:{}", port)])
        .cwd(&ctx.repo)
        .inherit_io()
        .run()?;

    if code != 0 && code != 130 {
        return Err(anyhow!("cloudflared exited with code {}", code));
    }

    Ok(())
}

/// Check if this extension should be enabled
pub fn should_enable(_ctx: &devkit_core::AppContext) -> bool {
    // Enable if ngrok or cloudflared is available
    devkit_core::cmd_exists("ngrok") || devkit_core::cmd_exists("cloudflared")
}
