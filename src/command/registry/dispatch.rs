use super::registry::CommandRegistry;
use crate::command::alias;

pub fn dispatch_line(registry: &CommandRegistry, line: &str) -> bool {
    let mut parts = line.split_whitespace();
    let raw_name = match parts.next() {
        Some(n) => n,
        None => return true,
    };
    let args: Vec<&str> = parts.collect();

    let canonical = alias::resolve(raw_name);

    match registry.get(canonical.as_str()) {
        Some(cmd) => cmd.run(registry, &args),
        None => {
            println!("Commande inconnue: '{}'. Tape 'help' pour l'aide.", raw_name);
            true
        }
    }
}
