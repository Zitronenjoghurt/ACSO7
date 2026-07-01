use crate::ui::log::EventLog;
use crate::ui::{ScreenId, ShipFocus};
use crate::world::WorldMeta;

#[derive(Debug, Default)]
pub struct UiState {
    pub current_screen: ScreenId,
    pub ship_focus: ShipFocus,
    pub menu_selected: usize,
    pub colonist_selected: usize,
    pub system_selected: usize,
    pub resource_selected: usize,
    pub history_tier: usize,
    pub new_world_name: String,
    pub saved_worlds: Vec<WorldMeta>,
    pub log: EventLog,
}
