use uuid::Uuid;
use super::human_stat::Stats;

pub struct Human {
    pub id: Uuid,
    pub age: u16,
    pub stats: Stats,
}

impl Human {
    pub fn new() -> Self {
        Self::new_with_age(21)
    }

    pub fn new_with_age(age: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            age,
            stats: Stats::random(),
        }
    }
}