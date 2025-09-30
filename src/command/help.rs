use super::Command;
use colored::Colorize;

pub struct Help;

impl Command for Help {
    fn run(&self, registry: &super::CommandRegistry, _args: &[&str]) -> bool {
        let commands = registry.get_all();
        println!("{}", "Here you can find every command :".blue(),);
        println!("| {0: <10} | {1: <10}","Command".blue().bold(), "Description".blue().bold());
        for (name, desc) in commands {
            println!("| {0: <10} | {1: <10}", name, desc)
        }

        true
    }
}
