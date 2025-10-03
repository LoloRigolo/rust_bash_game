use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use super::Human;

pub trait HumanRepository {
    fn add(&self, human: Human);
    fn get(&self, id: Uuid) -> Option<Human>;
    fn get_all(&self) -> Vec<Human>;
    fn clear(&self);
}

pub struct InMemoryHumanRepository {
    store: Mutex<HashMap<Uuid, Human>>,
}

impl InMemoryHumanRepository {
    pub fn new() -> Self {
        Self { store: Mutex::new(HashMap::new()) }
    }
}

impl HumanRepository for InMemoryHumanRepository {
    fn add(&self, human: Human) {
        let mut guard = self.store.lock().expect("poisoned mutex");
        guard.insert(human.id, human);
    }

    fn get(&self, id: Uuid) -> Option<Human> {
        let guard = self.store.lock().expect("poisoned mutex");
        guard.get(&id).cloned()
    }

    fn get_all(&self) -> Vec<Human> {
        let guard = self.store.lock().expect("poisoned mutex");
        guard.values().cloned().collect()
    }

    fn clear(&self) {
        let mut guard = self.store.lock().expect("poisoned mutex");
        guard.clear();
    }
}
