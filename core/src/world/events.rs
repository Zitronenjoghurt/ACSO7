use crate::world::colonist::Colonist;

#[derive(Debug)]
pub enum WorldEvent {
    PodDestroyed(Colonist),
}

impl WorldEvent {
    pub fn message(&self) -> String {
        match self {
            WorldEvent::PodDestroyed(c) => {
                format!("POD LOST — {} ({}) has perished", c.full_name(), c.age)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct WorldEvents {
    events: Vec<WorldEvent>,
}

impl WorldEvents {
    pub fn push(&mut self, event: WorldEvent) {
        self.events.push(event);
    }

    pub fn pod_destroyed(&mut self, colonist: Colonist) {
        self.push(WorldEvent::PodDestroyed(colonist))
    }

    pub fn into_messages(self) -> impl Iterator<Item = String> {
        self.events.into_iter().map(|e| e.message())
    }
}
