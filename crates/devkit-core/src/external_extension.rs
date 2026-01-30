use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context as _, Result};

/// Extension definition from TOML file
#[derive(Debug, Deserialize)]
pub struct ExtensionConfig {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub action: Vec<ActionConfig>,
}

/// Action definition from TOML
#[derive(Debug, Deserialize, Clone)]
pub struct ActionConfig {
    pub id: String,
    pub label: String,
    pub group: Option<String>,
    pub description: Option<String>,
    pub command: String,  // Path to executable (relative to extension directory)
    pub args: Option<Vec<String>>,  // Optional arguments
    pub env: Option<HashMap<String, String>>,  // Optional env vars
}

/// Wrapper that implements Extension trait for TOML-defined extensions
pub struct ExternalExtension {
    config: ExtensionConfig,
    extension_dir: PathBuf,  // Base directory for this extension
}

impl ExternalExtension {
    /// Load extension from TOML file in an extension directory
    pub fn load(extension_dir: &Path) -> Result<Self> {
        // Look for config.toml in the directory
        let toml_path = extension_dir.join("config.toml");

        if !toml_path.exists() {
            anyhow::bail!("No config.toml found in {}", extension_dir.display());
        }

        let toml_content = std::fs::read_to_string(&toml_path)
            .context(format!("Failed to read {}", toml_path.display()))?;

        let config: ExtensionConfig = toml::from_str(&toml_content)
            .context(format!("Failed to parse TOML from {}", toml_path.display()))?;

        Ok(Self {
            config,
            extension_dir: extension_dir.to_path_buf(),
        })
    }

    fn execute_action(&self, ctx: &crate::AppContext, action: &ActionConfig) -> Result<()> {
        // Resolve command path (relative to extension directory)
        let command_path = self.extension_dir.join(&action.command);

        if !command_path.exists() {
            anyhow::bail!(
                "Command not found: {} (resolved to {})",
                action.command,
                command_path.display()
            );
        }

        let mut cmd = Command::new(&command_path);

        // Add any configured args
        if let Some(args) = &action.args {
            cmd.args(args);
        }

        // Set context via environment variables
        cmd.env("DEVKIT_REPO_ROOT", &ctx.repo);
        cmd.env("DEVKIT_QUIET", if ctx.quiet { "1" } else { "0" });
        cmd.env("DEVKIT_FEATURE_DOCKER", if ctx.features.docker { "1" } else { "0" });
        cmd.env("DEVKIT_FEATURE_GIT", if ctx.features.git { "1" } else { "0" });
        cmd.env("DEVKIT_FEATURE_CARGO", if ctx.features.cargo { "1" } else { "0" });
        cmd.env("DEVKIT_FEATURE_NODE", if ctx.features.node { "1" } else { "0" });
        cmd.env("DEVKIT_FEATURE_DATABASE", if ctx.features.database { "1" } else { "0" });

        // Add any custom env vars from config
        if let Some(env) = &action.env {
            for (key, value) in env {
                cmd.env(key, value);
            }
        }

        // Inherit current directory
        cmd.current_dir(&ctx.repo);

        // Execute and wait
        let status = cmd.status()
            .context(format!("Failed to execute {}", command_path.display()))?;

        if !status.success() {
            anyhow::bail!(
                "Extension action '{}' failed with exit code {}",
                action.id,
                status.code().unwrap_or(-1)
            );
        }

        Ok(())
    }
}

impl crate::Extension for ExternalExtension {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn is_available(&self, _ctx: &crate::AppContext) -> bool {
        // Always available if we successfully loaded the TOML
        true
    }

    fn menu_items(&self, _ctx: &crate::AppContext) -> Vec<crate::MenuItem> {
        self.config
            .action
            .iter()
            .map(|action| {
                let action_clone = action.clone();
                let ext_dir = self.extension_dir.clone();
                let config = ExtensionConfig {
                    name: self.config.name.clone(),
                    version: self.config.version.clone(),
                    description: self.config.description.clone(),
                    action: vec![action_clone.clone()],
                };

                crate::MenuItem {
                    label: action.label.clone(),
                    group: action.group.clone(),
                    handler: Box::new(move |ctx| {
                        let ext = ExternalExtension {
                            config: config.clone(),
                            extension_dir: ext_dir.clone(),
                        };
                        ext.execute_action(ctx, &action_clone).map_err(|e| e.into())
                    }),
                }
            })
            .collect()
    }
}

// Need Clone for ExtensionConfig
impl Clone for ExtensionConfig {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            action: self.action.clone(),
        }
    }
}
