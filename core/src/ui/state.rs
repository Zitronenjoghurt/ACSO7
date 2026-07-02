use crate::ui::log::EventLog;
use crate::ui::popup::PopupState;
use crate::ui::{ScreenId, ShipFocus};
use crate::world::WorldMeta;
use crate::world::ship::resources::history::ResourceHistory;

#[derive(Debug)]
pub struct UiState {
    pub popup: Option<PopupState>,
    pub current_screen: ScreenId,
    pub ship_focus: ShipFocus,
    pub menu_selected: usize,
    pub colonist_selected: usize,
    pub system_selected: usize,
    pub resource_selected: usize,
    pub debug_selected: usize,
    pub debug_grant_step: f64,
    pub history_tier: usize,
    pub new_world_name: String,
    pub saved_worlds: Vec<WorldMeta>,
    pub log: EventLog,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            popup: None,
            current_screen: ScreenId::default(),
            ship_focus: ShipFocus::default(),
            menu_selected: 0,
            colonist_selected: 0,
            system_selected: 0,
            resource_selected: 0,
            debug_selected: 0,
            debug_grant_step: 1000.0,
            history_tier: ResourceHistory::DEFAULT_TIER,
            new_world_name: String::new(),
            saved_worlds: Vec::new(),
            log: EventLog::default(),
        }
    }
}
