pub mod human;
pub mod human_stat;
pub mod human_controller;
pub mod human_repository;

pub use human::Human;
pub use human_stat::Stats;
pub use human_controller::HumanController;
pub use human_repository::{HumanRepository, InMemoryHumanRepository};