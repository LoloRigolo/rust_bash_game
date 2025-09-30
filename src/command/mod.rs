mod base;
pub use base::Command;

pub mod alias;
pub mod help;
pub mod exit;

pub mod registry;
pub use registry::{CommandRegistry, register_commands, dispatch_line};
