use crate::world::colonist::Colonist;

pub struct Pods {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pod {
    pub colonist: Colonist,
}

impl Pod {
    pub fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            colonist: Colonist::generate(rng),
        }
    }
}
