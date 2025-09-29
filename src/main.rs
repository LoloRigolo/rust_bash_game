mod command;

use std::io::{self, Write};
use command::{CommandRegistry, register_commands, dispatch_line};

fn main() {
    let mut registry = CommandRegistry::new();
    register_commands(&mut registry);

    println!("GameShell v0.1 — tape 'help' ou 'exit'.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Erreur de lecture. Réessaie.");
            continue;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        if !dispatch_line(&registry, line) {
            break;
        }
    }
}
