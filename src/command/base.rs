pub trait Command {
    fn run(&self, registry: &super::CommandRegistry, args: &[&str]) -> bool;
}
