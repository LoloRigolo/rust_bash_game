pub mod registry;
pub mod register;
pub mod dispatch;

pub use registry::CommandRegistry;
pub use register::register_commands;
pub use dispatch::dispatch_line;
