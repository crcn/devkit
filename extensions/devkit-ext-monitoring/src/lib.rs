//! Monitoring stack extension
//!
//! Provides local Prometheus, Grafana, Loki, and Tempo stack

use anyhow::{Context, Result};
use devkit_core::{AppContext, Extension, MenuItem};
use std::process::Command;

pub struct MonitoringExtension;

impl Extension for MonitoringExtension {
    fn name(&self) -> &str {
        "monitoring"
    }

    fn is_available(&self, ctx: &AppContext) -> bool {
        ctx.features.docker
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "📊 Start monitoring stack".to_string(),
                group: None,
                handler: Box::new(|ctx| start_monitoring(ctx).map_err(Into::into)),
            },
            MenuItem {
                label: "⏹  Stop monitoring stack".to_string(),
                group: None,
                handler: Box::new(|ctx| stop_monitoring(ctx).map_err(Into::into)),
            },
        ]
    }
}

/// Start monitoring stack
pub fn start_monitoring(ctx: &AppContext) -> Result<()> {
    ctx.print_header("Starting Monitoring Stack");
    println!();

    ctx.print_info("Monitoring stack includes:");
    println!("  • Prometheus (metrics)    - http://localhost:9090");
    println!("  • Grafana (dashboards)    - http://localhost:3000");
    println!("  • Loki (logs)            - http://localhost:3100");
    println!("  • Tempo (traces)         - http://localhost:3200");
    println!();

    // Check if docker-compose file exists
    let compose_file = ctx.repo.join("docker-compose.monitoring.yml");

    if !compose_file.exists() {
        ctx.print_warning("docker-compose.monitoring.yml not found");
        ctx.print_info("Creating default monitoring stack configuration...");
        create_monitoring_compose(&ctx.repo)?;
    }

    ctx.print_info("Starting containers...");

    let output = Command::new("docker-compose")
        .args(["-f", "docker-compose.monitoring.yml", "up", "-d"])
        .current_dir(&ctx.repo)
        .output()
        .context("Failed to start monitoring stack")?;

    if output.status.success() {
        ctx.print_success("✓ Monitoring stack started");
        println!();
        println!("Access dashboards at:");
        println!("  Grafana:    http://localhost:3000 (admin/admin)");
        println!("  Prometheus: http://localhost:9090");
    } else {
        ctx.print_error(&format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Stop monitoring stack
pub fn stop_monitoring(ctx: &AppContext) -> Result<()> {
    ctx.print_info("Stopping monitoring stack...");

    let output = Command::new("docker-compose")
        .args(["-f", "docker-compose.monitoring.yml", "down"])
        .current_dir(&ctx.repo)
        .output()
        .context("Failed to stop monitoring stack")?;

    if output.status.success() {
        ctx.print_success("✓ Monitoring stack stopped");
    } else {
        ctx.print_error(&format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

fn create_monitoring_compose(repo: &std::path::Path) -> Result<()> {
    let compose_content = r#"version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-data:/var/lib/grafana

  loki:
    image: grafana/loki:latest
    ports:
      - "3100:3100"
    command: -config.file=/etc/loki/local-config.yaml

  tempo:
    image: grafana/tempo:latest
    ports:
      - "3200:3200"
    command: [ "-config.file=/etc/tempo.yaml" ]

volumes:
  grafana-data:
"#;

    let compose_file = repo.join("docker-compose.monitoring.yml");
    std::fs::write(compose_file, compose_content)?;

    // Create basic prometheus config
    let prometheus_config = r#"global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
"#;

    let prometheus_file = repo.join("prometheus.yml");
    std::fs::write(prometheus_file, prometheus_config)?;

    Ok(())
}
