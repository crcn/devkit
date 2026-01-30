//! Kubernetes operations extension

use anyhow::{Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use std::process::Command;

pub struct K8sExtension;

impl Extension for K8sExtension {
    fn name(&self) -> &str {
        "k8s"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        cmd_exists("kubectl")
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "☸️  Show cluster status".to_string(),
                group: None,
                handler: Box::new(|ctx| cluster_status(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "📋 List pods".to_string(),
                group: None,
                handler: Box::new(|ctx| list_pods(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "📊 Get services".to_string(),
                group: None,
                handler: Box::new(|ctx| list_services(ctx).map_err(Into::into)),
            },
        ]
    }
}

fn cmd_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Show cluster status
pub fn cluster_status(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Kubernetes Cluster Status");
    println!();

    let output = Command::new("kubectl")
        .args(["cluster-info"])
        .output()
        .context("Failed to run kubectl")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        ctx.print_success("✓ Cluster is running");
    } else {
        ctx.print_error(&format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// List pods
pub fn list_pods(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Pods");
    println!();

    let output = Command::new("kubectl")
        .args(["get", "pods", "-o", "wide"])
        .output()
        .context("Failed to run kubectl")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// List services
pub fn list_services(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Services");
    println!();

    let output = Command::new("kubectl")
        .args(["get", "services"])
        .output()
        .context("Failed to run kubectl")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

/// Port forward to a pod
pub fn port_forward(ctx: &AppContext, pod: &str, local_port: u16, remote_port: u16) -> Result<()> {
    ctx.print_info(&format!(
        "Forwarding localhost:{} -> {}:{}",
        local_port, pod, remote_port
    ));

    let status = Command::new("kubectl")
        .args([
            "port-forward",
            pod,
            &format!("{}:{}", local_port, remote_port),
        ])
        .status()
        .context("Failed to run kubectl port-forward")?;

    if !status.success() {
        return Err(anyhow::anyhow!("Port forwarding failed"));
    }

    Ok(())
}

/// Get logs from a pod
pub fn logs(ctx: &AppContext, pod: &str, follow: bool) -> Result<()> {
    ctx.print_info(&format!("Fetching logs from {}", pod));

    let mut args = vec!["logs", pod];
    if follow {
        args.push("-f");
    }

    let status = Command::new("kubectl")
        .args(&args)
        .status()
        .context("Failed to get logs")?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to fetch logs"));
    }

    Ok(())
}

/// Scale a deployment
pub fn scale(ctx: &AppContext, deployment: &str, replicas: u32) -> Result<()> {
    ctx.print_info(&format!("Scaling {} to {} replicas", deployment, replicas));

    let output = Command::new("kubectl")
        .args([
            "scale",
            "deployment",
            deployment,
            "--replicas",
            &replicas.to_string(),
        ])
        .output()
        .context("Failed to scale deployment")?;

    if output.status.success() {
        ctx.print_success(&format!("✓ Scaled {} to {} replicas", deployment, replicas));
    } else {
        ctx.print_error(&format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
