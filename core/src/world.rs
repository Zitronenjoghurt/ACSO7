pub mod colonist;
mod events;
pub mod ship;

use crate::world::events::WorldEvents;
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
    pub fn tick(&mut self, dt: f64) -> WorldEvents {
        let mut events = WorldEvents::default();
        self.mission_secs += dt;
        self.ship.tick(dt, &mut events);
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starved_pods_raise_alerts_and_emit_events() {
        let mut world = World::new("test");
        world.ship.pods.pods.truncate(1);
        world.ship.pods.pods[0].health = 0.05;

        let first = world.tick(1.0);
        assert!(world.ship.pods.life_support_failing());
        assert!(!world.ship.pods.alerts().is_empty());
        drop(first);

        let mut messages: Vec<String> = Vec::new();
        for _ in 0..1000 {
            messages.extend(world.tick(1.0).into_messages());
            if !messages.is_empty() {
                break;
            }
        }
        assert!(messages.iter().any(|m| m.contains("POD LOST")));
    }
}
