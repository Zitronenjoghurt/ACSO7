use crate::config::Config;
use crate::types::ship::Ship;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub config: Config,
    pub ship: Ship,
}
