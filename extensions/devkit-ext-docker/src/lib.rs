//! Docker and Docker Compose extension for devkit

pub mod compose;
pub mod logs;
pub mod shell;

pub use compose::{
    compose_up, compose_down, compose_restart, compose_build,
    nuke_rebuild, list_services, list_running_containers,
};
pub use logs::follow_logs;
pub use shell::open_shell;

/// Check if Docker extension should be enabled
pub fn should_enable(ctx: &devkit_core::AppContext) -> bool {
    ctx.features.docker
}
