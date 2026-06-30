use crate::ui::theme::Theme;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub autosave_interval_secs: u64,
    pub max_auto_saves: usize,
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            autosave_interval_secs: 60,
            max_auto_saves: 10,
            theme: Theme::default(),
        }
    }
}
