use std::collections::HashMap;
use crate::command::Command;

pub struct CommandInfo {
    pub description: &'static str,
    pub cmd: Box<dyn Command>,
}

pub struct CommandRegistry {
    map: HashMap<&'static str, CommandInfo>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn register(&mut self, name: &'static str, description: &'static str, cmd: Box<dyn Command>) {
        self.map.insert(name, CommandInfo { description, cmd });
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.map.get(name).map(|info| &info.cmd)
    }

    pub fn get_all(&self) -> Vec<(&'static str, &'static str)> {
        let mut v: Vec<(&'static str, &'static str)> =
            self.map.iter().map(|(&k, info)| (k, info.description)).collect();
        v.sort_unstable_by(|a, b| a.0.cmp(b.0));
        v
    }
}
