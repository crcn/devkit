//! Task discovery and execution engine for devkit

pub mod cmd_builder;
pub mod runner;
pub mod template;
pub mod watch;

pub use cmd_builder::CmdBuilder;
pub use runner::{list_commands, print_results, run_cmd, CmdOptions, CmdResult};
pub use template::{extract_vars, resolve_template};
pub use watch::{watch_and_run, WatchConfig};
