#[cfg(test)]
mod tests {
    use crate::object::human::{HumanController, InMemoryHumanRepository};

    #[test]
    fn repo_add_and_get_all() {
        let repo = InMemoryHumanRepository::new();
        let _ = HumanController::create(&repo);
        let _ = HumanController::create_new_born(&repo);

        let all = HumanController::get_all(&repo);
        assert!(all.len() >= 2);
    }

    #[test]
    fn repo_get_by_id() {
        let repo = InMemoryHumanRepository::new();
        let h = HumanController::create(&repo);

        let fetched = HumanController::get(&repo, h.id).expect("should exist");
        assert_eq!(fetched.id, h.id);
        assert_eq!(fetched.age, h.age);
    }
}
