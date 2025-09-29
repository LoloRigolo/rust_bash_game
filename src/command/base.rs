pub trait Command {
    fn run(&self, args: &[&str]) -> bool;
}
