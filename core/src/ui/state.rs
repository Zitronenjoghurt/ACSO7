use crate::ui::ScreenId;
use crate::world::WorldMeta;

#[derive(Debug, Default)]
pub struct UiState {
    pub current_screen: ScreenId,
    pub menu_selected: usize,
    pub new_world_name: String,
    pub saved_worlds: Vec<WorldMeta>,
}
