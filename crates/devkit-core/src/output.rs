//! Output formatting utilities

use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Json,
    Table,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "plain" => Some(OutputFormat::Plain),
            "json" => Some(OutputFormat::Json),
            "table" => Some(OutputFormat::Table),
            _ => None,
        }
    }
}

/// Format output based on the specified format
pub fn format_output<T: Serialize + Display>(data: &T, format: OutputFormat) -> String {
    match format {
        OutputFormat::Plain => format!("{}", data),
        OutputFormat::Json => {
            serde_json::to_string_pretty(data).unwrap_or_else(|_| format!("{}", data))
        }
        OutputFormat::Table => {
            // Simple table formatting - can be enhanced later
            format!("{}", data)
        }
    }
}

/// Format a list of items
pub fn format_list<T: Serialize + Display>(items: &[T], format: OutputFormat) -> String {
    match format {
        OutputFormat::Plain => items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join("\n"),
        OutputFormat::Json => serde_json::to_string_pretty(items).unwrap_or_else(|_| {
            items
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>()
                .join("\n")
        }),
        OutputFormat::Table => {
            // TODO: Implement proper table formatting
            items
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}
