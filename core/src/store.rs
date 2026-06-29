use crate::config::Config;
use crate::types::ship::Ship;
use crate::ui::ScreenId;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub config: Config,
    pub current_screen: ScreenId,
    pub ship: Ship,
}
