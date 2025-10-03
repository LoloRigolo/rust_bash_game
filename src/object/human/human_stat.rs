use rand::Rng;

#[derive(Debug, Clone)]
pub struct Stats {
    pub dex: u8,
    pub wisdom: u8,
    pub strength: u8,
    pub constit: u8,
    pub intelligence: u8,
    pub charism: u8,
}

impl Stats {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            dex: rng.gen_range(1..=100),
            wisdom: rng.gen_range(1..=100),
            strength: rng.gen_range(1..=100),
            constit: rng.gen_range(1..=100),
            intelligence: rng.gen_range(1..=100),
            charism: rng.gen_range(1..=100),
        }
    }

    pub fn as_array(&self) -> [u8; 6] {
        [self.dex, self.wisdom, self.strength, self.constit, self.intelligence, self.charism]
    }
}