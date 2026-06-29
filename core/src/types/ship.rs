#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub power: f64,
}

impl Default for Ship {
    fn default() -> Self {
        Self { power: 100.0 }
    }
}
