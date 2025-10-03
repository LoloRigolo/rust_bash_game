use super::Human;
use super::human_repository::HumanRepository;
use uuid::Uuid;

pub struct HumanController;

impl HumanController {
    pub fn create(repo: &dyn HumanRepository) -> Human {
        let h = Human::new();
        repo.add(h.clone());
        h
    }

    pub fn create_new_born(repo: &dyn HumanRepository) -> Human {
        let h = Human::new_born();
        repo.add(h.clone());
        h
    }

    pub fn get(repo: &dyn HumanRepository, id: Uuid) -> Option<Human> {
        repo.get(id)
    }

    pub fn get_all(repo: &dyn HumanRepository) -> Vec<Human> {
        repo.get_all()
    }
}
