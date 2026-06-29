#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub max_auto_saves: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { max_auto_saves: 10 }
    }
}
