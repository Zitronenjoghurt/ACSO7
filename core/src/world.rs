pub mod colonist;
pub mod ship;

use jiff::{Timestamp, Zoned};

pub type WorldId = String;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorldMeta {
    pub id: WorldId,
    pub name: String,
    pub created_at: Timestamp,
    pub last_played: Timestamp,
}

impl WorldMeta {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Timestamp::now();
        Self {
            id: Zoned::now().strftime("%Y%m%d%H%M%S%4f").to_string(),
            name: name.into(),
            created_at: now,
            last_played: now,
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct World {
    pub meta: WorldMeta,
    pub ship: ship::Ship,
    #[serde(default)]
    pub mission_secs: f64,
}

impl World {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            meta: WorldMeta::new(name),
            ship: ship::Ship::default(),
            mission_secs: 0.0,
        }
    }
}

// Tick
impl World {
    pub fn tick(&mut self, dt: f64) {
        self.mission_secs += dt;
        self.ship.tick(dt);
    }
}
