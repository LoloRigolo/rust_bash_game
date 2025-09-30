use super::Command;

pub struct Exit;

impl Command for Exit {
    fn run(&self, _registry: &super::CommandRegistry, _args: &[&str]) -> bool {
        println!("Bye !");
        false
    }
}
