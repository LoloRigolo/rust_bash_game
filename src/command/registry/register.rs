use super::registry::CommandRegistry;

use crate::command::{help, exit};

pub fn register_commands(registry: &mut CommandRegistry) {
    registry.register("help","Show every commands", Box::new(help::Help));
    registry.register("exit","Quit the game", Box::new(exit::Exit));
}
