#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Starscooper {
    pub health: f64,
    pub scoop_rate: f64,
}

impl Default for Starscooper {
    fn default() -> Self {
        Self {
            health: 1.0,
            scoop_rate: 100.0,
        }
    }
}
