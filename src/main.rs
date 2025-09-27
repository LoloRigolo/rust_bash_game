use std::io::{self, Write};

fn main() {
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

        match line {
            "help" => {
                println!("Manuel of command ?");
            }
            "exit" | "quit" => {
                println!("Au revoir !");
                break;
            }
            other => {
                println!("Commande inconnue: '{}'. Tape 'help' pour l'aide.", other);
            }
        }
    }
}
