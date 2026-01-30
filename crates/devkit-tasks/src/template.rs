//! Command template resolution with variable substitution

use anyhow::{Context, Result};
use std::collections::HashMap;

/// Resolve variables in a command template
///
/// Supports {var} syntax for variable substitution
/// Variables can come from:
/// - Environment variables
/// - Provided defaults
/// - User prompts (if interactive)
pub fn resolve_template(
    template: &str,
    vars: &HashMap<String, String>,
    env_vars: &HashMap<String, String>,
) -> Result<String> {
    let mut result = template.to_string();
    let mut missing_vars = Vec::new();

    // Find all {var} patterns
    let re = regex::Regex::new(r"\{([^}]+)\}").unwrap();

    for cap in re.captures_iter(template) {
        let var_name = &cap[1];
        let placeholder = &cap[0];

        // Try to resolve variable in order of precedence:
        // 1. Explicit vars (from config)
        // 2. Environment variables
        let value = vars
            .get(var_name)
            .or_else(|| env_vars.get(var_name))
            .cloned();

        match value {
            Some(val) => {
                result = result.replace(placeholder, &val);
            }
            None => {
                missing_vars.push(var_name.to_string());
            }
        }
    }

    if !missing_vars.is_empty() {
        return Err(anyhow::anyhow!(
            "Missing template variables: {}. Set them in your config or environment.",
            missing_vars.join(", ")
        ));
    }

    Ok(result)
}

/// Extract variable names from a template
pub fn extract_vars(template: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{([^}]+)\}").unwrap();
    re.captures_iter(template)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Prompt user for missing variables (interactive mode)
#[cfg(feature = "interactive")]
pub fn prompt_for_vars(vars: &[String]) -> Result<HashMap<String, String>> {
    use dialoguer::Input;

    let mut result = HashMap::new();

    for var in vars {
        let value: String = Input::new()
            .with_prompt(format!("Enter value for '{}'", var))
            .interact_text()
            .context(format!("Failed to get input for variable '{}'", var))?;

        result.insert(var.clone(), value);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_template_with_vars() {
        let mut vars = HashMap::new();
        vars.insert("env".to_string(), "prod".to_string());
        vars.insert("port".to_string(), "8080".to_string());

        let template = "kubectl apply -f k8s/{env}.yaml --port {port}";
        let result = resolve_template(template, &vars, &HashMap::new()).unwrap();

        assert_eq!(result, "kubectl apply -f k8s/prod.yaml --port 8080");
    }

    #[test]
    fn test_resolve_template_with_env_vars() {
        let vars = HashMap::new();
        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "alice".to_string());

        let template = "echo Hello {USER}";
        let result = resolve_template(template, &vars, &env_vars).unwrap();

        assert_eq!(result, "echo Hello alice");
    }

    #[test]
    fn test_resolve_template_missing_var() {
        let template = "echo {missing}";
        let result = resolve_template(template, &HashMap::new(), &HashMap::new());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing"));
    }

    #[test]
    fn test_extract_vars() {
        let template = "deploy {app} to {env} on port {port}";
        let vars = extract_vars(template);

        assert_eq!(vars, vec!["app", "env", "port"]);
    }
}
