use super::Command;

pub struct Exit;

impl Command for Exit {
    fn run(&self, _args: &[&str]) -> bool {
        println!("Bye !");
        false
    }
}
