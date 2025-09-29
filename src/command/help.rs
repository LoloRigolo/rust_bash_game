use super::Command;

pub struct Help;

impl Command for Help {
    fn run(&self, _args: &[&str]) -> bool {
        println!("Manuel of command ?");
        true
    }
}
