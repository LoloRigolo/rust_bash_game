mod base;
pub use base::Command;

pub mod help;
pub mod exit;
pub mod alias;

use std::collections::HashMap;

pub struct CommandRegistry {
    map: HashMap<&'static str, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn register(&mut self, name: &'static str, cmd: Box<dyn Command>) {
        self.map.insert(name, cmd);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.map.get(name)
    }
}

pub fn register_commands(registry: &mut CommandRegistry) {
    registry.register("help", Box::new(help::Help));
    registry.register("exit", Box::new(exit::Exit));
}

pub fn dispatch_line(registry: &CommandRegistry, line: &str) -> bool {
    let mut parts = line.split_whitespace();
    let raw_name = match parts.next() {
        Some(n) => n,
        None => return true,
    };
    let args: Vec<&str> = parts.collect();

    let canonical = alias::resolve(raw_name);

    match registry.get(canonical.as_str()) {
        Some(cmd) => cmd.run(&args),
        None => {
            println!("Commande inconnue: '{}'. Tape 'help' pour l'aide.", raw_name);
            true
        }
    }
}