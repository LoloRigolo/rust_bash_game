#[cfg(test)]

mod tests {

    use crate::object::human::{Human, HumanController};

    #[test]
    fn object_human() {
        let h1 = Human::new();
        let g  = HumanController::get(&h1);
        assert!(g.contains("Human: Age: 21, Stats: ["),"Show human")
    }
}